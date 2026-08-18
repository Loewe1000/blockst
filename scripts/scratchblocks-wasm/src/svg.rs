use crate::geometry::{geometry, Geometry, Shapes};

pub fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// The notch itself, drawn from `notch_start` rightwards, then the straight
/// run to the right-hand corner.
pub fn top_notch(w: f32, y: f32) -> String {
    let r = geometry().corner_radius;
    format!(
        "c 2 0 3 1 4 2 l 4 4 c 1 1 2 2 4 2 h 12 c 2 0 3 -1 4 -2 l 4 -4 c 1 -1 2 -2 4 -2 L {} {} a {r} {r} 0 0 1 {r} {r}",
        w - r,
        y,
    )
}

pub fn get_top(w: f32) -> String {
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        return blockly_top(&g, w);
    }
    let r = g.corner_radius;
    format!("M 0 {r} A {r} {r} 0 0 1 {r} 0 H {} {}", g.notch_start, top_notch(w, 0.0))
}

pub fn get_right_and_bottom(w: f32, y: f32, has_notch: bool, inset: f32) -> String {
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        return blockly_right_and_bottom(&g, w, y, has_notch, inset);
    }
    let r = g.corner_radius;
    let mut path = format!("L {} {} a {r} {r} 0 0 1 -{r} {r}", w, y - r);
    if has_notch {
        // Drawn right to left, so it starts where the notch ends.
        path.push_str(&format!(" L {} {} c -2 0 -3 1 -4 2 l -4 4 c -1 1 -2 2 -4 2 h -12 c -2 0 -3 -1 -4 -2 l -4 -4 c -1 -1 -2 -2 -4 -2", inset + g.notch_end, y));
    }
    if inset == 0.0 {
        path.push_str(&format!(" L {} {} a {r} {r} 0 0 1 -{r} -{r}", inset + r, y));
    } else {
        path.push_str(&format!(" L {} {} a {r} {r} 0 0 0 -{r} {r}", inset + r, y));
    }
    path
}

pub fn get_hat_top(w: f32) -> String {
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        return blockly_hat_top(&g, w);
    }
    let r = g.corner_radius;
    format!("M 0 16 c 25,-22 71,-22 96,0 L {} 16 a {r} {r} 0 0 1 {r} {r}", w - r)
}

pub fn get_arm(w: f32, arm_top: f32, inset_x: f32) -> String {
    // Keep notch geometry unscaled like regular stack blocks.
    // Only shift the arm horizontally by the mouth inset.
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        return blockly_arm(&g, w, arm_top, inset_x);
    }
    let x0 = inset_x;
    let r = g.corner_radius;
    let x1 = inset_x + g.notch_start;
    format!("L {} {} a {} {} 0 0 0 {} {} L {} {} {}", x0, arm_top - r, r, r, r, r, x1, arm_top, top_notch(w, arm_top))
}


// ---------------------------------------------------------------------------
// Blockly outlines
//
// Blockly builds its shapes from four fragments, and the modern renderer
// generates them from the same constants the old one wrote out by hand — the
// notch formula reproduces the pre-2019 literal `l 6,4 3,0 6,-4` exactly. So
// there is one set of fragments here, and the profile decides the numbers.
// ---------------------------------------------------------------------------

/// The notch, drawn left to right: a ramp down, a flat run, a ramp back up.
fn blockly_notch(g: &Geometry, dir: f32) -> String {
    let ramp = (g.notch_end - g.notch_start - g.notch_inner) / 2.0;
    format!(
        "l {},{} {},0 {},-{}",
        dir * ramp,
        g.notch_depth,
        dir * g.notch_inner,
        dir * ramp,
        g.notch_depth
    )
}

/// The puzzle tab a value block plugs in with, drawn downwards along the
/// left edge (`up = false`) or upwards (`up = true`).
fn blockly_tab(g: &Geometry, up: bool) -> String {
    let forward = if up { -1.0 } else { 1.0 };
    let back = -forward;
    let w = g.tab_width;
    let half = g.tab_height / 2.0;
    format!(
        "c 0,{} {},{} {},{} s {},{} {},{}",
        forward * (half + 2.5),
        -w,
        back * (half + 0.5),
        -w,
        forward * half,
        w,
        back * 2.5,
        w,
        forward * half
    )
}

