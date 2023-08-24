# fltk-extras

Some extra fltk widgets:

## Buttons

```rust
use fltk::{prelude::*, *};
use fltk_extras::button::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(0, 0, 0);
    app::set_foreground_color(255, 255, 255);
    app::set_background2_color(128, 128, 128);
    let mut w = window::Window::default().with_size(400, 300);
    let col = group::Flex::default()
        .with_size(80, 200)
        .column()
        .center_of_parent();
    Toggle::default();
    RoundToggle::default();
    CheckButton::default().with_label("Done?");
    HollowRoundToggle::default();
    RadioButton::default().with_label("Radio");
    HoverButton::default().with_label("Hover");
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
```

![image](https://user-images.githubusercontent.com/37966791/212541355-91062d78-5c5d-4b7a-aa6d-e1be49cff340.png)

## Sliders

```rust
use fltk::{prelude::*, *};
use fltk_extras::slider::*;

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
```

![image](https://user-images.githubusercontent.com/37966791/212541392-2cd4fb08-4152-484a-86da-64b2bc476a0e.png)

## Dials

```rust
use fltk::{prelude::*, *};
use fltk_extras::dial::*;

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
```

![image](https://user-images.githubusercontent.com/37966791/212541425-f594a7bc-d7bc-49e5-90f3-03f52d437cce.png)

## RadioGroups

```rust
use fltk::{prelude::*, *};
use fltk_extras::radiogroup::*;

fn main() {
    let a = app::App::default();
    app::set_background_color(0, 0, 0);
    app::set_foreground_color(255, 255, 255);
    app::set_background2_color(128, 128, 128);
    let mut w = window::Window::default().with_size(400, 400);
    let mut rg = RadioGroup::default().with_size(200, 30).center_of_parent();
    button::RadioButton::default().with_label("First");
    button::RadioButton::default().with_label("Second");
    button::RadioButton::default().with_label("Third");
    rg.end();
    w.end();
    w.show();
    a.run().unwrap();
}
```

![s](https://github.com/fltk-rs/fltk-extras/assets/37966791/19f3a07e-c262-4f5e-a2e8-a16ba6acc8aa)