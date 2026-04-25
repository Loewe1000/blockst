pub fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn top_notch(w: f32, y: f32) -> String {
    format!(
        "c 2 0 3 1 4 2 l 4 4 c 1 1 2 2 4 2 h 12 c 2 0 3 -1 4 -2 l 4 -4 c 1 -1 2 -2 4 -2 L {} {} a 4 4 0 0 1 4 4",
        w - 4.0,
        y,
    )
}

pub fn get_top(w: f32) -> String {
    format!("M 0 4 A 4 4 0 0 1 4 0 H 12 {}", top_notch(w, 0.0))
}

pub fn get_right_and_bottom(w: f32, y: f32, has_notch: bool, inset: f32) -> String {
    let mut path = format!("L {} {} a 4 4 0 0 1 -4 4", w, y - 4.0);
    if has_notch {
        path.push_str(&format!(" L {} {} c -2 0 -3 1 -4 2 l -4 4 c -1 1 -2 2 -4 2 h -12 c -2 0 -3 -1 -4 -2 l -4 -4 c -1 -1 -2 -2 -4 -2", inset + 48.0, y));
    }
    if inset == 0.0 {
        path.push_str(&format!(" L {} {} a 4 4 0 0 1 -4 -4", inset + 4.0, y));
    } else {
        path.push_str(&format!(" L {} {} a 4 4 0 0 0 -4 4", inset + 4.0, y));
    }
    path
}

pub fn get_hat_top(w: f32) -> String {
    format!("M 0 16 c 25,-22 71,-22 96,0 L {} 16 a 4 4 0 0 1 4 4", w - 4.0)
}

pub fn get_arm(w: f32, arm_top: f32) -> String {
    format!("L 16 {} a 4 4 0 0 0 4 4 L 28 {} {}", arm_top - 4.0, arm_top, top_notch(w, arm_top))
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
    // Procedure definition hat: rounded arch top (20px radius)
    format!("M 0 20 a 20 20 0 0 1 20 -20 L {} 0 a 20 20 0 0 1 20 20 {} Z",
        w - 20.0,
        get_right_and_bottom(w, h, true, 0.0))
}

pub fn mouth_path(w: f32, body_h: f32, else_h: Option<f32>, header_h: f32) -> String {
    // Reference JS formula:
    //   adjusted = max(29, raw_script_height + 3) - 2
    //   arm_y    = header_height + adjusted - 3
    //   tail_y   = arm_y + tail_height + 3  (tail_height = 40 - 11 = 29)
    let mut y = header_h;
    let mut path = format!("{} {}", get_top(w), get_right_and_bottom(w, y, true, 16.0));
    let adjusted_body = (body_h + 3.0).max(29.0) - 2.0;
    y += adjusted_body - 3.0;   // arm position
    path.push_str(&format!(" {}", get_arm(w, y)));
    if let Some(else_height) = else_h {
        y += 29.0 + 3.0;        // tail section: tail_height + 3
        path.push_str(&format!(" {}", get_right_and_bottom(w, y, true, 16.0)));
        let adjusted_else = (else_height + 3.0).max(29.0) - 2.0;
        y += adjusted_else - 3.0;
        path.push_str(&format!(" {}", get_arm(w, y)));
        y += 29.0 + 3.0;
        path.push_str(&format!(" {}", get_right_and_bottom(w, y, true, 0.0)));
    } else {
        y += 29.0 + 3.0;        // tail bottom = arm + tail_height + 3
        path.push_str(&format!(" {}", get_right_and_bottom(w, y, true, 0.0)));
    }
    path.push_str(" Z");
    path
}

/// Mouth path for c-block-cap blocks (like forever).
/// Same as mouth_path but bottom is a cap (rounded, no notch).
pub fn mouth_cap_path(w: f32, body_h: f32, header_h: f32) -> String {
    let mut y = header_h;
    let mut path = format!("{} {}", get_top(w), get_right_and_bottom(w, y, true, 16.0));
    let adjusted_body = (body_h + 3.0).max(29.0) - 2.0;
    y += adjusted_body - 3.0;   // arm position
    path.push_str(&format!(" {}", get_arm(w, y)));
    y += 29.0 + 3.0;            // tail bottom
    // No notch — cap-style rounded bottom
    path.push_str(&format!(" {}", get_right_and_bottom(w, y, false, 0.0)));
    path.push_str(" Z");
    path
}

pub fn reporter_path(w: f32, h: f32) -> String {
    let r = h / 2.0;
    format!(
        "M {r} 0 H {right} a {r} {r} 0 0 1 {r} {r} V {bottom} a {r} {r} 0 0 1 -{r} {r} H {r} a {r} {r} 0 0 1 -{r} -{r} V {r} a {r} {r} 0 0 1 {r} -{r} Z",
        right = w - r,
        bottom = h - r,
    )
}

pub fn boolean_path(w: f32, h: f32) -> String {
    let r = h / 2.0;
    format!("M {} 0 L {} 0 L {} {} L {} {} L {} {} L 0 {} L {} 0 Z", r, w-r, w, r, w-r, h, r, h, r, r)
}
