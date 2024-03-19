use gpui::{prelude::FluentBuilder, *};

#[derive(IntoElement)]
pub struct Sidebar {
    on_click: Option<Box<dyn Fn(&bool, &mut WindowContext) + 'static>>,
    bg_color: Option<u32>,
}

impl Sidebar {
    pub fn on_click(mut self, click_handler: impl Fn(&bool, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(click_handler));
        self
    }

    pub fn bg_color(mut self, color: u32) -> Self {
        self.bg_color = Some(color);
        self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(1)
            .w_10()
            .h_10()
            .when_some(self.bg_color, |this, bg_color| this.bg(rgb(bg_color)))
            .when_some(self.on_click, |this, my_click| {
                this.on_click(move |a, b| {
                    my_click(&true, b);
                })
            })
    }
}

pub fn sidebar() -> Sidebar {
    Sidebar {
        on_click: None,
        bg_color: None,
    }
}
