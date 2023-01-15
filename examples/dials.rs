use fltk::{prelude::*, *};
use svg_widgets::dial::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(0, 0, 0);
    app::set_foreground_color(255, 255, 255);
    app::set_background2_color(128, 128, 128);
    let mut w = window::Window::default().with_size(400, 300);
    let col = group::Flex::default()
        .column()
        .with_size(100, 200)
        .center_of_parent();
    let mut dial = Dial::default();
    dial.modifiable(false);
    dial.set_value(75);
    let mut halfdial = HalfDial::default();
    halfdial.set_value(23);
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
