use crate::{
  app::Diary,
  model::{key, streaks, words},
  theme::SERIF,
  ui::primitives::*,
};
use chrono::{Datelike, Days};
use gpui_kit::prelude::*;
use gpui_kit::*;
impl Diary {
  pub fn analytics(&self, _cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let today = self.current_day;
    let dates = self.data.entry_dates(today);
    let runs = streaks(&dates, today);
    let month = dates
      .iter()
      .filter(|d| d.month() == today.month() && d.year() == today.year())
      .count();
    let since = dates
      .first()
      .map(|d| d.format("Since %B %-d, %Y").to_string())
      .unwrap_or_default();
    let entry_noun = if dates.len() == 1 { "entry" } else { "entries" };
    let best_noun = if runs.best == 1 { "day" } else { "days" };
    let month_noun = if month == 1 { "day" } else { "days" };
    let summary = format!(
      "You've written {} {entry_noun}. You're on a {}-day streak, and your longest ran {} {best_noun}. This month you've written on {month} {month_noun}.",
      dates.len(),
      runs.current,
      runs.best
    );
    let mut page = div()
      .flex()
      .flex_col()
      .child(div().h(px(20.)).text_color(p.soft).child(since))
      .child(heading("Analytics"))
      .child(
        div()
          .mt(px(16.))
          .font_family(SERIF)
          .text_size(px(20.))
          .line_height(px(32.))
          .text_color(p.soft)
          .child(summary),
      );
    let mut heat = div().mt(px(36.)).child(
      div()
        .flex()
        .justify_between()
        .items_center()
        .mb(px(12.))
        .child(
          div()
            .font_weight(FontWeight::BOLD)
            .child("Writing, past year"),
        )
        .child(
          div()
            .flex()
            .gap(px(4.))
            .items_center()
            .text_size(px(12.))
            .text_color(p.soft)
            .child("Fewer words")
            .children(p.heat.map(|c| div().size(px(10.)).rounded(px(2.)).bg(c)))
            .child("More"),
        ),
    );
    let start = today - Days::new(364 + today.weekday().num_days_from_sunday() as u64);
    let mut labels = div()
      .relative()
      .w(px(583.))
      .h(px(16.))
      .text_size(px(12.))
      .text_color(p.soft);
    let mut grid = div().flex().gap(px(2.)).mt(px(4.));
    for week in 0..53 {
      let mut col = div().flex().flex_col().gap(px(2.));
      for day in 0..7 {
        let date = start + Days::new(week * 7 + day);
        let future = date > today;
        let wc = words(self.data.note(date));
        let level = match wc {
          0 => 0,
          1..=199 => 1,
          200..=449 => 2,
          450..=749 => 3,
          _ => 4,
        };
        if date.day() == 1 {
          labels = labels.child(
            div()
              .absolute()
              .left(px(week as f32 * 11.))
              .child(date.format("%b").to_string()),
          );
        }
        col = col.child(
          div()
            .id(SharedString::from(format!("heat-{}", key(date))))
            .size(px(9.))
            .rounded(px(2.))
            .when(!future, |el| el.bg(p.heat[level]))
            .tooltip(move |_, cx| {
              cx.new(|_| {
                super::text_input::Hint(format!("{} · {wc} words", date.format("%b %-d, %Y")), p)
              })
              .into()
            }),
        );
      }
      grid = grid.child(col);
    }
    heat = heat.child(
      div()
        .id("heatmap-scroll")
        .overflow_x_scroll()
        .child(labels)
        .child(grid),
    );
    page = page.child(heat).child(
      div()
        .mt(px(36.))
        .mb(px(12.))
        .font_weight(FontWeight::BOLD)
        .child("Habits"),
    );
    if self.data.habits.habits.is_empty() {
      return page.child(
        div()
          .text_color(p.soft)
          .child("Add a habit on the Today page to see it here."),
      );
    }
    for pair in self.data.habits.habits.chunks(2) {
      let mut row = div().flex().gap(px(16.)).mb(px(16.));
      for habit in pair {
        let dates = self.data.habit_dates(&habit.id, today);
        let runs = streaks(&dates, today);
        let mut strip = div().flex().gap(px(2.)).h(px(16.)).mt(px(14.));
        let mut checked = 0;
        for n in (0..30).rev() {
          let date = today - Days::new(n);
          let on = self.data.checked(&habit.id, date);
          if on {
            checked += 1;
          }
          strip = strip.child(div().flex_1().h_full().rounded(px(2.)).bg(if on {
            p.accent
          } else {
            p.track
          }));
        }
        let mut stats = div().flex().gap(px(8.)).mt(px(14.));
        for (value, label) in [
          (runs.current.to_string(), "streak"),
          (runs.best.to_string(), "best"),
          (dates.len().to_string(), "check-ins"),
          (
            format!("{}%", (checked as f32 / 30. * 100.).round() as usize),
            "last 30 days",
          ),
        ] {
          stats = stats.child(
            div()
              .flex_1()
              .min_w_0()
              .child(
                div()
                  .font_family(SERIF)
                  .text_size(px(20.))
                  .line_height(px(24.))
                  .child(value),
              )
              .child(div().text_size(px(12.)).text_color(p.soft).child(label)),
          );
        }
        row = row.child(
          card(p)
            .flex_1()
            .pb(px(16.))
            .child(
              div()
                .flex()
                .items_center()
                .justify_between()
                .gap(px(8.))
                .child(
                  div()
                    .font_weight(FontWeight::BOLD)
                    .child(habit.label.clone()),
                )
                .when(runs.current > 0, |el| {
                  el.child(streak_chip(runs.current, p))
                }),
            )
            .child(strip)
            .child(
              div()
                .flex()
                .justify_between()
                .mt(px(4.))
                .text_size(px(12.))
                .text_color(p.soft)
                .child("30 days ago")
                .child("Today"),
            )
            .child(stats),
        );
      }
      if pair.len() == 1 {
        row = row.child(div().flex_1());
      }
      page = page.child(row);
    }
    page
  }
}
