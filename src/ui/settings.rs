use crate::{app::Diary, theme::SERIF, ui::primitives::*, updates::UpdateState};
use gpui_kit::prelude::*;
use gpui_kit::*;
impl Diary {
  pub fn settings(&self, cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let (text, label, busy) = match &self.update {
      UpdateState::Idle => (
        "Check for a new version of Diary.".into(),
        "Check for updates",
        false,
      ),
      UpdateState::Checking => ("Checking for updates…".into(), "Checking…", true),
      UpdateState::Current => ("Diary is up to date".into(), "Check again", false),
      UpdateState::Available { version, .. } => (
        format!("Version {version} is available"),
        "View release",
        false,
      ),
      UpdateState::Error(error) => (
        format!("Couldn't reach the update server. {error}"),
        "Try again",
        false,
      ),
    };
    div()
      .flex()
      .flex_col()
      .pt(px(20.))
      .child(heading("Settings"))
      .child(
        card(p)
          .mt(px(24.))
          .p(px(20.))
          .flex_row()
          .items_center()
          .gap(px(16.))
          .child(
            div()
              .flex_none()
              .size(px(56.))
              .flex()
              .items_center()
              .justify_center()
              .rounded(px(14.))
              .bg(p.tint)
              .text_color(p.strong)
              .font_family(SERIF)
              .text_size(px(32.))
              .child("D"),
          )
          .child(
            div()
              .min_w_0()
              .child(
                div()
                  .flex()
                  .items_baseline()
                  .gap(px(10.))
                  .child(
                    div()
                      .font_family(SERIF)
                      .text_size(px(20.))
                      .font_weight(FontWeight::MEDIUM)
                      .child("Diary"),
                  )
                  .child(
                    div()
                      .text_size(px(13.))
                      .text_color(p.soft)
                      .child(concat!("Version ", env!("CARGO_PKG_VERSION"))),
                  ),
              )
              .child(
                div()
                  .mt(px(2.))
                  .text_color(p.soft)
                  .child("A quiet notebook for daily writing and small habits."),
              ),
          ),
      )
      .child(
        card(p).mt(px(16.)).px(px(20.)).py(px(16.)).child(
          div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(16.))
            .child(
              div()
                .flex_1()
                .min_w_0()
                .child(div().font_weight(FontWeight::BOLD).child("Software update"))
                .child(div().mt(px(2.)).text_color(p.soft).child(text)),
            )
            .child(
              text_button("check-updates", label, p)
                .flex_none()
                .h(px(32.))
                .px(px(14.))
                .border_1()
                .border_color(p.line)
                .disabled(busy)
                .on_click(cx.listener(|this, _, _, cx| {
                  if let UpdateState::Available { url, .. } = &this.update {
                    cx.open_url(url);
                  } else {
                    this.check_updates(cx);
                  }
                })),
            ),
        ),
      )
  }
}
