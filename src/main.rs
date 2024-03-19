use gpui::*;

mod state;

#[derive(IntoElement)]
struct Sidebar {
    bg_color: Option<Rgba>,
}

impl Sidebar {
    fn build(cx: &mut WindowContext) -> Self {
        Self {
            bg_color: state::ComponentState::add(&"sidebar", rgb(0x0000ff), cx)
        }
    }

    fn update(self, bg_color: Rgba, cx: &mut WindowContext) {
        state::ComponentState::update::<Rgba>(&"sidebar", |this, _| {
            *this = bg_color;
        }, cx)
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(1)
            .w_10()
            .h_10()
            .bg(match self.bg_color {
                Some(bg) => bg,
                None => rgba(0xff0000)
            })
            .on_click(move |_this, cx| {
                self.update(rgba(0x00ff00), cx);
            })
    }
}

fn sidebar(cx: &mut WindowContext) -> impl IntoElement {
    Sidebar::build(cx)
}

struct MyApp {
    is_pressed: bool,
}

impl Render for MyApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0xffffff))
            .child(sidebar(&mut cx.window_context()))
            // .child(cx.new_view(|cx| {
            //     div().w_10().h_10().bg(rgb(0x00ff00));
            // }))
    }
}

// new_sidebar::sidebar()
//     .bg_color(self.sidebar_background_color)
//     .on_click(cx.listener(move |this, b, view_cx| {
//         // println!("click: {:?} {:?}", a, b);
//         this.sidebar_background_color = 0x00ff00;
//         this.is_pressed = !this.is_pressed;
//     })),

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

                cx.new_view(|_cx| MyApp {
                    is_pressed: false,
                })
            },
        );
    });
}