fn blockly_top(g: &Geometry, w: f32) -> String {
    let r = g.corner_radius;
    format!(
        "M 0,{r} A {r},{r} 0 0,1 {r},0 H {} {} H {} a {r},{r} 0 0,1 {r},{r}",
        g.notch_start,
        blockly_notch(g, 1.0),
        w - r
    )
}

fn blockly_hat_top(g: &Geometry, w: f32) -> String {
    let r = g.corner_radius;
    // The curve is drawn `hat_curve_height` high but only three quarters of
    // that is reserved above the block, which is how Blockly does it.
    let visual = g.hat_curve_height * 0.75;
    let span = g.hat_width.min((w - r).max(r));
    let scale = span / g.hat_width;
    format!(
        "M 0,{visual} c {},{} {},{} {span},0 H {} a {r},{r} 0 0,1 {r},{r}",
        30.0 * scale,
        -g.hat_curve_height,
        70.0 * scale,
        -g.hat_curve_height,
        w - r
    )
}

/// Right edge down to `y`, then the bottom edge leftwards to `inset`.
fn blockly_right_and_bottom(g: &Geometry, w: f32, y: f32, has_notch: bool, inset: f32) -> String {
    let r = g.corner_radius;
    let mut path = format!("L {w},{} a {r},{r} 0 0,1 -{r},{r}", y - r);
    if has_notch {
        path.push_str(&format!(" L {},{y} {}", inset + g.notch_end, blockly_notch(g, -1.0)));
    }
    if inset == 0.0 {
        path.push_str(&format!(" L {},{y} a {r},{r} 0 0,1 -{r},-{r}", inset + r));
    } else {
        // Inner wall of a C-block: the corner curves the other way.
        path.push_str(&format!(" L {},{y} a {r},{r} 0 0,0 -{r},{r}", inset + r));
    }
    path
}

/// The top edge of the lower arm of a C-block.
fn blockly_arm(g: &Geometry, w: f32, arm_top: f32, inset_x: f32) -> String {
    let r = g.corner_radius;
    format!(
        "L {inset_x},{} a {r},{r} 0 0,0 {r},{r} L {},{arm_top} {} H {} a {r},{r} 0 0,1 {r},{r}",
        arm_top - r,
        inset_x + g.notch_start,
        blockly_notch(g, 1.0),
        w - r
    )
}

/// A value block: rounded box with the puzzle tab on its left edge.
fn blockly_reporter(g: &Geometry, w: f32, h: f32) -> String {
    let r = g.corner_radius.min(h / 2.0);
    let tab_top = g.tab_offset_from_top;
    let tab_bottom = tab_top + g.tab_height;
    format!(
        "M {r},0 H {} a {r},{r} 0 0,1 {r},{r} V {} a {r},{r} 0 0,1 -{r},{r} H {r} a {r},{r} 0 0,1 -{r},-{r} V {tab_bottom} {} V {r} a {r},{r} 0 0,1 {r},-{r} Z",
        w - r,
        h - r,
        blockly_tab(g, true)
    )
}

pub fn stack_path(w: f32, h: f32) -> String {
    format!("{} {} Z", get_top(w), get_right_and_bottom(w, h, true, 0.0))
}

pub fn cap_path(w: f32, h: f32) -> String {
    format!("{} {} Z", get_top(w), get_right_and_bottom(w, h, false, 0.0))
}

pub fn hat_path(w: f32, h: f32) -> String {
    format!("{} {} Z", get_hat_top(w), get_right_and_bottom(w, h, true, 0.0))
}

