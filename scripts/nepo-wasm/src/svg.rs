//! Blockly path geometry, transcribed from Open Roberta's renderer.
//!
//! Every constant and every path fragment below is copied verbatim from
//! OpenRoberta/blockly `core/block_render_svg.js` and `core/field*.js`. The
//! point of the prototype is to find out whether Blockst can produce the real
//! NEPO outline, so nothing here is re-derived by eye — a NEPO theme laid over
//! Blockst's Scratch geometry would only ever look approximately right.

// -- Blockly.BlockSvg.* ---------------------------------------------------

pub const SEP_SPACE_X: f32 = 10.0;
pub const SEP_SPACE_Y: f32 = 10.0;
pub const INLINE_PADDING_Y: f32 = 5.0;
pub const MIN_BLOCK_Y: f32 = 25.0;
pub const TAB_HEIGHT: f32 = 20.0;
pub const TAB_WIDTH: f32 = 8.0;
pub const NOTCH_WIDTH: f32 = 30.0;
pub const CORNER_RADIUS: f32 = 2.0;

/// Height of the protruding bottom notch, added to a block's own height when
/// it has a next connection (`renderDrawBottom_`: `this.height += 4`).
pub const NOTCH_DEPTH: f32 = 4.0;

// -- Blockly.Field.* ------------------------------------------------------

/// `Blockly.Field.prototype.size_ = new goog.math.Size(0, 25)`
pub const FIELD_HEIGHT: f32 = 25.0;
/// The white rounded box behind an editable field.
pub const FIELD_BOX_HEIGHT: f32 = 16.0;
pub const FIELD_BOX_RADIUS: f32 = 2.0;
/// `'x': -Blockly.BlockSvg.SEP_SPACE_X / 2`
pub const FIELD_BOX_X: f32 = -SEP_SPACE_X / 2.0;
/// `'width': field width + Blockly.BlockSvg.SEP_SPACE_X`
pub const FIELD_BOX_PAD: f32 = SEP_SPACE_X;
/// The editable colour swatch has a fixed visual width. Open Roberta gives it
/// more room than an empty text field, so it remains legible as a standalone
/// Colour reporter when plugged into an RGB-LED action.
pub const COLOUR_FIELD_WIDTH: f32 = 22.0;
/// `textElement_` carries `'y': this.size_.height - 12.5`
pub const FIELD_TEXT_BASELINE: f32 = FIELD_HEIGHT - 12.5;
/// `Blockly.FieldPixelbox` fixes its own width at 16.
pub const PIXEL_BOX_SIZE: f32 = 16.0;

/// `new Blockly.FieldDropdownImage(options, '/dropDowns/', 24, 24, 'png')` in
/// `blocks/mbedImage.js`: the picture itself is 24 by 24.
pub const IMAGE_FIELD_PICTURE: f32 = 24.0;
/// `FieldDropdownImage` sets `size_ = (width + 10, height + 2 *
/// INLINE_PADDING_Y)`. The extra ten pixels are where the dropdown arrow
/// goes; the extra ten in height are why a block holding one of these
/// pictures is taller than every other block — the picture needs five pixels
/// of air above and below, and `renderCompute_` takes the field's own height
/// as the row's minimum.
pub const IMAGE_FIELD_WIDTH: f32 = IMAGE_FIELD_PICTURE + 10.0;
pub const IMAGE_FIELD_HEIGHT: f32 = IMAGE_FIELD_PICTURE + 2.0 * INLINE_PADDING_Y;
/// `textElement_` is placed at `x = size_.width - 6`, `y = size_.height / 2`.
pub const IMAGE_FIELD_ARROW_X: f32 = IMAGE_FIELD_WIDTH - 6.0;
pub const IMAGE_FIELD_ARROW_Y: f32 = IMAGE_FIELD_HEIGHT / 2.0;

/// `.blocklyText { font-size: 11pt }`, converted to SVG user units (px).
pub const FONT_SIZE_PT: f32 = 11.0;
pub const FONT_SIZE_PX: f32 = FONT_SIZE_PT * 96.0 / 72.0;

// -- Path fragments -------------------------------------------------------

pub const NOTCH_PATH_LEFT: &str = "h 2.5l 5, 5 5, -5 h 2.5";
pub const NOTCH_PATH_RIGHT: &str = "h -3.5l -4, 4 -4, -4 h -3.5";

