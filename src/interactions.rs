use crate::{
  app::{Diary, View},
  model,
};
use chrono::Days;
use gpui_kit::*;
use std::time::{Duration, Instant};

impl Diary {
  pub fn segment(&self) -> Option<usize> {
    match self.view {
      View::Analytics => Some(2),
      View::Day if self.date == self.current_day => Some(1),
      View::Day if Some(self.date) == self.current_day.checked_sub_days(Days::new(1)) => Some(0),
      _ => None,
    }
  }
  pub fn navigate(
    &mut self,
    view: View,
    date: Option<chrono::NaiveDate>,
    window: &mut Window,
    cx: &mut Context<Self>,
  ) {
    self.commit_rename(cx);
    self.previous_segment = self.segment().unwrap_or(1);
    self.previous_segment_at = Instant::now();
    self.calendar = None;
    self.menu = None;
    self.view = view;
    self.nav_epoch += 1;
    if let Some(date) = date
      && self.date != date
    {
      self.date = date;
      // In-memory notes update on each edit; a pending save belongs to its original date.
      let text = self.data.note(date).to_owned();
      self.note.update(cx, |state, cx| {
        state.set_value(text, window, cx);
        state.set_placeholder(
          if date == model::today() {
            "Start writing today's entry…"
          } else {
            "Nothing was written this day."
          },
          window,
          cx,
        );
      });
    }
    self.focus.focus(window, cx);
    cx.notify();
  }
  pub fn begin_rename(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
    let name = self
      .data
      .habits
      .habits
      .iter()
      .find(|h| h.id == id)
      .map(|h| h.label.clone())
      .unwrap_or_default();
    self.menu = None;
    self.renaming = Some(id);
    self.rename_input.update(cx, |state, cx| {
      state.set_value(name, window, cx);
      state.focus(window, cx);
    });
    cx.notify();
  }
  pub fn commit_rename(&mut self, cx: &mut Context<Self>) {
    if let Some(id) = self.renaming.take() {
      let value = self.rename_input.read(cx).value();
      if !value.trim().is_empty() {
        if let Some(habit) = self.data.habits.habits.iter_mut().find(|h| h.id == id) {
          habit.label = value.trim().into();
        }
        self.persist();
      }
      cx.notify();
    }
  }
  pub fn toggle_todo(&mut self, id: String, cx: &mut Context<Self>) {
    self
      .completion_tasks
      .retain(|id, _| self.completed.contains(id));
    if self.completed.remove(&id) {
      self.completion_tasks.remove(&id);
    } else {
      self.completed.insert(id.clone());
      let task_id = id.clone();
      let task = cx.spawn(async move |weak, cx| {
        cx.background_executor()
          .timer(Duration::from_millis(820))
          .await;
        let _ = weak.update(cx, |this, cx| {
          if this.completed.remove(&task_id) {
            this.data.todos.retain(|t| t.id != task_id);
            this.persist();
            cx.notify();
          }
        });
      });
      self.completion_tasks.insert(id, task);
    }
    cx.notify();
  }
  pub fn keyboard(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
    let modifiers = event.keystroke.modifiers;
    let key = event.keystroke.key.as_str();
    if key == "escape" {
      self.calendar = None;
      self.menu = None;
      self.renaming = None;
      self
        .habit_input
        .update(cx, |s, cx| s.set_value("", window, cx));
      self
        .todo_input
        .update(cx, |s, cx| s.set_value("", window, cx));
      self.focus.focus(window, cx);
      cx.notify();
      return;
    }
    let command = if cfg!(target_os = "macos") {
      modifiers.platform
    } else {
      modifiers.control
    };
    if command {
      match key {
        "t" => self.select_day(self.current_day, window, cx),
        "[" => self.select_day(self.date - Days::new(1), window, cx),
        "]" => self.select_day(self.date + Days::new(1), window, cx),
        "s" => {
          if let Err(e) = self.flush() {
            self.save_error = Some(e);
            cx.notify();
          }
        }
        _ => return,
      }
      cx.stop_propagation();
    }
  }
  pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
    match self.flush() {
      Ok(()) => window.remove_window(),
      Err(e) => {
        self.save_error = Some(e);
        cx.notify();
      }
    }
  }
}
