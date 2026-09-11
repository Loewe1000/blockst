//! What the parser makes of a line of NEPO source.

use nepo_wasm::{matrix, parser};
use serde_json::Value;

fn parse(source: &str) -> Value {
    let request = serde_json::json!({
        "code": source,
        "language": "de",
        "platform": "calliope",
    });
    let json = parser::parse_request(&request.to_string()).expect("parse");
    serde_json::from_str(&json).expect("json")
}

/// The first script's blocks.
fn blocks(source: &str) -> Vec<Value> {
    parse(source)[0].as_array().expect("script").clone()
}

#[test]
fn text_maps_to_the_right_block() {
    let script = blocks("Start\n  Zeige Text \"Hallo\"\n  Schalte RGB LED an (#ff0000)");
    let ids: Vec<&str> = script.iter().map(|b| b["id"].as_str().unwrap()).collect();
    assert_eq!(
        ids,
        vec![
            "robControls_start",
            "mbedActions_display_text",
            "actions_rgbLed_hidden_on_calliope"
        ]
    );
}

#[test]
fn dropdowns_keep_their_value() {
    let script = blocks("Zeige Zeichen \"A\"");
    let fields = script[0]["rows"][0]["fields"].as_array().unwrap();
    let dropdown = fields
        .iter()
        .find(|field| field["kind"] == "dropdown")
        .expect("a dropdown");
    assert_eq!(dropdown["value"], "Zeichen");
}

#[test]
fn an_unspecified_dropdown_takes_the_first_option() {
    let script = blocks("Taste B gedrückt?");
    let fields = script[0]["rows"][0]["fields"].as_array().unwrap();
    let dropdown = fields.iter().find(|f| f["kind"] == "dropdown").unwrap();
    assert_eq!(dropdown["value"], "B");
}

#[test]
fn a_sensor_block_reports_a_boolean() {
    let script = blocks("Taste A gedrückt?");
    assert_eq!(script[0]["id"], "robSensors_key_getSample");
    assert_eq!(script[0]["shape"], "value");
    assert_eq!(script[0]["check"], serde_json::json!(["Boolean"]));
}

#[test]
fn a_control_block_takes_nested_statements() {
    let script =
        blocks("Wiederhole unendlich oft\n  Zeige Text \"Hallo\"\n  Zeige Text \"Welt\"\nEnde");
    let rows = script[0]["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2, "title row plus a mouth");
    assert_eq!(rows[1]["kind"], "statement");
    let body = rows[1]["body"].as_array().unwrap();
    assert_eq!(body.len(), 2);
    assert_eq!(body[0]["id"], "mbedActions_display_text");
}

#[test]
fn nesting_survives_without_the_closing_keyword() {
    let with_end = blocks("Wiederhole unendlich oft\n  Zeige Text \"Hallo\"\nEnde");
    let without_end = blocks("Wiederhole unendlich oft\n  Zeige Text \"Hallo\"");
    assert_eq!(with_end, without_end);
}

#[test]
fn a_value_block_plugs_into_the_socket() {
    let script = blocks("Zeige Text \"Hallo\"");
    let row = &script[0]["rows"][0];
    assert_eq!(row["kind"], "value");
    assert_eq!(
        row["check"],
        serde_json::json!(["Number", "Boolean", "String"])
    );
    assert_eq!(row["value"]["id"], "text");
    let field = row["value"]["rows"][0]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["kind"] == "input")
        .expect("text input");
    assert_eq!(field["kind"], "input");
    assert_eq!(field["value"], "Hallo");
}

#[test]
fn an_empty_multi_type_socket_stays_untyped() {
    let script = blocks("Zeige Text");
    let row = &script[0]["rows"][0];
    assert_eq!(
        row["check"],
        serde_json::json!(["Number", "Boolean", "String"])
    );
    assert!(row.get("value").is_none(), "socket should be empty");
}