pub fn proc_hat_path(w: f32, h: f32) -> String {
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        // Blockly has no separate define-hat: a procedure definition is an
        // ordinary block with a statement input.
        return format!("{} {} Z", blockly_top(&g, w), get_right_and_bottom(w, h, true, 0.0));
    }
    // Procedure definition hat: a rounded arch, wider than an ordinary corner.
    let r = g.proc_hat_radius;
    format!("M 0 {r} a {r} {r} 0 0 1 {r} -{r} L {} 0 a {r} {r} 0 0 1 {r} {r} {} Z",
        w - r,
        get_right_and_bottom(w, h, true, 0.0))
}

pub fn mouth_path(w: f32, body_h: f32, else_h: Option<f32>, header_h: f32) -> String {
    // Reference JS formula:
    //   adjusted = max(29, raw_script_height + 3) - 2
    //   arm_y    = header_height + adjusted - 3
    //   tail_y   = arm_y + tail_height + 3  (tail_height = 40 - 11 = 29)
    let k = (header_h / geometry().row_height).clamp(0.6, 1.2);
    let inset_x = 16.0 * k;
    let tail_h = 29.0 * k;
    let plus = 3.0 * k;
    let minus = 2.0 * k;
    let mut y = header_h;
    let mut path = format!("{} {}", get_top(w), get_right_and_bottom(w, y, true, inset_x));
    let adjusted_body = (body_h + plus).max(tail_h) - minus;
    y += adjusted_body - 3.0;   // arm position
    path.push_str(&format!(" {}", get_arm(w, y, inset_x)));
    if let Some(else_height) = else_h {
        y += tail_h + plus;        // tail section: tail_height + 3
        path.push_str(&format!(" {}", get_right_and_bottom(w, y, true, inset_x)));
        let adjusted_else = (else_height + plus).max(tail_h) - minus;
        y += adjusted_else - 3.0;
        path.push_str(&format!(" {}", get_arm(w, y, inset_x)));
        y += tail_h + plus;
        path.push_str(&format!(" {}", get_right_and_bottom(w, y, true, 0.0)));
    } else {
        y += tail_h + plus;        // tail bottom = arm + tail_height + 3
        path.push_str(&format!(" {}", get_right_and_bottom(w, y, true, 0.0)));
    }
    path.push_str(" Z");
    path
}

/// Mouth path for c-block-cap blocks (like forever).
/// Same as mouth_path but bottom is a cap (rounded, no notch).
pub fn mouth_cap_path(w: f32, body_h: f32, header_h: f32) -> String {
    let k = (header_h / geometry().row_height).clamp(0.6, 1.2);
    let inset_x = 16.0 * k;
    let tail_h = 29.0 * k;
    let plus = 3.0 * k;
    let minus = 2.0 * k;
    let mut y = header_h;
    let mut path = format!("{} {}", get_top(w), get_right_and_bottom(w, y, true, inset_x));
    let adjusted_body = (body_h + plus).max(tail_h) - minus;
    y += adjusted_body - 3.0;   // arm position
    path.push_str(&format!(" {}", get_arm(w, y, inset_x)));
    y += tail_h + plus;            // tail bottom
    // No notch — cap-style rounded bottom
    path.push_str(&format!(" {}", get_right_and_bottom(w, y, false, 0.0)));
    path.push_str(" Z");
    path
}

pub fn reporter_path(w: f32, h: f32) -> String {
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        return blockly_reporter(&g, w, h);
    }
    let r = h / 2.0;
    format!(
        "M {r} 0 H {right} a {r} {r} 0 0 1 {r} {r} V {bottom} a {r} {r} 0 0 1 -{r} {r} H {r} a {r} {r} 0 0 1 -{r} -{r} V {r} a {r} {r} 0 0 1 {r} -{r} Z",
        right = w - r,
        bottom = h - r,
    )
}

