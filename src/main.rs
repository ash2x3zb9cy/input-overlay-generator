#![deny(missing_debug_implementations, missing_docs)]

//! CLI app to generate configs for the OBS Input Overlay plugin.

use structopt::StructOpt;
use svg::{
    node::{self, element},
    Document,
};

/// Settings to apply to the whole document.
/// This includes base key dimensions, margin, etc.
#[derive(Debug, StructOpt)]
struct Opt {
    // TODO: make keys better
    /// List of keys to generate
    #[structopt(required = true)]
    keys: Vec<String>,

    /// Amount to raise the text by.
    #[structopt(long, default_value = "2")]
    text_offset_y: usize,

    /// Width of stroke on key rectangles
    #[structopt(long, alias = "stroke_weight", default_value = "1")]
    stroke_width: usize,

    /// Base interior width of a key
    #[structopt(long, short = "w", default_value = "16")]
    key_width: usize,

    /// Base interior height of a key.
    #[structopt(long, short = "h", default_value = "16")]
    key_height: usize,

    /// Horizontal spacing between keys.
    /// The Input Overlay OBS plugin requires a value of at least 3.
    #[structopt(long, default_value = "3")]
    key_margin_x: usize,

    /// Vertical spacing between keys.
    /// The Input Overlay OBS plugin requires a value of at least 3.
    #[structopt(long, default_value = "3")]
    key_margin_y: usize,

    // TODO: implement
    // /// Exterior margin at edges of document.
    // /// The Input Overlay OBS plugin requires a value of 1.
    // #[structopt(long, default_value = "1")]
    // document_margin_x: usize,
    #[structopt(flatten)]
    down_style: DownStyle,

    #[structopt(flatten)]
    up_style: UpStyle,
}

impl Opt {
    fn bounds_width(&self) -> usize {
        self.key_width + self.stroke_width + self.key_margin_x
    }
    fn bounds_height(&self) -> usize {
        self.key_height + self.stroke_width + self.key_margin_y
    }
}

/// Style settings to apply to unpressed keys.
#[derive(Debug, StructOpt)]
struct UpStyle {
    /// Stroke colour of rectangle
    #[structopt(long, default_value = "black")]
    stroke_color_up: String,

    /// Rounding radius on rectangle's corners
    #[structopt(long, default_value = "1")]
    stroke_radius_up: usize,

    /// Fill colour of rectangle
    #[structopt(long, default_value = "none")]
    rect_color_up: String,

    /// Fill colour of text
    #[structopt(long, default_value = "black")]
    text_color_up: String,

    /// Font of text
    #[structopt(long, default_value = "monospace")]
    font_family_up: String,

    /// Font size of text
    #[structopt(long, default_value = "10")]
    font_size_up: usize,
}

/// Style settings to apply to unpressed keys.
#[derive(Debug, StructOpt)]
struct DownStyle {
    /// Stroke colour of rectangle
    #[structopt(long, default_value = "gray")]
    stroke_color_down: String,

    /// Rounding radius on rectangle's corners
    #[structopt(long, default_value = "1")]
    stroke_radius_down: usize,

    /// Fill colour of rectangle
    #[structopt(long, default_value = "lightgray")]
    rect_color_down: String,

    /// Fill colour of text
    #[structopt(long, default_value = "black")]
    text_color_down: String,

    /// Font of text
    #[structopt(long, default_value = "monospace")]
    font_family_down: String,

    /// Font size of text
    #[structopt(long, default_value = "10")]
    font_size_down: usize,
}

fn main() {
    let opt = Opt::from_args();
    let bw = opt.bounds_width();
    let bh = opt.bounds_height();

    let total_w = opt.keys.len() * bw;
    let total_h = bh * 2;

    let mut document = Document::new().set("viewBox", (0, 0, total_w, total_h));

    let rect_base = element::Rectangle::new()
        .set("width", opt.key_width)
        .set("height", opt.key_height)
        .set("stroke-width", opt.stroke_width);

    let rect_up_base = rect_base
        .clone()
        .set("y", 0)
        .set("rx", opt.up_style.stroke_radius_up)
        .set("fill", opt.up_style.rect_color_up)
        .set("stroke", opt.up_style.stroke_color_up);

    let rect_down_base = rect_base
        .set("y", bh)
        .set("rx", opt.down_style.stroke_radius_down)
        .set("fill", opt.down_style.rect_color_down)
        .set("stroke", opt.down_style.stroke_color_down);

    let text_base = element::Text::new().set("text-anchor", "middle");

    let text_up_base = text_base
        .clone()
        .set("y", opt.key_height - opt.text_offset_y)
        .set("fill", opt.up_style.text_color_up)
        .set("font-family", opt.up_style.font_family_up)
        .set("font-size", opt.up_style.font_size_up);

    let text_down_base = text_base
        .set("y", opt.key_height + bh - opt.text_offset_y)
        .set("fill", opt.down_style.text_color_down)
        .set("font-family", opt.down_style.font_family_down)
        .set("font-size", opt.down_style.font_size_down);

    for (i, v) in opt.keys.iter().enumerate() {
        let text_node = node::Text::new(v);
        let rect_x = bw * i;
        let text_x = rect_x + (opt.key_width / 2);

        let text_up = text_up_base.clone().add(text_node.clone()).set("x", text_x);
        let rect_up = rect_up_base.clone().set("x", rect_x);
        let g1 = element::Group::new().add(rect_up).add(text_up);

        let text_down = text_down_base.clone().add(text_node).set("x", text_x);
        let rect_down = rect_down_base.clone().set("x", rect_x);
        let g2 = element::Group::new().add(rect_down).add(text_down);
        document = document.add(g1).add(g2);
    }

    svg::save("image.svg", &document).unwrap();
}
