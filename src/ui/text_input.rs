use crate::theme::Palette;
use gpui_kit::{
  base::input::{Input, InputBase, InputEditorStyle, InputState},
  prelude::*,
  *,
};

pub fn text_input(
  state: &Entity<InputState>,
  label: &'static str,
  p: Palette,
  window: &Window,
  cx: &mut App,
) -> InputBase {
  state.update(cx, |state, _| {
    state.set_editor_style(InputEditorStyle {
      foreground: p.ink,
      muted_foreground: p.faint,
      background: p.raised,
      selection: p.tint,
      caret: p.strong,
      ..Default::default()
    })
  });
  let handle = state.read(cx).focus_handle(cx);
  InputBase::new(("input", state.entity_id()))
    .track_focus(&handle)
    .focused(handle.is_focused(window))
    .accessibility_label(label)
    .aria_value(state.read(cx).value())
    .flex()
    .flex_1()
    .min_w_0()
    .h(px(26.))
    .text_size(px(14.))
    .styles(|styles| styles.focused(|style| style.border_b_1().border_color(p.accent)))
    .child(Input::new(state))
}

pub struct Hint(pub String, pub Palette);
impl Render for Hint {
  fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
    div()
      .px(px(8.))
      .py(px(4.))
      .rounded(px(6.))
      .bg(self.1.raised)
      .text_color(self.1.ink)
      .text_size(px(12.))
      .font_family(crate::theme::SANS)
      .child(self.0.clone())
  }
}