#[test]
fn aliases_parse_but_print_open_robertas_wording() {
    let script =
        blocks("Programmstart\n  Wiederhole fortlaufend\n    Zeige Text \"Hallo\"\n  Ende");
    assert_eq!(script[0]["id"], "robControls_start");
    // Field 0 is the plus mutator every Start block carries.
    assert_eq!(script[0]["rows"][0]["fields"][1]["value"], "Start");

    assert_eq!(script[1]["id"], "robControls_loopForever");
    assert_eq!(
        script[1]["rows"][0]["fields"][0]["value"],
        "Wiederhole unendlich oft"
    );
    assert_eq!(script[1]["rows"][1]["fields"][0]["value"], "mache");
}

#[test]
fn the_matrix_field_survives_parsing() {
    let script = blocks("Zeige Bild (.#.#.|.#.#.|.....|#...#|.###.)");
    let image = &script[0]["rows"][0]["value"];
    assert_eq!(image["id"], "mbedImage_image");
    let rows = image["rows"][0]["fields"][0]["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0], ".#.#.");
}

#[test]
fn the_matrix_expands_to_open_robertas_own_rows() {
    let scripts = parser::parse(
        "Zeige Bild (.#.#.|.#.#.|.....|#...#|.###.)",
        "de",
        "calliope",
    )
    .expect("parse");
    let mut block = scripts[0][0].clone();
    matrix::expand(&mut block);

    let image = block.rows[0].value.as_ref().expect("image block");
    assert_eq!(image.rows.len(), matrix::SIZE + 1, "a ruler plus five rows");
    for row in &image.rows {
        assert_eq!(row.align, "right");
    }
    let cells = image.rows[1]
        .fields
        .iter()
        .filter(|field| matches!(field, nepo_wasm::model::SegmentSpec::PixelCell { .. }))
        .count();
    assert_eq!(cells, matrix::SIZE);
}

#[test]
fn a_start_block_opens_a_new_script() {
    let scripts = parse("Start\n  Zeige Text \"A\"\nStart\n  Zeige Text \"B\"");
    assert_eq!(scripts.as_array().unwrap().len(), 2);
}

#[test]
fn an_unknown_line_names_the_blocks_that_do_exist() {
    let error = parser::parse("Fliege zum Mond", "de", "calliope").unwrap_err();
    assert!(error.contains("line 1"), "{error}");
    assert!(error.contains("robControls_start"), "{error}");
}

#[test]
fn a_block_the_platform_does_not_offer_is_rejected() {
    // The Calliope's built-in RGB LED is not in the micro:bit toolbox.
    assert!(parser::parse("Schalte RGB LED an (#ff0000)", "de", "calliope").is_ok());
    let error = parser::parse("Schalte RGB LED an (#ff0000)", "de", "microbit").unwrap_err();
    assert!(error.contains("microbit"), "{error}");
}

#[test]
fn comments_and_blank_lines_are_ignored() {
    let plain = blocks("Start\n  Zeige Text \"Hallo\"");
    let noisy = blocks("// ein Kommentar\nStart\n\n  Zeige Text \"Hallo\" // und noch einer\n");
    assert_eq!(plain, noisy);
}

#[test]
fn english_renders_the_english_wording() {
    let request = serde_json::json!({
        "code": "Start\n  Show Text \"Hello\"",
        "language": "en",
        "platform": "calliope",
    });
    let json = parser::parse_request(&request.to_string()).expect("parse");
    let scripts: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(scripts[0][0]["rows"][0]["fields"][1]["value"], "start");
    assert_eq!(scripts[0][1]["rows"][0]["fields"][0]["value"], "show");
}

#[test]
fn the_extended_beginner_control_set_parses_with_its_real_ids() {
    let script = blocks(
        "Wiederhole 3 mal\n  Warte ms 500\nEnde\nWarte bis Taste A gedrückt?\nwenn wahr\n  Lösche Bildschirm\nsonst\n  Schalte RGB LED aus\nEnde",
    );
    let ids: Vec<&str> = script
        .iter()
        .map(|block| block["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        vec![
            "controls_repeat_ext",
            "robControls_wait_for",
            "robControls_ifElse",
        ]
    );

    // "Wiederhole %1 mal" puts the count inside the sentence: Blockly's
    // `getInputsInline` inlines a value input that is followed by a dummy.
    let repeat_value = &script[0]["rows"][0]["fields"][1]["value"];
    assert_eq!(repeat_value["id"], "math_number");
    assert_eq!(repeat_value["rows"][0]["fields"][0]["value"], "3");
    assert_eq!(
        script[1]["rows"][0]["value"]["id"],
        "robSensors_key_getSample"
    );

    let branches = &script[2]["rows"];
    assert_eq!(branches[1]["body"][0]["id"], "mbedActions_display_clear");
    assert_eq!(
        branches[2]["body"][0]["id"],
        "actions_rgbLed_hidden_off_calliope"
    );
}

#[test]
fn boolean_literals_are_localised_reporting_blocks() {
    let de = blocks("Warte bis falsch");
    assert_eq!(de[0]["rows"][0]["value"]["id"], "logic_boolean");
    assert_eq!(
        de[0]["rows"][0]["value"]["rows"][0]["fields"][0]["value"],
        "falsch"
    );

    let request = serde_json::json!({
        "code": "wait until true",
        "language": "en",
        "platform": "calliope",
    });
    let json = parser::parse_request(&request.to_string()).expect("parse");
    let scripts: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(scripts[0][0]["rows"][0]["value"]["id"], "logic_boolean");
}

#[test]
fn inline_value_sockets_compose_math_and_logic_blocks() {
    let script = blocks(
        "Warte ms 1 + 2\nWarte bis Taste A gedrückt? und wahr\nwenn 3 ≤ 4\n  Lösche Bildschirm\nEnde\nganzzahliger Zufallswert zwischen 1 bis 10",
    );

    let arithmetic = &script[0]["rows"][0]["value"];
    assert_eq!(arithmetic["id"], "math_arithmetic");
    assert_eq!(arithmetic["rows"][0]["fields"][0]["kind"], "inline_value");
    assert_eq!(
        arithmetic["rows"][0]["fields"][0]["value"]["id"],
        "math_number"
    );
    assert_eq!(arithmetic["rows"][0]["fields"][1]["value"], "+");

    let operation = &script[1]["rows"][0]["value"];
    assert_eq!(operation["id"], "logic_operation");
    assert_eq!(
        operation["rows"][0]["fields"][0]["value"]["id"],
        "robSensors_key_getSample"
    );
    assert_eq!(script[2]["rows"][0]["value"]["id"], "logic_compare");
    assert_eq!(script[3]["id"], "math_random_int");
}

#[test]
fn beginner_sensors_and_io_blocks_keep_their_official_ids() {
    let script = blocks(
        "Pin 2 gedrückt?\ngib geschüttelt Lage\ngib Winkel ° Kompasssensor\ngib Wert % Mikrofon\ngib Wert ms Zeitgeber 1\ngib Wert ° Temperatursensor\ngib Wert % Lichtsensor\nSetze Zeitgeber 1 zurück\nSpiele Viertelnote C4\nZeige Bild Herz\nKommentar \"Notiz\"",
    );
    let ids: Vec<&str> = script
        .iter()
        .map(|block| block["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        vec![
            "robSensors_pintouch_getSample",
            "robSensors_gesture_getSample",
            "robSensors_compass_getSample",
            "robSensors_sound_getSample",
            "robSensors_timer_getSample",
            "robSensors_temperature_getSample",
            "robSensors_light_getSample",
            "mbedSensors_timer_reset",
            "mbedActions_play_note",
            "mbedActions_display_image",
            "text_comment",
        ]
    );
    let image = &script[9]["rows"][0]["value"];
    assert_eq!(image["id"], "mbedImage_get_image");
    assert_eq!(image["rows"][0]["fields"][0]["kind"], "image");
    assert_eq!(script[3]["rows"][0]["fields"][1]["kind"], "input");
    assert_eq!(script[3]["rows"][0]["fields"][1]["value"], "Wert");
}

#[test]
fn nested_conditions_from_the_visual_reference_parse_as_a_tree() {
    let script = blocks(
        "Wiederhole unendlich oft\n  wenn gib Wert % Lichtsensor < 50 und gib Wert % Lichtsensor ≤ 100\n    Zeige Text \"T\"\n  sonst\n    wenn gib Wert % Lichtsensor < 20 und gib Wert % Lichtsensor ≤ 50\n      Zeige Text \"D\"\n    sonst\n      Zeige Text \"N\"",
    );
    assert_eq!(script[0]["id"], "robControls_loopForever");
    let outer_if = &script[0]["rows"][1]["body"][0];
    assert_eq!(outer_if["id"], "robControls_ifElse");
    assert_eq!(outer_if["rows"][0]["value"]["id"], "logic_operation");
    assert_eq!(
        outer_if["rows"][0]["value"]["rows"][0]["fields"][0]["value"]["id"],
        "logic_compare"
    );
    assert_eq!(outer_if["rows"][2]["body"][0]["id"], "robControls_ifElse");
}

#[test]
fn while_loops_and_sound_mode_match_the_lesson_examples() {
    let script = blocks("Wiederhole solange Taste A gedrückt?\n  gib Geräusch % Mikrofon\nEnde");
    assert_eq!(script[0]["id"], "controls_whileUntil");
    assert_eq!(script[0]["rows"][0]["fields"][1]["value"], "solange");
    assert_eq!(
        script[0]["rows"][0]["value"]["id"],
        "robSensors_key_getSample"
    );
    let sound = &script[0]["rows"][1]["body"][0];
    assert_eq!(sound["id"], "robSensors_sound_getSample");
    assert_eq!(sound["rows"][0]["fields"][1]["value"], "Geräusch");
}

#[test]
fn variables_keep_their_dynamic_names() {
    let script = blocks("Start\n  Schreibe Punkte 0\n  Zeige Text Punkte");
    assert_eq!(script[1]["id"], "variables_set");
    assert_eq!(script[1]["rows"][0]["fields"][1]["value"], "Punkte");
    assert_eq!(script[1]["rows"][0]["value"]["id"], "math_number");
    assert_eq!(script[2]["rows"][0]["value"]["id"], "variables_get");
    assert_eq!(
        script[2]["rows"][0]["value"]["rows"][0]["fields"][0]["value"],
        "Punkte"
    );
}

#[test]
fn a_declaration_goes_into_the_start_block_and_the_program_follows_it() {
    let script = blocks(
        "Start\n  Variable Punkte : Zahl = 0\n  Variable Name : Zeichenkette = \"Ida\"\n  Zeige Text Punkte",
    );

    // The Start block keeps its title row and grows the mouth `updateShape_`
    // appends; both declarations go inside it.
    assert_eq!(script[0]["id"], "robControls_start");
    let mouth = &script[0]["rows"][1];
    assert_eq!(mouth["kind"], "statement");
    let declarations = mouth["body"].as_array().expect("declarations");
    assert_eq!(declarations.len(), 2);
    assert_eq!(declarations[0]["id"], "robGlobalVariables_declare");
    assert_eq!(declarations[0]["category"], "activity");

    // Only declarations may enter that mouth: the action block indented with
    // them is a sibling of the Start block, exactly as Open Roberta draws it.
    assert_eq!(script.len(), 2);
    assert_eq!(script[1]["id"], "mbedActions_display_text");
}

#[test]
fn a_start_block_without_declarations_keeps_its_single_row() {
    let script = blocks("Start\n  Zeige Text \"Hallo\"");
    assert_eq!(
        script[0]["rows"].as_array().expect("rows").len(),
        1,
        "the declaration mouth exists only once something declares a variable"
    );
}

#[test]
fn the_declared_type_reaches_the_socket_the_getter_and_the_setter() {
    let script = blocks(
        "Start\n  Variable Name : Zeichenkette = \"Ida\"\n  Schreibe Name\n  Zeige Text Name",
    );

    // The declaration's own socket is typed by its dropdown.
    let declaration = &script[0]["rows"][1]["body"][0];
    assert_eq!(declaration["rows"][0]["check"], serde_json::json!(["String"]));

    // The setter's empty socket announces the same type ...
    assert_eq!(script[1]["id"], "variables_set");
    assert_eq!(script[1]["rows"][0]["check"], serde_json::json!(["String"]));

    // ... and the getter reports it, which is what colours its output plug.
    let getter = &script[2]["rows"][0]["value"];
    assert_eq!(getter["id"], "variables_get");
    assert_eq!(getter["check"], serde_json::json!(["String"]));
}

#[test]
fn an_undeclared_variable_still_parses_but_stays_untyped() {
    let script = blocks("Zeige Text Punkte");
    let getter = &script[0]["rows"][0]["value"];
    assert_eq!(getter["id"], "variables_get");
    assert!(
        getter.get("check").is_none(),
        "with nothing declared there is no type to report"
    );
}

#[test]
fn increasing_a_variable_plugs_the_getter_into_a_math_block() {
    let script = blocks("erhöhe Punkte um 1");
    assert_eq!(script[0]["id"], "robMath_change");
    assert_eq!(script[0]["category"], "math");
    let fields = script[0]["rows"][0]["fields"].as_array().unwrap();
    let sockets: Vec<&Value> = fields
        .iter()
        .filter(|f| f["kind"] == "inline_value")
        .collect();
    assert_eq!(sockets.len(), 2);
    assert_eq!(sockets[0]["value"]["id"], "variables_get");
    assert_eq!(sockets[1]["value"]["id"], "math_number");
}

#[test]
fn a_list_keeps_only_the_item_rows_it_was_given() {
    let script = blocks("Liste : Zahl <- 1 2 3");
    let block = &script[0];
    assert_eq!(block["id"], "robLists_create_with");
    // Three items, three rows — the catalog's remaining item rows are the
    // ones Blockly's plus mutator never appended.
    let rows = block["rows"].as_array().expect("rows");
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[1]["align"], "right");
    // `setOutput(true, 'Array_' + listType)`.
    assert_eq!(block["check"], serde_json::json!(["Array_Number"]));
}

