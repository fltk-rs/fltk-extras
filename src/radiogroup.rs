use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use fltk_sys::{fl::Fl_set_box_type_cb, widget::Fl_Widget_set_box};
use std::{
    mem,
    sync::atomic::{AtomicU8, Ordering},
};
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

const LEFT_RECT_UP: i32 = 100;
const LEFT_RECT_DOWN: i32 = 101;
const MID_RECT_UP: i32 = 102;
const MID_RECT_DOWN: i32 = 103;
const RIGHT_RECT_UP: i32 = 104;
const RIGHT_RECT_DOWN: i32 = 105;

static ANGLE: AtomicU8 = AtomicU8::new(15);

// Necessary boilerplate to avoid UB in casting out of range enum value
fn frame_cb(
    old_frame: i32,
    cb: fn(x: i32, y: i32, w: i32, h: i32, c: Color),
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    unsafe {
        Fl_set_box_type_cb(old_frame, Some(mem::transmute(cb)), x, y, w, h);
    }
}

struct Position {
    x: f32,
    y: f32,
}

// copied with some modification from: https://gitlab.com/snakedye/snui/-/blob/master/snui/src/widgets/shapes/rectangle.rs?ref_type=heads
fn rounded_rect(mut pb: PathBuilder, width: f32, height: f32, radius: [f32; 4]) -> Path {
    let (x, y) = (0., 0.);
    let mut cursor = Position { x: 0., y: 0. };
    let [tl, tr, br, bl] = radius;
    cursor.y += tl;
    pb.move_to(cursor.x, cursor.y);
    pb.cubic_to(
        cursor.x,
        cursor.y,
        cursor.x,
        cursor.y - tl,
        {
            cursor.x += tl;
            cursor.x
        },
        {
            cursor.y -= tl;
            cursor.y
        },
    );
    pb.line_to(
        {
            cursor.x = x + width - tr;
            cursor.x
        },
        cursor.y,
    );
    pb.cubic_to(
        cursor.x,
        cursor.y,
        cursor.x + tr,
        cursor.y,
        {
            cursor.x += tr;
            cursor.x
        },
        {
            cursor.y += tr;
            cursor.y
        },
    );
    pb.line_to(cursor.x, {
        cursor.y = y + height - br;
        cursor.y
    });
    pb.cubic_to(
        cursor.x,
        cursor.y,
        cursor.x,
        cursor.y + br,
        {
            cursor.x -= br;
            cursor.x
        },
        {
            cursor.y += br;
            cursor.y
        },
    );
    pb.line_to(
        {
            cursor.x = x + bl;
            cursor.x
        },
        cursor.y,
    );
    pb.cubic_to(
        cursor.x,
        cursor.y,
        cursor.x - bl,
        cursor.y,
        {
            cursor.x -= bl;
            cursor.x
        },
        {
            cursor.y -= bl;
            cursor.y
        },
    );
    pb.close();
    pb.finish().unwrap()
}

fn left_rect_up(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let angle = ANGLE.load(Ordering::Relaxed) as f32;
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [angle, 0., 0., angle]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn left_rect_down(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.darker().to_rgb();
    let angle = ANGLE.load(Ordering::Relaxed) as f32;
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [angle, 0., 0., angle]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn mid_rect_up(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [0., 0., 0., 0.]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn mid_rect_down(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.darker().to_rgb();
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [0., 0., 0., 0.]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn right_rect_up(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let angle = ANGLE.load(Ordering::Relaxed) as f32;
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [0., angle, angle, 0.]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn right_rect_down(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.darker().to_rgb();
    let angle = ANGLE.load(Ordering::Relaxed) as f32;
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [0., angle, angle, 0.]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

pub struct RadioGroup {
    f: group::Flex,
}

impl Default for RadioGroup {
    fn default() -> Self {
        RadioGroup::new(0, 0, 0, 0, "")
    }
}

impl RadioGroup {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        frame_cb(LEFT_RECT_UP, left_rect_up, 2, 2, 4, 4);
        frame_cb(LEFT_RECT_DOWN, left_rect_down, 2, 2, 4, 4);
        frame_cb(MID_RECT_UP, mid_rect_up, 2, 2, 4, 4);
        frame_cb(MID_RECT_DOWN, mid_rect_down, 2, 2, 4, 4);
        frame_cb(RIGHT_RECT_UP, right_rect_up, 2, 2, 4, 4);
        frame_cb(RIGHT_RECT_DOWN, right_rect_down, 2, 2, 4, 4);
        let mut f = group::Flex::new(x, y, w, h, None)
            .with_label(label)
            .with_type(group::FlexType::Row);
        f.set_pad(1);
        Self { f }
    }
    pub fn set_radius(&mut self, r: f32) {
        ANGLE.store(r as u8, Ordering::Relaxed);
    }
    pub fn radius(&self) -> f32 {
        ANGLE.load(Ordering::Relaxed) as f32
    }
    pub fn end(&self) {
        self.f.end();
        let size = self.f.children() - 1;
        for i in 0..=size {
            let child = self.f.child(i).unwrap();
            let mut child = button::RadioButton::from_dyn_widget(&child).unwrap();
            child.clear_visible_focus();
            child.set_callback(cb);
            if i == 0 {
                unsafe {
                    Fl_Widget_set_box(child.as_widget_ptr() as _, LEFT_RECT_UP);
                }
            } else if i == size {
                unsafe {
                    Fl_Widget_set_box(child.as_widget_ptr() as _, RIGHT_RECT_UP);
                }
            } else {
                unsafe {
                    Fl_Widget_set_box(child.as_widget_ptr() as _, MID_RECT_UP);
                }
            }
        }
    }
}

fn cb(w: &mut impl ButtonExt) {
    w.parent().unwrap().parent().unwrap().redraw();
}

fltk::widget_extends!(RadioGroup, group::Flex, f);
