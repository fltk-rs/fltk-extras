use fltk::{prelude::*, *};
use svg_widgets::slider::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(0, 0, 0);
    app::set_foreground_color(255, 255, 255);
    app::set_background2_color(128, 128, 128);
    let mut w = window::Window::default().with_size(400, 300);
    let col = group::Flex::default()
        .column()
        .with_size(200, 200)
        .center_of_parent();
    let _hslider = FancyHorSlider::default().with_label("Hor");
    let _vslider = FancyVertSlider::default().with_label("Vert");
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
