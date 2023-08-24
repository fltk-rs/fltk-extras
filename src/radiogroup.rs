use crate::styles::colors::*;
use fltk::{enums::*, prelude::*, *};
use tiny_skia::{
    FillRule, LineCap, Paint, Path, PathBuilder, Pixmap, Stroke, Transform,
};

struct Position {
    x: f32,
    y: f32,
}

// copied with some modification from: https://gitlab.com/snakedye/snui/-/blob/master/snui/src/widgets/shapes/rectangle.rs?ref_type=heads
pub fn rounded_rect(mut pb: PathBuilder, width: f32, height: f32, radius: [f32; 4]) -> Path {
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

const LEFT_RECT_UP: i32 = 100;
const LEFT_RECT_DOWN: i32 = 101;
const MID_RECT_UP: i32 = 102;
const MID_RECT_DOWN: i32 = 103;
const RIGHT_RECT_UP: i32 = 104;
const RIGHT_RECT_DOWN: i32 = 105;

fn left_rect_up(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [15., 0., 0., 15.]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    let mut stroke = Stroke::default();
    stroke.width = 2.0;
    stroke.line_cap = LineCap::Round;
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn left_rect_down(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [15., 0., 0., 15.]);
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
    let mut stroke = Stroke::default();
    stroke.width = 2.0;
    stroke.line_cap = LineCap::Round;
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn mid_rect_down(x: i32, y: i32, w: i32, h: i32, _c: Color) {
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

fn right_rect_up(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [0., 15., 15., 0.]);
    let mut pixmap = Pixmap::new(w as _, h as _).unwrap();
    let mut stroke = Stroke::default();
    stroke.width = 2.0;
    stroke.line_cap = LineCap::Round;
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    let mut img = image::RgbImage::new(pixmap.data(), w, h, ColorDepth::Rgba8).unwrap();
    img.draw(x, y, w, h);
}

fn right_rect_down(x: i32, y: i32, w: i32, h: i32, _c: Color) {
    if w < 1 || h < 1 {
        return;
    }
    let c = SEL_BLUE;
    let (r, g, b) = c.to_rgb();
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    let pb = PathBuilder::new();
    let path = rounded_rect(pb, w as _, h as _, [0., 15., 15., 0.]);
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
        app::set_frame_type_cb(
            unsafe { std::mem::transmute(LEFT_RECT_UP) },
            left_rect_up,
            2,
            2,
            4,
            4,
        );
        app::set_frame_type_cb(
            unsafe { std::mem::transmute(LEFT_RECT_DOWN) },
            left_rect_down,
            2,
            2,
            4,
            4,
        );
        app::set_frame_type_cb(
            unsafe { std::mem::transmute(MID_RECT_UP) },
            mid_rect_up,
            2,
            2,
            4,
            4,
        );
        app::set_frame_type_cb(
            unsafe { std::mem::transmute(MID_RECT_DOWN) },
            mid_rect_down,
            2,
            2,
            4,
            4,
        );
        app::set_frame_type_cb(
            unsafe { std::mem::transmute(RIGHT_RECT_UP) },
            right_rect_up,
            2,
            2,
            4,
            4,
        );
        app::set_frame_type_cb(
            unsafe { std::mem::transmute(RIGHT_RECT_DOWN) },
            right_rect_down,
            2,
            2,
            4,
            4,
        );
        let f = group::Flex::new(x, y, w, h, None)
            .with_label(label)
            .with_type(group::FlexType::Row);
        Self { f }
    }
    pub fn end(&self) {
        self.f.end();
        let size = self.f.children() - 1;
        for i in 0..=size {
            let child = self.f.child(i).unwrap();
            let mut child = button::RadioButton::from_dyn_widget(&child).unwrap();
            child.set_callback(cb);
            child.clear_visible_focus();
            if i == 0 {
                child.set_frame(unsafe { std::mem::transmute(LEFT_RECT_UP) });
            } else if i == size {
                child.set_frame(unsafe { std::mem::transmute(RIGHT_RECT_UP) });
            } else {
                child.set_frame(unsafe { std::mem::transmute(MID_RECT_UP) })
            }
        }
    }
}

fn cb(w: &mut impl ButtonExt) {
    w.window().unwrap().redraw();
}

fltk::widget_extends!(RadioGroup, group::Flex, f);
