use crate::theme::{Palette, SERIF};
use gpui_kit::prelude::*;
use gpui_kit::{
  base::{Button, Checkbox},
  *,
};

pub fn icon(name: &str, size: f32, color: Hsla) -> Svg {
  svg()
    .path(format!("diary/{name}.svg"))
    .size(px(size))
    .text_color(color)
}
pub fn button(id: impl Into<ElementId>, label: impl Into<SharedString>, p: Palette) -> Button {
  Button::new(id)
    .accessibility_label(label)
    .flex()
    .items_center()
    .justify_center()
    .cursor_pointer()
    .rounded(px(8.))
    .text_color(p.soft)
    .hover(move |s| s.bg(p.subtle).text_color(p.ink))
    .active(move |s| s.bg(p.track))
    .focus_visible(move |s| {
      s.shadow(vec![
        BoxShadow::new(px(0.), px(0.), p.accent).spread_radius(px(2.)),
      ])
    })
}
pub fn icon_button(id: &'static str, name: &str, label: &'static str, p: Palette) -> Button {
  button(id, label, p)
    .size(px(32.))
    .child(icon(name, 18., p.soft))
}
pub fn text_button(id: impl Into<ElementId>, text: impl Into<SharedString>, p: Palette) -> Button {
  let text = text.into();
  button(id, text.clone(), p)
    .h(px(28.))
    .px(px(8.))
    .text_color(p.strong)
    .child(text)
}
pub fn card(p: Palette) -> Div {
  div()
    .flex()
    .flex_col()
    .min_w_0()
    .bg(p.raised)
    .border_1()
    .border_color(p.line)
    .rounded(px(12.))
    .px(px(16.))
    .pt(px(14.))
    .pb(px(8.))
}
pub fn header(title: impl Into<SharedString>, meta: impl Into<SharedString>, p: Palette) -> Div {
  div()
    .flex()
    .items_center()
    .justify_between()
    .gap(px(8.))
    .mb(px(6.))
    .child(div().font_weight(FontWeight::BOLD).child(title.into()))
    .child(
      div()
        .text_size(px(13.))
        .text_color(p.soft)
        .child(meta.into()),
    )
}
pub fn heading(text: impl Into<SharedString>) -> Div {
  div()
    .font_family(SERIF)
    .text_size(px(34.))
    .line_height(px(39.1))
    .font_weight(FontWeight::MEDIUM)
    .child(text.into())
}
pub fn streak_chip(streak: usize, p: Palette) -> Div {
  div()
    .flex_none()
    .h(px(20.))
    .px(px(7.))
    .rounded_full()
    .bg(p.subtle)
    .text_color(p.soft)
    .text_size(px(12.))
    .child(format!("🔥 {streak}"))
}

pub fn checkbox(
  id: impl Into<ElementId>,
  label: impl Into<SharedString>,
  checked: bool,
  round: bool,
  p: Palette,
) -> Checkbox {
  Checkbox::new(id)
    .checked(checked)
    .accessibility_label(label)
    .flex()
    .items_center()
    .justify_center()
    .cursor_pointer()
    .size(px(18.))
    .flex_none()
    .rounded(px(if round { 9. } else { 4. }))
    .border_1()
    .border_color(if checked { p.accent } else { p.faint })
    .when(checked, |el| {
      el.bg(p.accent).child(icon("check", 12., p.on_accent))
    })
    .hover(move |s| s.border_color(p.accent))
    .focus_visible(move |s| {
      s.shadow(vec![
        BoxShadow::new(px(0.), px(0.), p.accent).spread_radius(px(2.)),
      ])
    })
}
