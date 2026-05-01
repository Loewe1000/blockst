use crate::model::{BlockSpec, SegmentSpec};
use std::cell::RefCell;
use std::collections::HashMap;

const LABEL_MARGIN: f32 = 4.447_998;

// Thread-local widths map: when set, text_width() uses these values instead of
// hardcoded Helvetica Neue metrics. Activated by set_widths() / clear_widths().
thread_local! {
    static WIDTHS_ACTIVE: RefCell<bool> = const { RefCell::new(false) };
    static WIDTHS_MAP: RefCell<HashMap<String, f32>> = RefCell::new(HashMap::new());
    static INSET_SCALE: RefCell<f32> = const { RefCell::new(1.0) };
}

pub fn set_widths(widths: HashMap<String, f32>) {
    WIDTHS_ACTIVE.with(|a| *a.borrow_mut() = true);
    WIDTHS_MAP.with(|m| *m.borrow_mut() = widths);
}

pub fn clear_widths() {
    WIDTHS_ACTIVE.with(|a| *a.borrow_mut() = false);
    WIDTHS_MAP.with(|m| m.borrow_mut().clear());
}

pub fn set_inset_scale(scale: f32) {
    // Backward compatible:
    // - decimal factor: 0.7 -> 70%
    // - numeric percent: 70 -> 70%
    let normalized = if scale > 8.0 { scale / 100.0 } else { scale };
    let clamped = normalized.clamp(0.01, 6.0);
    INSET_SCALE.with(|s| *s.borrow_mut() = clamped);
}

pub fn clear_inset_scale() {
    INSET_SCALE.with(|s| *s.borrow_mut() = 1.0);
}

pub fn current_inset_scale() -> f32 {
    INSET_SCALE.with(|s| *s.borrow())
}

fn s(value: f32) -> f32 {
    value * current_inset_scale()
}

fn inset(base: f32, min: f32) -> f32 {
    (base * current_inset_scale()).max(min)
}

fn v(base_child_h: f32, base_padding: f32) -> f32 {
    base_child_h + (base_padding * current_inset_scale())
}

pub fn input_box_height(input: &str) -> f32 {
    match input {
        "boolean" => inset(32.0, 24.0),
        "color" => inset(32.0, 24.0),
        "dropdown" | "dropdown-field" => inset(32.0, 24.0),
        _ => inset(32.0, 24.0),
    }
}

fn is_label(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Text { .. })
}

fn is_icon(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Icon { .. })
}

fn is_dropdown(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Input { input, .. } if input == "dropdown" || input == "dropdown-field")
}

fn is_boolean(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Input { input, .. } if input == "boolean")
}

fn is_round(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Input { input, .. } if input == "number" || input == "string" || input == "color" || input == "dropdown")
        || matches!(segment, SegmentSpec::Block { block } if matches!(block.shape.as_str(), "reporter"))
}

fn horizontal_padding(block: &BlockSpec, segment: &SegmentSpec) -> f32 {
    if block.shape == "reporter" {
        if is_icon(segment) {
            s(16.0)
        } else if is_round(segment) {
            s(4.0)
        } else if is_label(segment) || is_dropdown(segment) || is_boolean(segment) {
            s(12.0)
        } else if is_round(segment) {
            s(4.0)
        } else {
            s(8.0)
        }
    } else if block.shape == "boolean" {
        if is_icon(segment) {
            s(24.0)
        } else if is_label(segment) || is_dropdown(segment) || is_round(segment) {
            s(20.0)
        } else if matches!(segment, SegmentSpec::Block { block } if block.shape == "reporter") {
            s(20.0)
        } else if matches!(segment, SegmentSpec::Block { block } if block.shape == "reporter") {
            s(24.0)
        } else if is_round(segment) {
            s(20.0)
        } else if is_boolean(segment) {
            s(8.0)
        } else {
            s(8.0)
        }
    } else {
        s(8.0)
    }
}

fn margin_between(a: &SegmentSpec, b: &SegmentSpec) -> f32 {
    if is_label(a) && is_label(b) {
        s(LABEL_MARGIN)
    } else {
        s(8.0)
    }
}

fn icon_width(name: &str) -> f32 {
    match name {
        "greenFlag" | "green-flag" | "flag" => 20.0,
        "turnLeft" | "turn-left" | "arrow-left" => 24.0,
        "turnRight" | "turn-right" | "arrow-right" => 24.0,
        "loopArrow" | "loop-arrow" => 24.0,
        _ => 20.0,
    }
}

