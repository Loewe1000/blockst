use crate::measure::{block_size, c_block_inner_width, c_block_size, max_nested_height, script_size_with_inside, segment_width, text_width};
use crate::model::{BlockSpec, DocumentSpec, ScriptSpec, SegmentSpec};
use crate::svg::{boolean_path, cap_path, escape_text, hat_path, mouth_cap_path, mouth_path, proc_hat_path, reporter_path, stack_path};
use crate::theme::colors_for;

const LABEL_MARGIN: f32 = 4.447_998;

pub fn render_document(document: &DocumentSpec) -> String {
    let scale = document.scale.unwrap_or(1.0).max(0.1);
    let theme = document.theme.as_deref().unwrap_or("normal");
    let font = &document.font;

    let mut script_svgs = String::new();
    let mut total_width = 0.0f32;
    let mut total_height = 0.0f32;

    for (index, script) in document.scripts.iter().enumerate() {
        if index > 0 {
            total_height += 36.0;
        }
        let (svg, width, height) = render_script(script, theme, false);
        script_svgs.push_str(&format!("<g transform=\"translate(0 {})\">{}</g>", total_height + 1.0, svg));
        total_height += height + 1.0;
        total_width = total_width.max(width + 4.0);
    }

    if document.scripts.is_empty() {
        total_width = 1.0;
        total_height = 1.0;
    }

    let width = total_width * scale;
    let height = total_height * scale;

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\"><defs>{}</defs><g transform=\"scale({scale})\">{}</g></svg>",
        defs(theme, font),
        script_svgs,
    )
}

