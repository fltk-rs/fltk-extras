use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc,
};

#[derive(Debug, Clone)]
pub struct Dial {
    dial: valuator::FillDial,
    value: Arc<AtomicI32>,
    modifiable: Arc<AtomicBool>,
}

impl Default for Dial {
    fn default() -> Self {
        Dial::new(0, 0, 0, 0, "")
    }
}

impl Dial {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut dial = valuator::FillDial::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top);
        dial.set_label_size(app::font_size() + 2);
        dial.set_frame(FrameType::NoBox);
        dial.set_color(Color::color_average(dial.color(), Color::Foreground, 0.9));
        dial.set_selection_color(RED);
        let value = Arc::new(AtomicI32::new(0));
        let value_c = value.clone();
        dial.draw(move |w| {
            draw::set_draw_color(w.parent().unwrap().color());
            draw::draw_pie(
                w.x() + (w.w() / 6),
                w.y() + (w.h() / 6),
                w.w() * 2 / 3,
                w.h() * 2 / 3,
                0.,
                360.,
            );
            draw::set_font(Font::Helvetica, 16);
            draw::set_draw_color(w.label_color());
            draw::draw_text2(
                &value_c.load(Ordering::Relaxed).to_string(),
                w.x(),
                w.y(),
                w.w(),
                w.h(),
                Align::Center,
            );
        });
        let modifiable = Arc::new(AtomicBool::new(true));
        let mod_c = modifiable.clone();
        let val_c = value.clone();
        dial.set_callback(move |d| {
            if mod_c.load(Ordering::Relaxed) {
                val_c.store((d.value() * 100.) as i32, Ordering::Relaxed);
            } else {
                d.set_value(val_c.load(Ordering::Relaxed) as f64 / 100.)
            }
        });
        Self {
            dial,
            value,
            modifiable,
        }
    }
    pub fn set_value(&mut self, val: i32) {
        self.value.store(val, Ordering::Relaxed);
        self.dial.set_value(val as f64 / 100.);
    }
    pub fn modifiable(&mut self, val: bool) {
        self.modifiable.store(val, Ordering::Relaxed);
    }
    pub fn set_callback<F: 'static + FnMut(&mut Self)>(&mut self, mut cb: F) {
        let mut s = self.clone();
        self.dial.set_callback(move |_| {
            cb(&mut s);
            app::redraw();
        });
    }
}

fltk::widget_extends!(Dial, valuator::FillDial, dial);

#[derive(Debug, Clone)]
pub struct HalfDial {
    main_wid: group::Flex,
    value: Arc<AtomicI32>,
    value_frame: frame::Frame,
}

impl Default for HalfDial {
    fn default() -> Self {
        HalfDial::new(0, 0, 0, 0, "")
    }
}

impl HalfDial {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let value = AtomicI32::new(0);
        let mut main_wid = group::Flex::new(x, y, w, h, None)
            .with_label(label)
            .with_align(Align::Top)
            .column();
        main_wid.set_label_size(app::font_size() + 3);
        let mut value_frame = frame::Frame::default().with_label("0");
        value_frame.set_label_size(app::font_size() + 12);
        main_wid.set_selection_color(RED);
        main_wid.end();
        main_wid.fixed(&value_frame, 30);
        let value = Arc::new(value);
        let value_c = value.clone();
        main_wid.draw(move |w| {
            let parent = w.parent().unwrap();
            let parent_col = parent.color();
            draw::set_draw_color(Color::color_average(parent_col, Color::Foreground, 0.9));
            draw::draw_pie(w.x(), w.y(), w.w(), w.h(), -45., 225.);
            draw::set_draw_color(w.selection_color());
            let val = value_c.load(Ordering::Relaxed);
            let val = if val > 100 { 100 } else { val };
            draw::draw_pie(
                w.x(),
                w.y(),
                w.w(),
                w.h(),
                (100 - val) as f64 * 2.7 - 45.,
                225.,
            );
            draw::set_draw_color(parent_col);
            draw::draw_pie(
                w.x() + (w.w() / 4),
                w.y() + (w.h() / 4),
                w.w() * 2 / 4,
                w.h() * 2 / 4,
                0.,
                360.,
            );
            w.draw_children();
        });
        main_wid.resize_callback({
            let mut value_frame = value_frame.clone();
            move |_, x, y, w, _h| {
                value_frame.resize(x, y + 80, w, 40);
            }
        });
        Self {
            main_wid,
            value,
            value_frame,
        }
    }
    pub fn set_value(&mut self, val: i32) {
        self.value.store(val, Ordering::Relaxed);
        self.value_frame.set_label(&val.to_string());
        self.main_wid.redraw();
    }
}

widget_extends!(HalfDial, group::Flex, main_wid);