pub fn text_width(text: &str) -> f32 {
    // Use measured widths from Typst if available
    if WIDTHS_ACTIVE.with(|a| *a.borrow()) {
        if let Some(w) = WIDTHS_MAP.with(|m| m.borrow().get(text).copied()) {
            return w;
        }
    }

    // Fall back to hardcoded Helvetica Neue metrics
    let mut width = 0.0;
    for ch in text.chars() {
        width += match ch {
            'i' | 'l' | 'I' | '|' | '!' | '.' | ',' | ':' | ';' => 4.5,
            'f' | 'j' | 't' => 5.4,
            'r' => 5.6,
            'm' | 'w' => 14.5,
            'M' | 'W' => 14.8,
            ' ' => 4.447_998,
            _ if ch.is_ascii_digit() => 8.9,
            _ if ch.is_ascii_uppercase() => 10.5,
            _ => 8.8,
        };
    }
    width
}

pub fn segment_width(segment: &SegmentSpec) -> f32 {
    match segment {
        SegmentSpec::Text { value } => text_width(value),
        SegmentSpec::Icon { name } => icon_width(name),
        SegmentSpec::Input { input, value, nested, .. } => {
            if let Some(block) = nested {
                return block_size(block).0;
            }
            match input.as_str() {
                "boolean" => inset(48.0, 34.0),
                "color" => inset(40.0, 28.0),
                "dropdown" | "dropdown-field" => {
                    // Keep a guaranteed gap between dropdown text and arrow.
                    let text_pad = inset(11.0, 7.0);
                    let arrow_w = 12.0;
                    let arrow_right = inset(12.0, 8.0);
                    let min_gap = inset(7.0, 4.0);
                    text_width(value) + text_pad + arrow_w + arrow_right + min_gap
                }
                _ => {
                    let side_pad = inset(22.0, 14.0);
                    let min_w = inset(40.0, 30.0);
                    (text_width(value) + side_pad).max(min_w)
                }
            }
        }
        SegmentSpec::Block { block } => block_size(block).0,
    }
}

pub(crate) fn line_metrics(block: &BlockSpec) -> (f32, f32, f32, f32) {
    let segments = &block.segments;
    let mut width = 0.0;
    let mut previous: Option<&SegmentSpec> = None;
    let is_notch_block = matches!(block.shape.as_str(), "stack" | "c-block" | "c-block cap" | "hat" | "define-hat" | "cap");
    let mut first_non_label_aligned = false;

    for segment in segments {
        if let Some(prev) = previous {
            width += margin_between(prev, segment);
        }

        // Scratchblocks notch alignment: align the first non-label, non-icon
        // input so its left edge clears the notch area (right of notch is ~48px).
        // cmw = 48 - horizontal_padding(block, first_segment)
        if is_notch_block && !first_non_label_aligned && !is_label(segment) && !is_icon(segment) {
            if let Some(first) = segments.first() {
                let cmw = 48.0 - horizontal_padding(block, first);
                if width < cmw {
                    width = cmw;
                }
            }
            first_non_label_aligned = true;
        }

        width += segment_width(segment);
        previous = Some(segment);
    }

    let pad_left = segments.first().map(|segment| horizontal_padding(block, segment)).unwrap_or(0.0);
    let pad_right = segments.last().map(|segment| horizontal_padding(block, segment)).unwrap_or(0.0);
    (width + pad_left + pad_right, width, pad_left, pad_right)
}

pub fn c_block_inner_width(block: &BlockSpec) -> f32 {
    let (header_inner, _, _, _) = line_metrics(block);
    header_inner.max(160.0)
}

/// Returns the maximum height of any nested block/input segment within a block's segments.
/// Used to expand the outer block height when it contains tall nested blocks.
pub fn max_nested_height(block: &BlockSpec) -> f32 {
    let mut max_h: f32 = 0.0;
    for seg in &block.segments {
        let h = match seg {
            SegmentSpec::Block { block } => block_size(block).1,
            SegmentSpec::Input { nested: Some(b), .. } => block_size(b).1,
            SegmentSpec::Input { input, nested: None, .. } => input_box_height(input),
            _ => 0.0,
        };
        if h > max_h {
            max_h = h;
        }
    }
    max_h
}