fn defs(theme: &str, font: &str) -> String {
    let text_fill = if theme == "high-contrast" || theme == "print" { "#000" } else { "#fff" };

    let (flag_outer, flag_inner) = match theme {
        "print" => ("#000000", "#ffffff"),
        _ => ("#45993d", "#4cbf56"),
    };
    let (arrow_outer, arrow_inner) = match theme {
        "print" => ("#000000", "#000000"),
        "high-contrast" => ("#000000", "#000000"),
        _ => ("#3d79cc", "#ffffff"),
    };
    let (loop_outer, loop_inner) = match theme {
        "print" => ("#000000", "#000000"),
        "high-contrast" => ("#000000", "#000000"),
        _ => ("#cf8b17", "#ffffff"),
    };

        let input_text_fill = if theme == "print" || theme == "high-contrast" { "#000" } else { "#575e75" };

    format!(
        "<style>.sb-label{{font:500 12pt {font};fill:{text_fill};word-spacing:1pt}}.sb-input-text{{font:500 12pt {font};fill:{input_text_fill}}}</style>\
        <g id=\"sb-greenFlag\">\
          <path d=\"M20.8 3.7c-.4-.2-.9-.1-1.2.2-2 1.6-4.8 1.6-6.8 0-2.3-1.9-5.6-2.3-8.3-1v-.4c0-.6-.5-1-1-1s-1 .4-1 1v18.8c0 .5.5 1 1 1h.1c.5 0 1-.5 1-1v-6.4c1-.7 2.1-1.2 3.4-1.3 1.2 0 2.4.4 3.4 1.2 2.9 2.3 7 2.3 9.8 0 .3-.2.4-.5.4-.9V4.7c0-.5-.3-.9-.8-1zm-.3 10.2C18 16 14.4 16 11.9 14c-1.1-.9-2.5-1.4-4-1.4-1.2.1-2.3.5-3.4 1.1V4c2.5-1.4 5.5-1.1 7.7.6 2.4 1.9 5.7 1.9 8.1 0h.2l.1.1-.1 9.2z\" fill=\"{flag_outer}\"/>\
          <path d=\"M20.6 4.8l-.1 9.1v.1c-2.5 2-6.1 2-8.6 0-1.1-.9-2.5-1.4-4-1.4-1.2.1-2.3.5-3.4 1.1V4c2.5-1.4 5.5-1.1 7.7.6 2.4 1.9 5.7 1.9 8.1 0h.2c0 .1.1.1.1.2z\" fill=\"{flag_inner}\"/>\
        </g>\
        <g id=\"sb-turnLeft\">\
          <path d=\"M20.56 18.21a10.22 10.22 0 0 1-8.1 4.22 2.26 2.26 0 0 1-.15-4.52 5.57 5.57 0 0 0 4.24-2.53 5.05 5.05 0 0 0 .55-4.62A4.25 4.25 0 0 0 15.77 9a4.29 4.29 0 0 0-2-.8 4.82 4.82 0 0 0-3.15.8l1.12 1.41A1.59 1.59 0 0 1 10.58 13H2.89a1.56 1.56 0 0 1-1.26-.63A1.54 1.54 0 0 1 1.35 11l1.72-7.43A1.59 1.59 0 0 1 4.6 2.4a1.57 1.57 0 0 1 1.24.6l1.08 1.35a10.68 10.68 0 0 1 7.72-1.68A9.88 9.88 0 0 1 19.22 4.81 9.65 9.65 0 0 1 22.05 9a10.08 10.08 0 0 1-1.49 9.21z\" fill=\"{arrow_outer}\"/>\
          <path d=\"M19.78 17.65a9.29 9.29 0 0 1-7.35 3.83 1.31 1.31 0 0 1-.08-2.62 6.52 6.52 0 0 0 5-2.92 6.04 6.04 0 0 0 .67-5.51 5.28 5.28 0 0 0-1.64-2.16 5.18 5.18 0 0 0-2.48-1 5.86 5.86 0 0 0-4.67 1.57L10.96 11a.59.59 0 0 1-.43 1H2.92a.6.6 0 0 1-.6-.75l1.71-7.42a.59.59 0 0 1 1-.21l1.67 2.1a9.71 9.71 0 0 1 7.75-2.07 8.84 8.84 0 0 1 4.12 1.92 8.67 8.67 0 0 1 2.54 3.72 9.14 9.14 0 0 1-1.33 8.36z\" fill=\"{arrow_inner}\"/>\
        </g>\
        <g id=\"sb-turnRight\">\
          <path d=\"M21.38 11.83h-7.61a.59.59 0 0 1-.43-1l1.75-2.19a5.9 5.9 0 0 0-4.7-1.58 5.07 5.07 0 0 0-4.11 3.17A6 6 0 0 0 7 15.77a6.51 6.51 0 0 0 5 2.92 1.31 1.31 0 0 1-.08 2.62 9.3 9.3 0 0 1-7.35-3.82 9.16 9.16 0 0 1-1.4-8.37A8.51 8.51 0 0 1 5.71 5.4a8.76 8.76 0 0 1 4.11-1.92 9.71 9.71 0 0 1 7.75 2.07l1.67-2.1a.59.59 0 0 1 1 .21L22 11.08a.59.59 0 0 1-.62.75z\" fill=\"{arrow_outer}\"/>\
          <path d=\"M20.56 11.83h-7.61a.59.59 0 0 1-.43-1l1.75-2.19A5.9 5.9 0 0 0 9.57 7.06a5.07 5.07 0 0 0-4.11 3.17 6 6 0 0 0 .72 5.54 6.51 6.51 0 0 0 5 2.92 1.31 1.31 0 0 1-.08 2.62 9.3 9.3 0 0 1-7.35-3.82 9.16 9.16 0 0 1-1.4-8.37A8.51 8.51 0 0 1 4.89 5.4 8.76 8.76 0 0 1 9 3.48a9.71 9.71 0 0 1 7.75 2.07l1.67-2.1a.59.59 0 0 1 1 .21l1.81 7.42a.59.59 0 0 1-.67.75z\" fill=\"{arrow_inner}\"/>\
        </g>\
        <g id=\"sb-loopArrow\">\
          <path d=\"M23.3 11c-.3.6-.9 1-1.5 1h-1.6c-.1 1.3-.5 2.5-1.1 3.6-.9 1.7-2.3 3.2-4.1 4.1-1.7.9-3.6 1.2-5.5.9-1.8-.3-3.5-1.1-4.9-2.3-.7-.7-.7-1.9 0-2.6.6-.6 1.6-.7 2.3-.2H7c.9.6 1.9.9 2.9.9s1.9-.3 2.7-.9c1.1-.8 1.8-2.1 1.8-3.5h-1.5c-.9 0-1.7-.7-1.7-1.7 0-.4.2-.9.5-1.2l4.4-4.4c.7-.6 1.7-.6 2.4 0L23 9.2c.5.5.6 1.2.3 1.8z\" fill=\"{loop_outer}\"/>\
          <path d=\"M21.8 11h-2.6c0 1.5-.3 2.9-1 4.2-.8 1.6-2.1 2.8-3.7 3.6-1.5.8-3.3 1.1-4.9.8-1.6-.2-3.2-1-4.4-2.1-.4-.3-.4-.9-.1-1.2.3-.4.9-.4 1.2-.1 1 .7 2.2 1.1 3.4 1.1s2.3-.3 3.3-1c.9-.6 1.6-1.5 2-2.6.3-.9.4-1.8.2-2.8h-2.4c-.4 0-.7-.3-.7-.7 0-.2.1-.3.2-.4l4.4-4.4c.3-.3.7-.3.9 0L22 9.8c.3.3.4.6.3.9s-.3.3-.5.3z\" fill=\"{loop_inner}\"/>\
        </g>\
        <g id=\"sb-penIcon\" stroke=\"#575E75\" fill=\"none\" stroke-linejoin=\"round\"><path d=\"M8.753 34.602l-4.251 1.779 1.784-4.236c1.218-2.892 2.907-5.423 5.03-7.538L31.066 4.93c.846-.842 2.65-.41 4.032.967 1.38 1.375 1.816 3.173.97 4.015L16.318 29.59c-2.123 2.116-4.664 3.799-7.565 5.012\" fill=\"#FFF\"/><path d=\"M29.41 6.111s-4.45-2.379-8.202 5.771c-1.734 3.766-4.35 1.546-4.35 1.546\"/><path d=\"M36.42 8.825c0 .463-.14.873-.432 1.164l-9.335 9.301c.282-.29.41-.668.41-1.12 0-.874-.507-1.963-1.406-2.868-1.362-1.358-3.147-1.8-4.002-.99L30.99 5.01c.844-.84 2.65-.41 4.035.96.898.904 1.396 1.982 1.396 2.855M10.515 33.774a23.74 23.74 0 0 1-1.764.83L4.5 36.382l1.786-4.235c.258-.604.529-1.186.833-1.757.69.183 1.449.625 2.109 1.282.659.658 1.102 1.412 1.287 2.102\" fill=\"#4C97FF\"/><path d=\"M36.498 8.748c0 .464-.141.874-.433 1.165l-19.742 19.68c-2.131 2.111-4.673 3.793-7.572 5.01L4.5 36.381l.974-2.317 1.925-.808c2.899-1.218 5.441-2.899 7.572-5.01l19.742-19.68c.292-.292.432-.702.432-1.165 0-.647-.27-1.4-.779-2.123.249.172.498.377.736.614.898.905 1.396 1.983 1.396 2.856\" fill=\"#575E75\" opacity=\".15\"/><path d=\"M18.45 12.831a.904.904 0 1 1-1.807 0 .904.904 0 0 1 1.807 0z\" fill=\"#575E75\"/></g>\
        <g id=\"sb-dropdownArrow\" transform=\"scale(0.944)\"><path d=\"M12.71 2.44A2.41 2.41 0 0 1 12 4.16L8.08 8.08a2.45 2.45 0 0 1-3.45 0L.72 4.16A2.42 2.42 0 0 1 0 2.44 2.48 2.48 0 0 1 .71.71C1 .47 1.43 0 6.36 0s5.39.46 5.64.71a2.44 2.44 0 0 1 .71 1.73z\" fill=\"#231f20\" opacity=\".1\"/><path d=\"M6.36 7.79a1.43 1.43 0 0 1-1-.42L1.42 3.45a1.44 1.44 0 0 1 0-2c.56-.56 9.31-.56 9.87 0a1.44 1.44 0 0 1 0 2L7.37 7.37a1.43 1.43 0 0 1-1.01.42z\" fill=\"#fff\"/></g>\
        <g id=\"sb-dropdownArrow-dark\" transform=\"scale(0.944)\"><path d=\"M12.71 2.44A2.41 2.41 0 0 1 12 4.16L8.08 8.08a2.45 2.45 0 0 1-3.45 0L.72 4.16A2.42 2.42 0 0 1 0 2.44 2.48 2.48 0 0 1 .71.71C1 .47 1.43 0 6.36 0s5.39.46 5.64.71a2.44 2.44 0 0 1 .71 1.73z\" fill=\"#231f20\" opacity=\".1\"/><path d=\"M6.36 7.79a1.43 1.43 0 0 1-1-.42L1.42 3.45a1.44 1.44 0 0 1 0-2c.56-.56 9.31-.56 9.87 0a1.44 1.44 0 0 1 0 2L7.37 7.37a1.43 1.43 0 0 1-1.01.42z\" fill=\"#575E75\"/></g>\
        <g id=\"sb-dropdownArrow-print\" transform=\"scale(0.944)\"><path d=\"M12.71 2.44A2.41 2.41 0 0 1 12 4.16L8.08 8.08a2.45 2.45 0 0 1-3.45 0L.72 4.16A2.42 2.42 0 0 1 0 2.44 2.48 2.48 0 0 1 .71.71C1 .47 1.43 0 6.36 0s5.39.46 5.64.71a2.44 2.44 0 0 1 .71 1.73z\" fill=\"#000\" opacity=\".1\"/><path d=\"M6.36 7.79a1.43 1.43 0 0 1-1-.42L1.42 3.45a1.44 1.44 0 0 1 0-2c.56-.56 9.31-.56 9.87 0a1.44 1.44 0 0 1 0 2L7.37 7.37a1.43 1.43 0 0 1-1.01.42z\" fill=\"#000\"/></g>\
        <g id=\"sb-penIcon-print\" stroke=\"#000\" fill=\"none\" stroke-linejoin=\"round\"><path d=\"M8.753 34.602l-4.251 1.779 1.784-4.236c1.218-2.892 2.907-5.423 5.03-7.538L31.066 4.93c.846-.842 2.65-.41 4.032.967 1.38 1.375 1.816 3.173.97 4.015L16.318 29.59c-2.123 2.116-4.664 3.799-7.565 5.012\" fill=\"#FFF\"/><path d=\"M29.41 6.111s-4.45-2.379-8.202 5.771c-1.734 3.766-4.35 1.546-4.35 1.546\"/><path d=\"M36.42 8.825c0 .463-.14.873-.432 1.164l-9.335 9.301c.282-.29.41-.668.41-1.12 0-.874-.507-1.963-1.406-2.868-1.362-1.358-3.147-1.8-4.002-.99L30.99 5.01c.844-.84 2.65-.41 4.035.96.898.904 1.396 1.982 1.396 2.855M10.515 33.774a23.74 23.74 0 0 1-1.764.83L4.5 36.382l1.786-4.235c.258-.604.529-1.186.833-1.757.69.183 1.449.625 2.109 1.282.659.658 1.102 1.412 1.287 2.102\" fill=\"#000\"/><path d=\"M36.498 8.748c0 .464-.141.874-.433 1.165l-19.742 19.68c-2.131 2.111-4.673 3.793-7.572 5.01L4.5 36.381l.974-2.317 1.925-.808c2.899-1.218 5.441-2.899 7.572-5.01l19.742-19.68c.292-.292.432-.702.432-1.165 0-.647-.27-1.4-.779-2.123.249.172.498.377.736.614.898.905 1.396 1.983 1.396 2.856\" fill=\"#000\" opacity=\".15\"/><path d=\"M18.45 12.831a.904.904 0 1 1-1.807 0 .904.904 0 0 1 1.807 0z\" fill=\"#000\"/></g>\""
    )
}

