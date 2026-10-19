use crate::{
  inputs,
  model::{self, DiaryData},
  storage::Storage,
  theme::Palette,
};
use chrono::{Datelike, NaiveDate};
use gpui_kit::{
  base::input::{InputState, TextareaState},
  *,
};
use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq)]
pub enum View {
  Day,
  Analytics,
  Settings,
}
pub struct Diary {
  pub data: DiaryData,
  pub storage: Option<Storage>,
  pub load_error: Option<String>,
  pub save_error: Option<String>,
  pub palette: Palette,
  pub date: NaiveDate,
  pub current_day: NaiveDate,
  pub view: View,
  pub calendar: Option<NaiveDate>,
  pub menu: Option<String>,
  pub menu_position: Point<Pixels>,
  pub renaming: Option<String>,
  pub completed: BTreeSet<String>,
  pub completion_tasks: std::collections::BTreeMap<String, Task<()>>,
  pub note: Entity<TextareaState>,
  pub habit_input: Entity<InputState>,
  pub todo_input: Entity<InputState>,
  pub rename_input: Entity<InputState>,
  pub update: crate::updates::UpdateState,
  pub focus: FocusHandle,
  pub pill_focus: FocusHandle,
  pub subscriptions: Vec<Subscription>,
  pub clock: Option<Task<()>>,
  pub update_task: Option<Task<()>>,
  pub nav_epoch: usize,
  pub previous_segment: usize,
  pub previous_segment_at: std::time::Instant,
}
impl Diary {
  pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
    let palette = crate::theme::apply(window, cx);
    let date = model::today();
    let loaded = crate::storage::store_path()
      .and_then(|path| crate::storage::load(&path).map(|data| (path, data)));
    let (data, storage, load_error) = match loaded {
      Ok((path, data)) => (data, Some(Storage::new(path)), None),
      Err(error) => (DiaryData::default(), None, Some(error)),
    };
    let note = cx.new(|cx| {
      TextareaState::new(window, cx)
        .default_value(data.note(date))
        .placeholder("Start writing today's entry…")
        .soft_wrap(true)
    });
    let habit_input = cx.new(|cx| InputState::new(window, cx).placeholder("Add habit"));
    let todo_input = cx.new(|cx| InputState::new(window, cx).placeholder("Add task"));
    let rename_input = cx.new(|cx| InputState::new(window, cx).placeholder("Habit name"));
    let subscriptions =
      inputs::subscribe(&note, &habit_input, &todo_input, &rename_input, window, cx);
    let mut app = Self {
      data,
      storage,
      load_error,
      save_error: None,
      palette,
      date,
      current_day: date,
      view: View::Day,
      calendar: None,
      menu: None,
      menu_position: point(px(0.), px(0.)),
      renaming: None,
      completed: BTreeSet::new(),
      completion_tasks: Default::default(),
      note,
      habit_input,
      todo_input,
      rename_input,
      update: Default::default(),
      focus: cx.focus_handle(),
      pill_focus: cx.focus_handle(),
      subscriptions,
      clock: None,
      update_task: None,
      nav_epoch: 0,
      previous_segment: 1,
      previous_segment_at: std::time::Instant::now(),
    };
    app
      .subscriptions
      .push(cx.observe_window_appearance(window, |this, window, cx| {
        this.palette = crate::theme::apply(window, cx);
        cx.notify();
      }));
    app.clock = Some(cx.spawn(async |weak, cx| {
      loop {
        cx.background_executor()
          .timer(std::time::Duration::from_secs(1))
          .await;
        if weak
          .update(cx, |this, cx| {
            let today = model::today();
            if today != this.current_day {
              this.current_day = today;
              cx.notify();
            }
            let error = this.storage.as_ref().and_then(Storage::error);
            if this.save_error != error {
              this.save_error = error;
              cx.notify();
            }
          })
          .is_err()
        {
          break;
        }
      }
    }));
    app.subscriptions.push(cx.on_app_quit(|this, _| {
      let _ = this.flush();
      async {}
    }));
    let weak = cx.weak_entity();
    window.on_window_should_close(cx, move |_, cx| {
      weak
        .update(cx, |this, cx| {
          let result = this.flush();
          if let Err(error) = result {
            this.save_error = Some(error);
            cx.notify();
            false
          } else {
            true
          }
        })
        .unwrap_or(true)
    });
    app.focus.focus(window, cx);
    app
  }
  pub fn persist(&self) {
    if let Some(storage) = &self.storage {
      storage.queue(&self.data);
    }
  }
  pub fn flush(&self) -> Result<(), String> {
    self.storage.as_ref().map(Storage::flush).unwrap_or(Ok(()))
  }
  pub fn select_day(&mut self, date: NaiveDate, window: &mut Window, cx: &mut Context<Self>) {
    if date > self.current_day {
      return;
    }
    self.navigate(View::Day, Some(date), window, cx);
  }
  pub fn month(&self) -> NaiveDate {
    self.calendar.unwrap_or(self.date).with_day(1).unwrap()
  }
}
