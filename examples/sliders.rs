use fltk::{enums::Color, prelude::*, *};
use svg_widgets::slider::*;

fn main() {
    let a = app::App::default();
    let mut w = window::Window::default().with_size(400, 300);
    w.set_color(Color::White);
    let col = group::Flex::default()
    .column()
    .with_size(200, 200)
    .center_of_parent();
    let _hslider = FancyHorSlider::default();
    let _vslider = FancyVertSlider::default();
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