fn render_script(script: &ScriptSpec, theme: &str, inside: bool) -> (String, f32, f32) {
    let (width, height) = script_size_with_inside(&script.blocks, inside);
    let mut y = 1.0;
    let mut svg = String::new();
    for block in &script.blocks {
        let (block_svg, _, block_h) = render_block(block, theme);
        let x = if inside { 0.0 } else { 2.0 };
        svg.push_str(&format!("<g transform=\"translate({} {})\">{}</g>", x, y, block_svg));
        y += block_h;
    }
    (svg, width, height)
}

fn render_block(block: &BlockSpec, theme: &str) -> (String, f32, f32) {
    let colors = colors_for(&block.category, theme);
    // Pen blocks get a pen icon prepended
    if block.category == "pen" && !block.segments.is_empty() {
        return render_pen_block(block, theme);
    }
    match block.shape.as_str() {
        "reporter" => render_reporter_like(block, theme, colors.fill, colors.stroke, false),
        "boolean" => render_reporter_like(block, theme, colors.fill, colors.stroke, true),
        "c-block" => render_c_block(block, theme, false),
        "c-block cap" => render_c_block(block, theme, true),
        "define-hat" => render_define_hat(block, theme),
        "hat" => render_simple_block(block, theme, &hat_path),
        "cap" => render_simple_block(block, theme, &cap_path),
        _ => render_simple_block(block, theme, &stack_path),
    }
}
fn render_pen_block(block: &BlockSpec, theme: &str) -> (String, f32, f32) {
    let colors = colors_for(&block.category, theme);
    let (total_width, height) = block_size(block);
    let icon_scale = 0.6; // 24x24 inside the block
    let icon_size = 24.0;
    // Extra space: icon (scaled) + separator gap
    let pen_extra = 32.0;
    let mut svg = String::new();
    svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\"/>", stack_path(total_width, height), colors.fill, colors.stroke));
    // Draw pen icon scaled and centered vertically
    let icon_y = ((height - icon_size) / 2.0).floor();
    let pen_icon = if theme == "print" { "#sb-penIcon-print" } else { "#sb-penIcon" };
    svg.push_str(&format!("<g transform=\"translate(4 {}) scale({})\"><use href=\"{pen_icon}\"/></g>", icon_y, icon_scale));
    svg.push_str(&render_segments(block, &block.segments, theme, colors.text, pen_extra, height, 0.0));
    (svg, total_width, height)
}

