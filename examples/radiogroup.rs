use fltk::{prelude::*, *};
use fltk_extras::radiogroup::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(0, 0, 0);
    app::set_foreground_color(255, 255, 255);
    app::set_background2_color(128, 128, 128);
    let mut w = window::Window::default().with_size(400, 400);
    let mut rg = RadioGroup::default().with_size(200, 30).center_of_parent();
    rg.set_radius(15.); // this is the default
    button::RadioButton::default().with_label("First");
    button::RadioButton::default().with_label("Second");
    button::RadioButton::default().with_label("Third");
    rg.end();
    w.end();
    w.show();
    a.run().unwrap();
}