pub fn boolean_path(w: f32, h: f32) -> String {
    let g = geometry();
    if g.shapes == Shapes::Blockly {
        // Blockly draws no hexagon: in geras and thrasos a boolean is an
        // ordinary value block, and only the Scratch-like zelos renderer
        // gives it its own shape. Booleans plug in with the same tab.
        return blockly_reporter(&g, w, h);
    }
    let r = h / 2.0;
    format!("M {} 0 L {} 0 L {} {} L {} {} L {} {} L 0 {} L {} 0 Z", r, w-r, w, r, w-r, h, r, h, r, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{clear_geometry, set_geometry, Geometry};

    /// The Scratch outline must not move: the notch runs from 12 to 48 and
    /// the corners have radius 4.
    #[test]
    fn scratch_outline_is_unchanged() {
        clear_geometry();
        let path = stack_path(100.0, 48.0);
        assert!(path.starts_with("M 0 4 A 4 4 0 0 1 4 0 H 12"), "top: {path}");
        assert!(path.contains("L 48 48"), "bottom notch should end at 48: {path}");
    }

    /// And the measurements have to be read rather than baked in — a profile
    /// that moves the notch has to move the drawing with it. Without this the
    /// refactor would pass its own test suite while quietly ignoring the
    /// geometry it was written for.
    #[test]
    fn a_different_geometry_moves_the_outline() {
        set_geometry(Geometry {
            corner_radius: 8.0,
            notch_start: 15.0,
            notch_end: 30.0,
            ..Geometry::scratch()
        });
        let path = stack_path(100.0, 48.0);
        clear_geometry();

        assert!(path.starts_with("M 0 8 A 8 8 0 0 1 8 0 H 15"), "top: {path}");
        assert!(path.contains("L 30 48"), "bottom notch should end at 30: {path}");
    }
}

#[cfg(test)]
mod blockly_tests {
    use super::*;
    use crate::geometry::{clear_geometry, set_geometry, Geometry};

    fn with_blockly<T>(f: impl FnOnce() -> T) -> T {
        set_geometry(Geometry::blockly_modern());
        let out = f();
        clear_geometry();
        out
    }

    /// The notch is the one fragment both Blockly generations share: the
    /// modern formula reproduces the pre-2019 literal exactly, and it sits
    /// between 15 and 30 in each.
    #[test]
    fn notch_matches_the_published_fragment() {
        let path = with_blockly(|| stack_path(120.0, 24.0));
        assert!(path.contains("H 15 l 6,4 3,0 6,-4"), "top notch: {path}");
        assert!(path.contains("L 30,24 l -6,4 -3,0 -6,-4"), "bottom notch: {path}");
    }

    #[test]
    fn corners_use_the_blockly_radius() {
        let path = with_blockly(|| stack_path(120.0, 24.0));
        assert!(path.starts_with("M 0,8 A 8,8 0 0,1 8,0"), "top left: {path}");
    }

    /// A value block plugs in with a puzzle tab on its left edge, not with a
    /// Scratch pill.
    #[test]
    fn a_value_block_grows_a_puzzle_tab() {
        let path = with_blockly(|| reporter_path(60.0, 24.0));
        assert!(path.contains("V 20 c 0,-10 -8,8 -8,-7.5 s 8,2.5 8,-7.5"), "tab: {path}");
    }

    /// Booleans are not hexagons here. geras and thrasos draw them as
    /// ordinary value blocks; only the Scratch-like zelos gives them a shape
    /// of their own.
    #[test]
    fn a_boolean_is_shaped_like_any_other_value() {
        let (boolean, reporter) = with_blockly(|| (boolean_path(60.0, 24.0), reporter_path(60.0, 24.0)));
        assert_eq!(boolean, reporter);
    }

    #[test]
    fn a_hat_block_gets_the_flat_dome() {
        let path = with_blockly(|| hat_path(120.0, 24.0));
        assert!(path.starts_with("M 0,11.25 c 30,-15 70,-15 100,0"), "hat: {path}");
    }

    /// Scratch must be untouched by any of this.
    #[test]
    fn scratch_shapes_are_unaffected() {
        clear_geometry();
        let path = stack_path(120.0, 48.0);
        assert!(path.starts_with("M 0 4 A 4 4 0 0 1 4 0 H 12 c 2 0 3 1 4 2"), "{path}");
    }
}