fn render_define_hat(block: &BlockSpec, theme: &str) -> (String, f32, f32) {
    let colors = colors_for(&block.category, theme);
    let (width, height) = block_size(block);
    let mut svg = String::new();
    svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\"/>", proc_hat_path(width, height), colors.fill, colors.stroke));
    
    // Inner outline: 48px tall (line_height=40 + corner_radii=8)
    let content_h = 48.0;
    
    // Compute inner outline width from remaining segments (without "define")
    let define_text_width = text_width("define");
    let mut remaining: Vec<SegmentSpec> = block.segments.clone();
    remaining.retain(|seg| !matches!(seg, SegmentSpec::Text { value } if value == "define"));
    
    let inner_content_w = if remaining.is_empty() {
        0.0
    } else {
        // Measure remaining segments via block_size on a temp stack block
        let temp = BlockSpec {
            shape: "stack".to_string(),
            category: "custom-arg".to_string(),
            segments: remaining.clone(),
            body: vec![],
            else_body: vec![],
            else_segments: vec![],
        };
        let (tw, _) = block_size(&temp);
        tw
    };
    let inner_w = inner_content_w.max(100.0);
    
    // "define" text in left section, vertically centered
    let define_y = 20.0 + (48.0 - 12.0) / 2.0;
    // Position of inner outline: after pad(8) + "define" + margin
    let define_gap = 8.0;
    let inner_x = 8.0 + define_text_width + define_gap;
    
    svg.push_str(&format!("<text class=\"sb-label\" x=\"0\" y=\"13\" fill=\"{}\" transform=\"translate(10 {})\">define</text>",
        colors.text, define_y));
    
    // Inner outline starts after "define" label + gap
    svg.push_str(&format!("<g transform=\"translate({} 20)\">", inner_x));
    svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\"/>", stack_path(inner_w, content_h), colors.alt, colors.stroke));
    
    // Remaining segments inside
    if !remaining.is_empty() {
        // Use "stack" shape so inner blocks don't get reporter notch alignment
        let temp = BlockSpec {
            shape: "stack".to_string(),
            category: "custom-arg".to_string(),
            segments: remaining,
            body: vec![],
            else_body: vec![],
            else_segments: vec![],
        };
        svg.push_str(&render_segments(&temp, &temp.segments, theme, colors.text, 0.0, 40.0, 5.0));
    }
    
    svg.push_str("</g>");
    (svg, width, height)
}

