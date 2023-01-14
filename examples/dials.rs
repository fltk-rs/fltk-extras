use fltk::{enums::Color, prelude::*, *};
use svg_widgets::dial::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(255, 255, 255);
    let mut w = window::Window::default().with_size(400, 300);
    let col = group::Flex::default()
        .column()
        .with_size(100, 200)
        .center_of_parent();
    let _dial = Dial::default();
    let mut halfdial = HalfDial::default();
    halfdial.set_selection_color(Color::Red);
    halfdial.set_value(23);
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