#[test]
fn a_list_element_takes_the_type_of_the_list_it_comes_from() {
    let script = blocks(
        "Start\n  Variable Namen : Liste Zeichenkette\n  Zeige Text von der Liste Namen nimm #tes 1",
    );
    let get_index = &script[1]["rows"][0]["value"];
    assert_eq!(get_index["id"], "robLists_getIndex");
    assert_eq!(get_index["check"], serde_json::json!(["String"]));
}

#[test]
fn a_condition_without_a_sonst_line_stays_a_plain_if() {
    // Both `robControls_if` and `robControls_ifElse` match the header line;
    // only the `sonst` line tells them apart, so the plain block has to win
    // the tie — and has to win it the same way on every run.
    for _ in 0..8 {
        let script = blocks("wenn Taste A gedrückt?\n  Lösche Bildschirm\nEnde");
        assert_eq!(script[0]["id"], "robControls_if");
        assert_eq!(script[0]["rows"].as_array().expect("rows").len(), 2);
    }

    let script = blocks("wenn Taste A gedrückt?\n  Lösche Bildschirm\nsonst\n  Zeige Text \"aus\"");
    assert_eq!(script[0]["id"], "robControls_ifElse");
    assert_eq!(script[0]["rows"][2]["body"][0]["id"], "mbedActions_display_text");
}