fn render_simple_block(
    block: &BlockSpec,
    theme: &str,
    path_fn: &dyn Fn(f32, f32) -> String,
) -> (String, f32, f32) {
    let colors = colors_for(&block.category, theme);
    let (width, height) = block_size(block);
    let mut svg = String::new();
    svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\"/>", path_fn(width, height), colors.fill, colors.stroke));
    svg.push_str(&render_segments(block, &block.segments, theme, colors.text, 0.0, height, 0.0));
    if block.shape == "cap" {
        svg.push_str("<circle cx=\"0\" cy=\"0\" r=\"0\"/>");
    }
    (svg, width, height)
}

fn render_reporter_like(block: &BlockSpec, theme: &str, fill: &str, stroke: &str, boolean: bool) -> (String, f32, f32) {
    let (width, height) = block_size(block);
    let path = if boolean { boolean_path(width, height) } else { reporter_path(width, height) };
    let text_fill = colors_for(&block.category, theme).text;
    // Center content horizontally when block is wider than content (e.g. min-width)
    let (content_w, _, _, _) = crate::measure::line_metrics(block);
    let base_x = ((width - content_w) / 2.0).max(0.0);
    let svg = format!(
        "<path d=\"{}\" fill=\"{}\" stroke=\"{}\"/>{}",
        path,
        fill,
        stroke,
        render_segments(block, &block.segments, theme, text_fill, base_x, height, 0.0),
    );
    (svg, width, height)
}

fn render_c_block(block: &BlockSpec, theme: &str, cap: bool) -> (String, f32, f32) {
    let colors = colors_for(&block.category, theme);
    let (width, height) = c_block_size(block);
    let path_width = c_block_inner_width(block);
    let (_, body_h) = script_size_with_inside(&block.body, true);
    let has_else = !block.else_body.is_empty() || !block.else_segments.is_empty();
    let else_h = if has_else { Some(script_size_with_inside(&block.else_body, true).1) } else { None };
    let mut svg = String::new();

    // Dynamic header height: expand when header contains tall nested blocks
    let header_nested_h = max_nested_height(block);
    let header_h = if header_nested_h > 32.0 { 48.0 + (header_nested_h - 32.0) } else { 48.0 };

    if cap {
        svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\" />", mouth_cap_path(path_width, body_h.max(1.0), header_h), colors.fill, colors.stroke));
    } else {
        svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\" />", mouth_path(path_width, body_h.max(1.0), else_h, header_h), colors.fill, colors.stroke));
    }

    svg.push_str(&render_segments(block, &block.segments, theme, colors.text, 0.0, header_h, 0.0));

    svg.push_str(&render_segments(block, &block.segments, theme, colors.text, 0.0, header_h, 0.0));

    let body_y = header_h - 1.0;
    let (body_svg, _, _) = render_script(&ScriptSpec { blocks: block.body.clone() }, theme, true);
    svg.push_str(&format!("<g transform=\"translate(16 {body_y})\" >{body_svg}</g>"));

    if has_else {
        let adjusted_body = (body_h + 3.0).max(29.0) - 2.0;
        let arm_y = header_h + adjusted_body - 3.0;
        // Place else label centered in the 29px tail area (arm_y+3 to arm_y+32).
        // child_offset for text with line_height=32 gives child_y = line_y + 9.
        // SVG text y=13 is baseline, visual center ≈ baseline - 5.
        // So visual center = line_y + 9 + 13 - 5 = line_y + 17.
        // Center else text in the 29px tail area (arm_y+3 to arm_y+32)
        // Visual center of 12pt text ≈ child_y + 7.5, child_y = line_y + 9
        // So visual center = line_y + 16.5. Target = arm_y + 17.5.
        // => line_y = arm_y + 1.0
        let else_text_line_y = arm_y + 1.0;
        svg.push_str(&render_segments(block, &block.else_segments, theme, colors.text, 0.0, 32.0, else_text_line_y));

        // Place else body blocks at the top of the else section interior.
        // Analogy to the first body: body_y = header_h - 1 (i.e. notch_y - 1).
        // The dividing notch for the else interior is at tail_y = arm_y + 32,
        // so else body starts at tail_y - 1 = arm_y + 31.
        let y = arm_y + 31.0;
        let (else_svg, _, _) = render_script(&ScriptSpec { blocks: block.else_body.clone() }, theme, true);
        svg.push_str(&format!("<g transform=\"translate(16 {})\" >{else_svg}</g>", y));
    } else if block.category == "control" && !has_else {
        svg.push_str(&format!("<use href=\"#sb-loopArrow\" transform=\"translate({} {})\" />", path_width - 32.0, height - 28.0));
    }
    (svg, width, height)
}