pub fn block_size(block: &BlockSpec) -> (f32, f32) {
    match block.shape.as_str() {
        "reporter" => {
            let (inner, _, _, _) = line_metrics(block);
            let nested_h = max_nested_height(block);
            let height = if nested_h > 24.0 { nested_h + s(8.0) } else { v(32.0, 8.0) };
            (inner.max(s(48.0)), height)
        }
        "boolean" => {
            let (inner, _, _, _) = line_metrics(block);
            let nested_h = max_nested_height(block);
            let height = if nested_h > 24.0 { nested_h + s(8.0) } else { v(32.0, 8.0) };
            (inner.max(s(48.0)), height)
        }
        "c-block" | "c-block cap" => c_block_size(block),
        "hat" => {
            let (inner, _, _, _) = line_metrics(block);
            let width = inner.max(s(100.0));
            let nested_h = max_nested_height(block);
            let height = if nested_h > 32.0 {
                v(32.0, 32.0) + (nested_h - 32.0)
            } else {
                v(32.0, 32.0)
            };
            (width, height)
        }
        "define-hat" => {
            // Layout: pad_left(8) + "define"(w) + margin(4.447) + remaining... + pad_right(8)
            let define_w = text_width("define");
            // Compute remaining content width (without "define" label)
            let mut remaining_w: f32 = 0.0;
            let mut prev: Option<&SegmentSpec> = None;
            for seg in &block.segments {
                if matches!(seg, SegmentSpec::Text { value } if value == "define") {
                    continue;
                }
                if let Some(p) = prev {
                    remaining_w += margin_between(p, seg);
                }
                remaining_w += segment_width(seg);
                prev = Some(seg);
            }
            // Add padding for remaining segments
            let inner_content_w = if remaining_w > 0.0 {
                let mut temp_segs: Vec<SegmentSpec> = block.segments.clone();
                temp_segs.retain(|seg| !matches!(seg, SegmentSpec::Text { value } if value == "define"));
                if let (Some(first), Some(last)) = (temp_segs.first(), temp_segs.last()) {
                    remaining_w + horizontal_padding(block, first) + horizontal_padding(block, last)
                } else {
                    remaining_w
                }
            } else {
                0.0
            };
            // Outer width = pad(8) + define + gap + inner_content + pad(8)
            let define_gap: f32 = s(8.0);
            let width = (s(8.0) + define_w + define_gap + inner_content_w + s(8.0)).max(s(100.0));
            let nested_h = max_nested_height(block);
            let base_h: f32 = v(32.0, 52.0);
            let height = if nested_h > 32.0 { base_h + (nested_h - 32.0) } else { base_h };
            (width, height)
        }
        "cap" => {
            let (inner, _, _, _) = line_metrics(block);
            let width = inner.max(s(64.0));
            let nested_h = max_nested_height(block);
            let height = if nested_h > 32.0 { nested_h + s(8.0) } else { v(32.0, 8.0) };
            (width, height)
        }
        _ => {
            let (inner, _, _, _) = line_metrics(block);
            let nested_h = max_nested_height(block);
            let height = if nested_h > 32.0 { nested_h + s(16.0) } else { v(32.0, 16.0) };
            
            // Pen blocks have an extra space for the pen icon on the left
            let pen_extra = if block.category == "pen" && !block.segments.is_empty() {
                s(32.0) // 24.0 (icon size) + 8.0
            } else {
                0.0
            };
            
            let width = (inner + pen_extra).max(s(64.0));
            (width, height)
        }
    }
}

pub fn script_size_with_inside(blocks: &[BlockSpec], inside: bool) -> (f32, f32) {
    let mut width: f32 = 0.0;
    let mut y: f32 = 1.0;
    for block in blocks {
        let (w, h) = block_size(block);
        width = width.max(w);
        y += h;
    }
    let mut height = y + 1.0;
    if !inside {
        if let Some(last) = blocks.last() {
            let has_puzzle = matches!(last.shape.as_str(), "stack" | "c-block" | "hat" | "define-hat");
            if has_puzzle {
                height += 8.0;
            }
        }
    }
    (width, height.max(1.0))
}

pub fn c_block_size(block: &BlockSpec) -> (f32, f32) {
    let inner_width = c_block_inner_width(block);
    let (body_w, body_h) = script_size_with_inside(&block.body, true);
    let script_width = body_w.max(1.0);
    let width = inner_width.max(s(16.0) + script_width);

    let has_else = !block.else_body.is_empty() || !block.else_segments.is_empty();

    // Dynamic header height: expand when header contains tall nested blocks
    let header_nested_h = max_nested_height(block);
    let header_h = if header_nested_h > 32.0 { 48.0 + (header_nested_h - 32.0) } else { 48.0 };

    let script_line_height = (body_h + s(3.0)).max(29.0) - s(2.0);
    let tail_line_height = 40.0 - 11.0;

    if !has_else {
        let height = header_h + script_line_height + tail_line_height;
        (width, height)
    } else {
        let (else_w, else_h) = script_size_with_inside(&block.else_body, true);
        let width = width.max(s(16.0) + else_w.max(1.0));
        let else_script_line_height = (else_h + s(3.0)).max(29.0) - s(2.0);
        let height = header_h + script_line_height + 29.0 + else_script_line_height + tail_line_height;
        (width, height)
    }
}
