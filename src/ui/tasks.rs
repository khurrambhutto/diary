use super::text_input::text_input;
use crate::{app::Diary, ui::primitives::*};
use gpui_kit::prelude::*;
use gpui_kit::*;
impl Diary {
  pub fn todos(&self, window: &Window, cx: &mut Context<Self>) -> Div {
    let p = self.palette;
    let open = self.data.todos.len() - self.completed.len();
    let mut list = card(p).child(header(
      "To Do",
      if open == 1 {
        "1 task".into()
      } else {
        format!("{open} tasks")
      },
      p,
    ));
    if self.data.todos.is_empty() {
      list = list.child(
        div()
          .my(px(2.))
          .text_color(p.soft)
          .child("Nothing on your list."),
      );
    }
    for todo in &self.data.todos {
      let id = todo.id.clone();
      let done = self.completed.contains(&id);
      let row = div()
        .flex()
        .items_start()
        .gap(px(10.))
        .py(px(7.))
        .min_w_0()
        .child(
          checkbox(
            SharedString::from(format!("todo-{id}")),
            todo.text.clone(),
            done,
            true,
            p,
          )
          .mt(px(1.))
          .on_change({
            let toggle =
              cx.listener(move |this, _: &ClickEvent, _, cx| this.toggle_todo(id.clone(), cx));
            move |_, event, window, cx| toggle(event, window, cx)
          }),
        )
        .child(
          div()
            .flex_1()
            .min_w_0()
            .text_color(if done { p.faint } else { p.ink })
            .when(done, |el| el.line_through())
            .child(todo.text.clone()),
        );
      if done {
        list = list.child(div().overflow_hidden().child(row).with_animation(
          SharedString::from(format!("complete-{}", todo.id)),
          Animation::new(std::time::Duration::from_millis(820)),
          |el, t| {
            let collapse = ((t - 0.73) / 0.27).clamp(0., 1.);
            el.max_h(px(80. * (1. - collapse))).opacity(1. - collapse)
          },
        ));
      } else {
        list = list.child(row);
      }
    }
    list.child(
      div()
        .flex()
        .items_center()
        .gap(px(10.))
        .h(px(34.))
        .child(icon("plus", 18., p.faint))
        .child(text_input(&self.todo_input, "Add task", p, window, cx)),
    )
  }
}
