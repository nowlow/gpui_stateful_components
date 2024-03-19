use gpui::*;

#[derive(Clone, Copy)]
pub struct Sidebar {
    bg: Rgba,
}

impl IntoElement for Sidebar {
    fn into_element(self) -> Self::Element {
        div().flex_col().bg(self.bg).size_full().w(px(300.0))
    }

    type Element = Div;
}

impl Sidebar {
    fn new() -> Self {
        Sidebar { bg: rgba(0x00ffff) }
    }
}

impl Render for Sidebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .bg(rgba(0xff00ff))
            .size_full()
            .w(px(300.0))
            .on_mouse_down(
                MouseButton::Left,
                |event: &MouseDownEvent, cx: &mut WindowContext| {
                    println!("On click!");
                    // self.bg = rgba(0xff0000)
                },
            )
    }
}
