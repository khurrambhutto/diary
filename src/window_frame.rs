use gpui_kit::{
  base::{Root, RootPlugin},
  prelude::*,
  *,
};

const CORNER_RADIUS: f32 = 12.;
const RESIZE_INSET: f32 = 8.;

pub fn corner_radius(window: &Window) -> Pixels {
  let tiled =
    matches!(window.window_decorations(), Decorations::Client { tiling } if tiling.is_tiled());
  px(
    if window.is_maximized() || window.is_fullscreen() || tiled {
      0.
    } else {
      CORNER_RADIUS
    },
  )
}

// Own the frame so GPUI Component's fixed square border is never drawn.
pub struct WindowFrame;
impl WindowFrame {
  pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
    Self
  }
}
impl Render for WindowFrame {
  fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
    div()
  }
}
impl RootPlugin for WindowFrame {
  fn prepare(&mut self, window: &mut Window, _: &mut Context<Self>) {
    window.set_rem_size(px(14.));
  }
  fn decorate(
    &self,
    surface: AnyElement,
    _: &Root,
    window: &mut Window,
    _: &mut App,
  ) -> impl IntoElement {
    let client = matches!(window.window_decorations(), Decorations::Client { .. });
    let floating = corner_radius(window) > px(0.);
    let inset = if client && floating {
      px(RESIZE_INSET)
    } else {
      px(0.)
    };
    if client {
      window.set_client_inset(px(RESIZE_INSET));
    }
    let mut frame = div().relative().size_full().p(inset).child(surface);
    if inset > px(0.) {
      for edge in [
        ResizeEdge::Top,
        ResizeEdge::Bottom,
        ResizeEdge::Left,
        ResizeEdge::Right,
        ResizeEdge::TopLeft,
        ResizeEdge::TopRight,
        ResizeEdge::BottomLeft,
        ResizeEdge::BottomRight,
      ] {
        let top = matches!(
          edge,
          ResizeEdge::Top | ResizeEdge::TopLeft | ResizeEdge::TopRight
        );
        let bottom = matches!(
          edge,
          ResizeEdge::Bottom | ResizeEdge::BottomLeft | ResizeEdge::BottomRight
        );
        let left = matches!(
          edge,
          ResizeEdge::Left | ResizeEdge::TopLeft | ResizeEdge::BottomLeft
        );
        let right = matches!(
          edge,
          ResizeEdge::Right | ResizeEdge::TopRight | ResizeEdge::BottomRight
        );
        let cursor = match edge {
          ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
          ResizeEdge::Left | ResizeEdge::Right => CursorStyle::ResizeLeftRight,
          ResizeEdge::TopLeft | ResizeEdge::BottomRight => CursorStyle::ResizeUpLeftDownRight,
          _ => CursorStyle::ResizeUpRightDownLeft,
        };
        frame = frame.child(
          div()
            .absolute()
            .cursor(cursor)
            .when(top, |el| el.top_0().h(inset))
            .when(bottom, |el| el.bottom_0().h(inset))
            .when(left, |el| el.left_0().w(inset))
            .when(right, |el| el.right_0().w(inset))
            .when(!top && !bottom, |el| el.top(inset).bottom(inset))
            .when(!left && !right, |el| el.left(inset).right(inset))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
              window.start_window_resize(edge);
              cx.stop_propagation();
            }),
        );
      }
    }
    frame
  }
}
