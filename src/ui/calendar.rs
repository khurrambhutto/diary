use crate::{
  app::Diary,
  model::{key, words},
  theme::SERIF,
  ui::primitives::*,
};
use chrono::{Datelike, Days, Months};
use gpui_kit::prelude::*;
use gpui_kit::*;
impl Diary {
  pub fn calendar_overlay(&self, cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let month = self.month();
    let next = month + Months::new(1);
    let can_next = next <= self.current_day;
    let mut panel = div()
      .relative()
      .w(px(292.))
      .p(px(12.))
      .bg(p.raised)
      .border_1()
      .border_color(p.line)
      .rounded(px(12.))
      .child(
        div()
          .flex()
          .justify_between()
          .items_center()
          .mb(px(6.))
          .child(
            div()
              .font_family(SERIF)
              .text_size(px(18.))
              .font_weight(FontWeight::MEDIUM)
              .child(month.format("%B %Y").to_string()),
          )
          .child(
            div()
              .flex()
              .gap(px(2.))
              .child(
                icon_button("prev-month", "chevron-left", "Previous month", p)
                  .size(px(28.))
                  .on_click(cx.listener(move |this, _, _, cx| {
                    this.calendar = Some(month - Months::new(1));
                    cx.notify();
                  })),
              )
              .child(
                icon_button("next-month", "chevron-right", "Next month", p)
                  .size(px(28.))
                  .disabled(!can_next)
                  .when(!can_next, |el| el.opacity(0.35))
                  .on_click(cx.listener(move |this, _, _, cx| {
                    this.calendar = Some(next);
                    cx.notify();
                  })),
              ),
          ),
      )
      .child(
        div()
          .flex()
          .h(px(24.))
          .children(["S", "M", "T", "W", "T", "F", "S"].map(|label| {
            div()
              .flex_1()
              .text_center()
              .text_size(px(12.))
              .text_color(p.soft)
              .child(label)
          })),
      );
    let offset = month.weekday().num_days_from_sunday() as usize;
    let count = (next - month).num_days() as usize;
    for week in 0..(count + offset).div_ceil(7) {
      let mut row = div().flex().gap(px(2.)).mb(px(2.));
      for weekday in 0..7 {
        let index = week * 7 + weekday;
        if index < offset || index >= offset + count {
          row = row.child(div().flex_1().h(px(36.)));
          continue;
        }
        let date = month + Days::new((index - offset) as u64);
        let selected = date == self.date;
        let today = date == self.current_day;
        let future = date > self.current_day;
        let written = self
          .data
          .journal
          .get(&key(date))
          .is_some_and(|s| words(s) > 0);
        let day = button(
          SharedString::from(format!("day-{}", key(date))),
          date.format("%A, %B %-d, %Y").to_string(),
          p,
        )
        .relative()
        .flex_1()
        .h(px(36.))
        .disabled(future)
        .text_color(if future { p.faint } else { p.ink })
        .when(today, |el| {
          el.border_1()
            .border_color(p.accent)
            .text_color(p.strong)
            .font_weight(FontWeight::BOLD)
        })
        .when(selected, |el| el.bg(p.strong).text_color(p.on_accent))
        .child(date.day().to_string())
        .when(written, |el| {
          el.child(
            div()
              .absolute()
              .bottom(px(4.))
              .size(px(4.))
              .rounded_full()
              .bg(if selected { p.on_accent } else { p.accent }),
          )
        })
        .on_click(cx.listener(move |this, _, window, cx| this.select_day(date, window, cx)));
        row = row.child(day);
      }
      panel = panel.child(row);
    }
    panel = panel.child(
      div()
        .flex()
        .justify_between()
        .items_center()
        .border_t_1()
        .border_color(p.line)
        .mt(px(8.))
        .pt(px(8.))
        .child(
          div()
            .flex()
            .items_center()
            .gap(px(6.))
            .text_size(px(12.))
            .text_color(p.soft)
            .child(div().size(px(4.)).rounded_full().bg(p.accent))
            .child("Entry written"),
        )
        .child(text_button("calendar-today", "Today", p).on_click(
          cx.listener(|this, _, window, cx| this.select_day(this.current_day, window, cx)),
        )),
    );
    div()
      .absolute()
      .inset_0()
      .child(
        div()
          .id("calendar-dismiss")
          .absolute()
          .inset_0()
          .on_click(cx.listener(|this, _, _, cx| {
            this.calendar = None;
            cx.notify();
          })),
      )
      .child(div().absolute().left(px(16.)).top(px(84.)).child(panel))
  }
}
