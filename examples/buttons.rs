use fltk::{prelude::*, *};
use fltk_extras::button::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(0, 0, 0);
    app::set_foreground_color(255, 255, 255);
    app::set_background2_color(128, 128, 128);
    let mut w = window::Window::default().with_size(400, 400);
    let col = group::Flex::default()
        .with_size(80, 200)
        .column()
        .center_of_parent();
    Toggle::default();
    RoundToggle::default();
    CheckButton::default().with_label("Done?");
    HollowRoundToggle::default();
    RadioButton::default().with_label("Radio");
    HoverButton::default().with_label("Hello");
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
