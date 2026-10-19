use crate::{app::Diary, model::today};
use gpui_kit::{
  base::input::{InputEvent, InputState, TextareaState},
  *,
};

pub fn subscribe(
  note: &Entity<TextareaState>,
  habit: &Entity<InputState>,
  todo: &Entity<InputState>,
  rename: &Entity<InputState>,
  window: &mut Window,
  cx: &mut Context<Diary>,
) -> Vec<Subscription> {
  vec![
    cx.subscribe_in(note, window, |this, state, event, _, cx| {
      if matches!(event, InputEvent::Change) && this.date == today() {
        this
          .data
          .set_note(this.date, state.read(cx).value().to_string());
        this.persist();
        cx.notify();
      }
    }),
    cx.subscribe_in(habit, window, |this, state, event, window, cx| {
      if matches!(event, InputEvent::PressEnter { .. }) {
        let text = state.read(cx).value().to_string();
        this.data.add_habit(&text);
        state.update(cx, |state, cx| state.set_value("", window, cx));
        this.persist();
        cx.notify();
      }
    }),
    cx.subscribe_in(todo, window, |this, state, event, window, cx| {
      if matches!(event, InputEvent::PressEnter { .. }) {
        let text = state.read(cx).value().to_string();
        this.data.add_todo(&text);
        state.update(cx, |state, cx| state.set_value("", window, cx));
        this.persist();
        cx.notify();
      }
    }),
    cx.subscribe_in(rename, window, |this, _, event, _, cx| {
      if matches!(event, InputEvent::PressEnter { .. } | InputEvent::Blur) {
        this.commit_rename(cx);
      }
    }),
  ]
}