fn render_segments(block: &BlockSpec, segments: &[SegmentSpec], theme: &str, text_fill: &str, base_x: f32, line_height: f32, line_y: f32) -> String {
    let mut x = 0.0;
    let mut svg = String::new();
    let mut previous: Option<&SegmentSpec> = None;
    let pad_left = segments.first().map(|segment| horizontal_padding(block, segment)).unwrap_or(0.0);
    let is_notch_block = matches!(block.shape.as_str(), "stack" | "c-block" | "hat" | "cap");
    let mut first_non_label_aligned = false;
    for (index, segment) in segments.iter().enumerate() {
        if let Some(prev) = previous {
            x += margin_between(prev, segment);
        }

        // Scratchblocks notch alignment: align the first non-label, non-icon
        // input so its left edge clears the notch area (right of notch is ~48px).
        if is_notch_block && !first_non_label_aligned && !matches!(segment, SegmentSpec::Text { .. }) && !matches!(segment, SegmentSpec::Icon { .. }) {
            let cmw = 48.0 - pad_left;
            if x < cmw {
                x = cmw;
            }
            first_non_label_aligned = true;
        }

        match segment {
            SegmentSpec::Text { value } => {
                let child_y = child_offset(block, segment, line_height, index == 0, line_y, index, segments.len());
                svg.push_str(&format!("<text class=\"sb-label\" x=\"{}\" y=\"13\" fill=\"{}\" transform=\"translate({} {})\">{}</text>", 0, text_fill, base_x + pad_left + x, child_y, escape_text(value)));
                x += text_width(value);
            }
            SegmentSpec::Icon { name } => {
                let child_y = child_offset(block, segment, line_height, index == 0, line_y, index, segments.len());
                if name == "flag" || name == "greenFlag" || name == "green-flag" {
                    svg.push_str(&format!("<use href=\"#sb-greenFlag\" transform=\"translate({} {})\"/>", base_x + pad_left + x, child_y));
                } else if name == "turnRight" || name == "turn-right" || name == "arrow-right" {
                    svg.push_str(&format!("<use href=\"#sb-turnRight\" transform=\"translate({} {})\"/>", base_x + pad_left + x, child_y));
                } else if name == "turnLeft" || name == "turn-left" || name == "arrow-left" {
                    svg.push_str(&format!("<use href=\"#sb-turnLeft\" transform=\"translate({} {})\"/>", base_x + pad_left + x, child_y));
                } else if name == "loopArrow" || name == "loop-arrow" {
                    svg.push_str(&format!("<use href=\"#sb-loopArrow\" transform=\"translate({} {})\"/>", base_x + pad_left + x, child_y));
                } else {
                    let icon_text = if name == "arrow-right" { "↻" } else if name == "arrow-left" { "↺" } else if name == "pen" { "✎" } else { "•" };
                    svg.push_str(&format!("<text class=\"sb-label\" x=\"0\" y=\"13\" fill=\"{}\" transform=\"translate({} {})\">{}</text>", text_fill, base_x + pad_left + x, child_y, icon_text));
                }
                x += icon_width(name);
            }
            SegmentSpec::Input { input, value, color, nested } => {
                let child_y = child_offset(block, segment, line_height, index == 0, line_y, index, segments.len());
                if let Some(block) = nested {
                    let (nested_svg, w, _) = render_block(block, theme);
                    svg.push_str(&format!("<g transform=\"translate({} {})\">{}</g>", base_x + pad_left + x, child_y, nested_svg));
                    x += w;
                } else {
                    let w = segment_width(segment);
                    let cat_colors = colors_for(&block.category, theme);
                    let is_dropdown = input == "dropdown" || input == "dropdown-field";
                    let is_round = input == "number" || input == "color" || input == "string" || input == "dropdown";
                    let is_square_dropdown = input == "dropdown-field";
                    let rx = if is_round { 16.0 } else if is_square_dropdown { 4.0 } else { 0.0 };
                    // custom-arg inputs use block fill instead of white (define-hat params)
                    let custom_fill = block.category == "custom-arg";
                    let fill = if input == "color" {
                        color.as_str()
                    } else if is_dropdown {
                        if is_square_dropdown { cat_colors.fill } else { if custom_fill { cat_colors.fill } else { cat_colors.alt } }
                    } else if custom_fill {
                        cat_colors.fill
                    } else {
                        "#ffffff"
                    };
                    let stroke = if input == "boolean" {
                        if theme == "print" || theme == "high-contrast" { cat_colors.stroke } else { "rgba(0,0,0,0.2)" }
                    } else if is_square_dropdown {
                        cat_colors.stroke
                    } else if is_dropdown {
                        cat_colors.stroke
                    } else if custom_fill {
                        cat_colors.stroke
                    } else {
                        if theme == "print" || theme == "high-contrast" { cat_colors.stroke } else { "rgba(0,0,0,0.15)" }
                    };
                    let opacity_fill = if theme == "print" || theme == "high-contrast" { "#ffffff" } else { "rgba(0,0,0,0.15)" };
                    let opacity_stroke = if theme == "print" || theme == "high-contrast" { "#000000" } else { "rgba(0,0,0,0.2)" };
                    if input == "boolean" {
                        svg.push_str(&format!("<path d=\"{}\" fill=\"{}\" stroke=\"{}\" transform=\"translate({} {})\"/>", boolean_path(w, 32.0), opacity_fill, opacity_stroke, base_x + pad_left + x, child_y));
                    } else if is_dropdown {
                        // Dropdown: rounded rect + text left-aligned + real dropdown arrow
                        let text_fill = if theme == "print" || theme == "high-contrast" { "#000000" } else { cat_colors.text };
                        let dropdown_fill = if theme == "print" && is_round {
                            "#ffffff"
                        } else if theme == "print" {
                            "#ffffff"
                        } else {
                            fill
                        };
                        svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"32\" rx=\"{}\" ry=\"{}\" fill=\"{}\" stroke=\"{}\"/>",
                            base_x + pad_left + x, child_y, w, rx, rx, dropdown_fill, stroke));
                        // Text position per scratchblocks scratch3: px=11 for dropdowns
                        let text_pad = 11.0;
                        let text_x = base_x + pad_left + x + text_pad;
                        svg.push_str(&format!("<text class=\"sb-input-text\" style=\"fill:{}\" x=\"{}\" y=\"{}\">{}</text>",
                            text_fill, text_x, child_y + 21.0, escape_text(value)));
                        // Arrow per scratchblocks scratch3: at w-24, so it sits ~7px after text end
                        let arrow_x = base_x + pad_left + x + w - 24.0;
                        let arrow_y = child_y + 11.5;
                        // In print theme, always use black arrow
                        let arrow_id = if theme == "print" {
                            "#sb-dropdownArrow-print"
                        } else if text_fill == "#ffffff" {
                            "#sb-dropdownArrow"
                        } else {
                            "#sb-dropdownArrow-dark"
                        };
                        svg.push_str(&format!("<use href=\"{}\" transform=\"translate({} {})\"/>", arrow_id, arrow_x, arrow_y));
                    } else if input == "color" {
                        // Color swatch (always use theme-normal stroke for color swatches)
                        let swatch_stroke = if theme == "print" { "#000000" } else { stroke };
                        svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"32\" rx=\"{}\" ry=\"{}\" fill=\"{}\" stroke=\"{}\"/>",
                            base_x + pad_left + x, child_y, w, rx, rx, fill, swatch_stroke));
                    } else {
                        // Number/string input: use black stroke in print theme
                        let num_stroke = if theme == "print" { "#000000" } else { stroke };
                        svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"32\" rx=\"{}\" ry=\"{}\" fill=\"{}\" stroke=\"{}\"/>", base_x + pad_left + x, child_y, w, rx, rx, fill, num_stroke));
                        if !value.is_empty() {
                            let text_cx = base_x + pad_left + x + (w - text_width(value)) / 2.0;
                            if custom_fill {
                                // Pink background → white text
                                svg.push_str(&format!("<text class=\"sb-input-text\" style=\"fill:#ffffff\" x=\"{}\" y=\"{}\">{}</text>", text_cx, child_y + 22.0, escape_text(value)));
                            } else {
                                svg.push_str(&format!("<text class=\"sb-input-text\" x=\"{}\" y=\"{}\">{}</text>", text_cx, child_y + 22.0, escape_text(value)));
                            }
                        }
                    }
                    x += w;
                }
            }
            SegmentSpec::Block { block } => {
                let child_y = child_offset(block, segment, line_height, index == 0, line_y, index, segments.len());
                let (nested_svg, w, _) = render_block(block, theme);
                svg.push_str(&format!("<g transform=\"translate({} {})\">{}</g>", base_x + pad_left + x, child_y, nested_svg));
                x += w;
            }
        }
        previous = Some(segment);
    }
    svg
}

