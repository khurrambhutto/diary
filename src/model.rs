use chrono::{Days, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn today() -> NaiveDate {
  Local::now().date_naive()
}
pub fn key(date: NaiveDate) -> String {
  date.format("%Y-%m-%d").to_string()
}
pub fn words(text: &str) -> usize {
  text.split_whitespace().count()
}
pub fn id() -> String {
  static NEXT: AtomicU64 = AtomicU64::new(0);
  format!(
    "r-{}-{}",
    Local::now().timestamp_micros(),
    NEXT.fetch_add(1, Ordering::Relaxed)
  )
}
pub fn timestamp() -> String {
  Local::now().to_rfc3339()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
  pub id: String,
  pub label: String,
  pub created_at: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
  pub id: String,
  pub text: String,
  pub created_at: String,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Habits {
  pub habits: Vec<Habit>,
  #[serde(default)]
  pub checks: BTreeMap<String, BTreeMap<String, bool>>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DiaryData {
  #[serde(default)]
  pub journal: BTreeMap<String, String>,
  #[serde(default)]
  pub habits: Habits,
  #[serde(default, deserialize_with = "deserialize_todos")]
  pub todos: Vec<Todo>,
  // Retain keys owned by older versions when saving the shared file.
  #[serde(flatten)]
  pub extra: BTreeMap<String, serde_json::Value>,
}
fn deserialize_todos<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Todo>, D::Error> {
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum Shape {
    Backlog(Vec<Todo>),
    Daily(BTreeMap<String, Vec<Todo>>),
  }
  Ok(match Shape::deserialize(d)? {
    Shape::Backlog(items) => items,
    Shape::Daily(days) => {
      let mut unique = BTreeMap::new();
      for item in days.into_values().flatten() {
        unique.entry(item.id.clone()).or_insert(item);
      }
      let mut items: Vec<_> = unique.into_values().collect();
      items.sort_by(|a, b| a.created_at.cmp(&b.created_at));
      items
    }
  })
}
impl DiaryData {
  pub fn note(&self, day: NaiveDate) -> &str {
    self
      .journal
      .get(&key(day))
      .map(String::as_str)
      .unwrap_or("")
  }
  pub fn set_note(&mut self, day: NaiveDate, text: String) {
    if text.is_empty() {
      self.journal.remove(&key(day));
    } else {
      self.journal.insert(key(day), text);
    }
  }
  pub fn checked(&self, id: &str, day: NaiveDate) -> bool {
    self
      .habits
      .checks
      .get(id)
      .and_then(|c| c.get(&key(day)))
      .copied()
      .unwrap_or(false)
  }
  pub fn toggle_habit(&mut self, id: &str, day: NaiveDate) {
    let on = !self.checked(id, day);
    self
      .habits
      .checks
      .entry(id.into())
      .or_default()
      .insert(key(day), on);
  }
  pub fn add_habit(&mut self, label: &str) {
    if !label.trim().is_empty() {
      self.habits.habits.push(Habit {
        id: id(),
        label: label.trim().into(),
        created_at: timestamp(),
      });
    }
  }
  pub fn add_todo(&mut self, text: &str) {
    if !text.trim().is_empty() {
      self.todos.push(Todo {
        id: id(),
        text: text.trim().into(),
        created_at: timestamp(),
      });
    }
  }
  pub fn delete_habit(&mut self, id: &str) {
    self.habits.habits.retain(|h| h.id != id);
    self.habits.checks.remove(id);
  }
  pub fn entry_dates(&self, until: NaiveDate) -> Vec<NaiveDate> {
    self
      .journal
      .iter()
      .filter(|(_, text)| words(text) > 0)
      .filter_map(|(date, _)| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok())
      .filter(|date| *date <= until)
      .collect()
  }
  pub fn habit_dates(&self, id: &str, until: NaiveDate) -> Vec<NaiveDate> {
    self
      .habits
      .checks
      .get(id)
      .into_iter()
      .flat_map(|c| c.iter())
      .filter(|(_, on)| **on)
      .filter_map(|(d, _)| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
      .filter(|d| *d <= until)
      .collect()
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct Streaks {
  pub current: usize,
  pub best: usize,
}
pub fn streaks(dates: &[NaiveDate], until: NaiveDate) -> Streaks {
  let mut sorted: Vec<_> = dates.iter().copied().filter(|d| *d <= until).collect();
  sorted.sort_unstable();
  sorted.dedup();
  let mut result = Streaks::default();
  let mut run = 0;
  let mut previous = None;
  for &date in &sorted {
    run = if previous == date.checked_sub_days(Days::new(1)) {
      run + 1
    } else {
      1
    };
    result.best = result.best.max(run);
    previous = Some(date);
  }
  if previous == Some(until) || previous == until.checked_sub_days(Days::new(1)) {
    result.current = run;
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  fn d(n: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 10, n).unwrap()
  }
  #[test]
  fn streak_grace_and_gaps() {
    let dates = [d(1), d(2), d(3), d(7), d(8)];
    assert_eq!(
      streaks(&dates, d(9)),
      Streaks {
        current: 2,
        best: 3
      }
    );
    assert_eq!(streaks(&dates, d(10)).current, 0);
    assert_eq!(streaks(&[d(1), d(1), d(2), d(30)], d(2)).current, 2);
  }
  #[test]
  fn old_store_roundtrip() {
    let raw = r#"{"journal":{"2026-10-01":"hello"},"habits":{"habits":[{"id":"read","label":"Read","createdAt":"2026-10-01"}],"checks":{"read":{"2026-10-01":true}}},"todos":[{"id":"t","text":"Task","createdAt":"2026-10-01"}],"other":42}"#;
    let data: DiaryData = serde_json::from_str(raw).unwrap();
    assert!(data.checked("read", d(1)));
    assert_eq!(data.note(d(1)), "hello");
    let saved = serde_json::to_value(data).unwrap();
    assert_eq!(saved["other"], 42);
    assert_eq!(saved["todos"][0]["createdAt"], "2026-10-01");
  }
  #[test]
  fn legacy_todos_deduplicate() {
    let raw = r#"{"todos":{"2026-10-01":[{"id":"a","text":"A","createdAt":"1"}],"2026-10-02":[{"id":"a","text":"A","createdAt":"1"},{"id":"b","text":"B","createdAt":"2"}]}}"#;
    assert_eq!(
      serde_json::from_str::<DiaryData>(raw).unwrap().todos.len(),
      2
    );
  }
  #[test]
  fn empty_notes_and_future_dates_do_not_count() {
    let mut data = DiaryData::default();
    data.set_note(d(1), " \n ".into());
    data.set_note(d(2), "one\n two".into());
    data.set_note(d(30), "future".into());
    assert_eq!(data.entry_dates(d(3)), vec![d(2)]);
    assert_eq!(words(data.note(d(2))), 2);
    data.set_note(d(2), String::new());
    assert!(!data.journal.contains_key("2026-10-02"));
  }
  #[test]
  fn habit_delete_removes_checks_and_empty_labels_are_rejected() {
    let mut data = DiaryData::default();
    data.add_habit("   ");
    data.add_todo("\n");
    assert!(data.habits.habits.is_empty() && data.todos.is_empty());
    data.add_habit(" Read ");
    let id = data.habits.habits[0].id.clone();
    data.toggle_habit(&id, d(1));
    assert!(data.checked(&id, d(1)));
    data.delete_habit(&id);
    assert!(data.habits.checks.is_empty());
  }
}