/// The socket cut into a block's right edge for a value input.
pub const TAB_PATH_DOWN_INNER: &str = "v 4.1 l-5.154 -0.469c0 0 -1.096 3.125 -1.33 4.656s-0.25 3.703 0.406 5.078s 1.984 2.688 3.484 2.953s2.375 0.203 2.594 0.219 v 3.5";

/// The plug on a value block's left edge.
pub const TAB_PATH_DOWN_OUTER: &str = "v -5 c-4.217 0.097 -4.471 -0.54 -5.281 -1.609c-1.109 -1.46 -1.159 -3.82 -0.159 -7.12l0.33 -1.061l5.11 0.44 v -5.2";

/// `Blockly.BlockSvg.prototype.drawInputType_` traces this over an *empty*
/// socket in the colour of the type it accepts. It is the visual grammar that
/// tells a NEPO user a string goes here and a number does not.
pub const INPUT_TYPE_PATH: &str = "l-6.188 -0.547c0 0 -1.781 4.781 -1.938 7.281s0.578 4.594 0.984 5.281s1.719 2.234 3.016 2.797s3.172 0.641 4.125 0.641";
pub const INPUT_TYPE_STROKE_WIDTH: &str = "3";

pub fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn num(value: f32) -> String {
    // Keep the paths readable and stable across platforms: three decimals is
    // well under a device pixel at any sane scale.
    let rounded = (value * 1000.0).round() / 1000.0;
    if rounded == rounded.trunc() {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded}")
    }
}

/// `renderDrawTop_`
pub fn draw_top(
    steps: &mut Vec<String>,
    right_edge: f32,
    square_top_left: bool,
    has_previous: bool,
) {
    if square_top_left {
        steps.push("m 0,0".to_string());
    } else {
        steps.push(format!("m 0,{}", num(CORNER_RADIUS)));
        steps.push(format!(
            "A {},{} 0 0,1 {},0",
            num(CORNER_RADIUS),
            num(CORNER_RADIUS),
            num(CORNER_RADIUS)
        ));
    }
    if has_previous {
        steps.push(format!("H {}", num(NOTCH_WIDTH - 15.0)));
        steps.push(NOTCH_PATH_LEFT.to_string());
    }
    steps.push(format!("H {}", num(right_edge)));
}

/// `Blockly.BlockSvg.INNER_TOP_LEFT_CORNER`
pub fn inner_top_left_corner() -> String {
    format!(
        "{} h -{} a {},{} 0 0,0 -{},{}",
        NOTCH_PATH_RIGHT,
        num(NOTCH_WIDTH - 15.0 + 1.0 - CORNER_RADIUS),
        num(CORNER_RADIUS),
        num(CORNER_RADIUS),
        num(CORNER_RADIUS),
        num(CORNER_RADIUS)
    )
}

/// `Blockly.BlockSvg.INNER_BOTTOM_LEFT_CORNER`
pub fn inner_bottom_left_corner() -> String {
    format!(
        "a {},{} 0 0,0 {},{}",
        num(CORNER_RADIUS),
        num(CORNER_RADIUS),
        num(CORNER_RADIUS),
        num(CORNER_RADIUS)
    )
}

/// `renderDrawBottom_`
pub fn draw_bottom(steps: &mut Vec<String>, square_bottom_left: bool, has_next: bool) {
    if has_next {
        steps.push(format!("H {} {}", num(NOTCH_WIDTH), NOTCH_PATH_RIGHT));
    }
    if square_bottom_left {
        steps.push("H 0".to_string());
    } else {
        steps.push(format!("H {}", num(CORNER_RADIUS)));
        steps.push(format!(
            "a {},{} 0 0,1 -{},-{}",
            num(CORNER_RADIUS),
            num(CORNER_RADIUS),
            num(CORNER_RADIUS),
            num(CORNER_RADIUS)
        ));
    }
}

/// `renderDrawLeft_`
pub fn draw_left(steps: &mut Vec<String>, has_output: bool) {
    if has_output {
        steps.push(format!("V {}", num(TAB_HEIGHT)));
        steps.push(TAB_PATH_DOWN_OUTER.to_string());
    }
    steps.push("z".to_string());
}

pub fn vertical(steps: &mut Vec<String>, distance: f32) {
    steps.push(format!("v {}", num(distance)));
}

pub fn horizontal_to(steps: &mut Vec<String>, x: f32) {
    steps.push(format!("H {}", num(x)));
}

pub fn fmt(value: f32) -> String {
    num(value)
}