fn child_offset(parent: &BlockSpec, segment: &SegmentSpec, line_height: f32, first_line: bool, line_y: f32, index: usize, _len: usize) -> f32 {
    let (pt, pb) = if parent.shape == "hat" { (24.0, 8.0) } else { (4.0, 4.0) };
    let child_height = match segment {
        SegmentSpec::Text { .. } => 12.0,
        SegmentSpec::Icon { name } => icon_height(name),
        SegmentSpec::Input { nested, .. } => nested.as_ref().map(|block| block_size(block).1).unwrap_or(32.0),
        SegmentSpec::Block { block } => block_size(block).1,
    };
    let mut y = pt + (line_height - child_height - pt - pb) / 2.0;
    if matches!(segment, SegmentSpec::Text { .. }) {
        y -= 1.0;
    } else if let SegmentSpec::Icon { name } = segment {
        y += icon_dy(name);
        if matches!(parent.shape.as_str(), "stack" | "c-block" | "hat" | "cap") && first_line && index == 0 {
            y += 4.0;
        }
    }
    (line_y + y).floor()
}

fn horizontal_padding(block: &BlockSpec, segment: &SegmentSpec) -> f32 {
    if block.shape == "reporter" {
        if matches!(segment, SegmentSpec::Icon { .. }) {
            16.0
        } else if is_round(segment) {
            4.0
        } else if matches!(segment, SegmentSpec::Text { .. }) || is_dropdown(segment) || is_boolean(segment) {
            12.0
        } else if is_round(segment) {
            4.0
        } else {
            8.0
        }
    } else if block.shape == "boolean" {
        if matches!(segment, SegmentSpec::Icon { .. }) {
            24.0
        } else if matches!(segment, SegmentSpec::Text { .. }) || is_dropdown(segment) || is_round(segment) {
            20.0
        } else if matches!(segment, SegmentSpec::Block { block } if block.shape == "reporter") {
            24.0
        } else if is_round(segment) {
            20.0
        } else if is_boolean(segment) {
            8.0
        } else {
            8.0
        }
    } else {
        8.0
    }
}

