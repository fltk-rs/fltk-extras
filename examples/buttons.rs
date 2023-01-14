use fltk::{enums::Color, prelude::*, *};
use svg_widgets::button::*;

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default().with_size(400, 300);
    w.set_color(Color::White);
    let col = group::Flex::default()
        .with_size(80, 200)
        .column()
        .center_of_parent();
    Toggle::default();
    RoundToggle::default();
    CheckButton::default().with_label("Done?");
    HollowRoundToggle::default();
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
