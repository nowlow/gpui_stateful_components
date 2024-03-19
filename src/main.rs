use gpui::{prelude::FluentBuilder, *};

mod state;

use state::State;

fn get_key(id: u8, key: &str) -> [u8; 16] {
    let mut bytes = [0u8; 16];

    bytes[0] = id;

    let key_bytes = key.as_bytes();
    for (i, &byte) in key_bytes.iter().enumerate().take(15) {
        bytes[i + 1] = byte;
    }
    bytes
}

#[derive(IntoElement)]
struct StatefulSquare {
    id: u8,
    bg_color: State<Rgba>,
    count: State<u32>,
}

impl StatefulSquare {
    fn build(id: u8, cx: &mut WindowContext) -> Self {
        Self {
            id,
            bg_color: State::new(rgb(0x0000ff), get_key(id, "bg_color"), cx),
            count: State::new(0, get_key(id, "count"), cx),
        }
    }
}

impl RenderOnce for StatefulSquare {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id as i32)
            .flex()
            .justify_center()
            .w_10()
            .h_10()
            .justify_center()
            .items_center()
            .when_some(self.bg_color.get(), |this, bg_color| this.bg(bg_color))
            .when_some(self.count.get(), |this, count| this.text_color(rgb( if count % 2 == 0 { 0xffffff } else { 0x000000 } )))
            .on_click(move |_this, cx| {
                if let Some(count) = self.count.get() {
                    if count % 2 == 0 {
                        self.bg_color.update(rgb(0x00ff00), cx);
                    } else {
                        self.bg_color.update(rgb(0x0000ff), cx)
                    }

                    self.count.update(count + 1, cx);
                }
            })
            .child(match self.count.get() {
                Some(count) => format!("{}", count).to_owned(),
                _ => "0".to_owned()
            })
    }
}

fn stateful_square(id: u8, cx: &mut WindowContext) -> impl IntoElement {
    StatefulSquare::build(id, cx)
}

struct MyApp {}

impl Render for MyApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0xffffff))
            .gap_10()
            .child(stateful_square(1, &mut cx.window_context()))
            .child(stateful_square(2, &mut cx.window_context()))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    traffic_light_position: Some(Point::new(px(12.0), px(12.0))),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| {
                state::ComponentState::init(cx);

                cx.new_view(|_cx| MyApp {})
            },
        );
    });
}