fn margin_between(a: &SegmentSpec, b: &SegmentSpec) -> f32 {
    if matches!(a, SegmentSpec::Text { .. }) && matches!(b, SegmentSpec::Text { .. }) {
        LABEL_MARGIN
    } else {
        8.0
    }
}

fn is_dropdown(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Input { input, .. } if input == "dropdown" || input == "dropdown-field")
}

fn is_boolean(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Input { input, .. } if input == "boolean")
}

fn is_round(segment: &SegmentSpec) -> bool {
    matches!(segment, SegmentSpec::Input { input, .. } if input == "number" || input == "string" || input == "color" || input == "dropdown")
        || matches!(segment, SegmentSpec::Block { block } if block.shape == "reporter")
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

fn icon_height(name: &str) -> f32 {
    match name {
        "greenFlag" | "green-flag" | "flag" => 21.0,
        "turnLeft" | "turn-left" | "arrow-left" => 24.0,
        "turnRight" | "turn-right" | "arrow-right" => 24.0,
        "loopArrow" | "loop-arrow" => 24.0,
        _ => 20.0,
    }
}

fn icon_dy(name: &str) -> f32 {
    match name {
        "greenFlag" | "green-flag" | "flag" => -2.0,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{BlockSpec, SegmentSpec};

    #[test]
    fn test_render_c_block_with_else() {
        // Build a simple if-else block spec
        let block = BlockSpec {
            shape: "c-block".to_string(),
            category: "control".to_string(),
            segments: vec![
                SegmentSpec::Text { value: "falls".to_string() },
                SegmentSpec::Input { input: "boolean".to_string(), value: "".to_string(), color: "".to_string(), nested: None },
                SegmentSpec::Text { value: "dann".to_string() },
            ],
            body: vec![
                BlockSpec {
                    shape: "stack".to_string(),
                    category: "looks".to_string(),
                    segments: vec![SegmentSpec::Text { value: "sagen".to_string() }, SegmentSpec::Input { input: "string".to_string(), value: "Hallo".to_string(), color: "".to_string(), nested: None }],
                    body: vec![],
                    else_body: vec![],
                    else_segments: vec![],
                }
            ],
            else_body: vec![
                BlockSpec {
                    shape: "stack".to_string(),
                    category: "looks".to_string(),
                    segments: vec![SegmentSpec::Text { value: "sagen".to_string() }, SegmentSpec::Input { input: "string".to_string(), value: "Tschüss".to_string(), color: "".to_string(), nested: None }],
                    body: vec![],
                    else_body: vec![],
                    else_segments: vec![],
                }
            ],
            else_segments: vec![SegmentSpec::Text { value: "sonst".to_string() }],
        };
        let (svg, width, height) = render_c_block(&block, "normal", false);
        // Check that the else segment appears in SVG
        assert!(svg.contains("sonst"), "SVG should contain 'sonst' text");
        // Check that the else body appears (maybe via transform)
        assert!(svg.contains("Tschüss"), "SVG should contain else body text");
        // Debug output
        println!("SVG width={} height={}", width, height);
        println!("{}", svg);
    }
}
