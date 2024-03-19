use gpui::*;

#[derive(IntoElement)]
pub struct Interface {}

impl Interface {
    pub fn new() -> Self {
        Interface {}
    }
}

impl RenderOnce for Interface {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex_col()
            .bg(rgb(0x00ff00))
            .size_full()
            .p(px(10.0))
            .child("Interface")
    }
}
