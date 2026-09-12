# Generated block data — what came out

## Languages

German is the reference locale (full tables); English is the base the other languages inherit from, each of them a specs-only locale built from Blockly's msg/json file. `gaps` are standard blocks whose message that language does not translate — they fall back to English.

| Language | blocks | gaps |
| --- | ---: | ---: |
| en | 40 | 0 |
| ar | 40 | 0 |
| ca | 40 | 0 |
| cs | 39 | 1 |
| cy | — | no msg/json file |
| el | 40 | 0 |
| es | 40 | 0 |
| fa | 40 | 0 |
| fr | 40 | 0 |
| gd | — | no msg/json file |
| he | 40 | 0 |
| hi | 40 | 0 |
| hr | 40 | 0 |
| hu | 39 | 0 |
| id | 40 | 0 |
| it | 40 | 0 |
| ja | 40 | 0 |
| nb | 40 | 0 |
| nl | 40 | 0 |
| pl | 40 | 0 |
| pt | 40 | 0 |
| ro | 40 | 0 |
| ru | 40 | 0 |
| sl | 40 | 0 |
| tr | 40 | 0 |

Regenerate with `python3 scripts/blockly-data/generate.py`.

## Counts

| Set | Blocks |
| --- | ---: |
| Blockly standard (German) | 40 |
| jwinf robot world | 38 |
| jwinf turtle world | 42 |
| standard blocks worded differently in the old Blockly | 4 |
| world blocks dropped as socket-twins of a field variant | 23 |

## Shapes

| Shape | Blockly | jwinf worlds |
| --- | ---: | ---: |
| boolean | 12 | 0 |
| c-block | 7 | 0 |
| cap | 1 | 0 |
| reporter | 16 | 22 |
| stack | 4 | 56 |

## Gaps

- 48 world blocks without a German label — these need translating by hand: connect, containerSize, dropAbove, dropBridgeInFront, dropInFront, dropNum, dropNum_noShadow, dropPlatformAbove, dropPlatformInFront, gridEdgeNorth, gridEdgeSouth, jump, jumpAmount, nbInBag, nbWithdrawables, onBlack, onBlue, onBottomArrow, onChocolate, onCircle, onClubs, onCross, onDiamonds, onDotted, onFemale, onGreen, onHearts, onHole, onLeftArrow, onMale, onOneShape, onPaint, onQuadrille, onRightArrow, onRound, onSpades, onSquare, onStar, onStriped, onTopArrow, onTriangle, onTwoShapes, onYellow, wait, waterInFront, withdrawNum, withdrawNum_noShadow, writeCode
- 14 world blocks whose German and English labels are identical, so one of the two upstream tables is wrong — checked by hand, this goes both ways (`row` is English in the German table, `turnleftamountvalue_options` is German in the English one): alert, col, colour2, inputvalue, log, peneither, row, turn, turnleftamountvalue_moreoptions, turnleftamountvalue_noround, turnleftamountvalue_options, turnrightamountvalue_moreoptions, turnrightamountvalue_noround, turnrightamountvalue_options

## Wording that differs between old and current Blockly

The old wording is what a learner sees on jwinf, so the jwinf locale keeps it.

| Block | jwinf (old) | current |
| --- | --- | --- |
| `controls_repeat_ext` | wiederhole %1 mal: | wiederhole %1-mal: |
| `lists_repeat` | erzeuge Liste mit %2 mal dem Element %1 | erzeuge Liste mit %2-mal dem Element %1 |
| `text_create_join_container` | Text %1 %2 | verbinden %1 %2 |
| `text_create_join_item` | etwas | Element |

## MakeCode

Regenerate with `python3 scripts/makecode-data/generate.py`.

### Counts

| Set | Blocks |
| --- | ---: |
| micro:bit (incl. shared loops/logic/math/variables/arrays/text/functions) | 218 |
| Calliope mini, extra/overridden | 37 |
| dropped as placeholder-only (micro:bit) | 3 |
| dropped as placeholder-only (Calliope) | 0 |

### By category

| Category | micro:bit | Calliope extra |
| --- | ---: | ---: |
| arrays | 16 | 2 |
| basic | 9 | 8 |
| control | 11 | 0 |
| functions | 3 | 1 |
| game | 22 | 2 |
| images | 6 | 2 |
| input | 21 | 5 |
| led | 12 | 2 |
| logic | 12 | 0 |
| loops | 8 | 0 |
| math | 13 | 0 |
| motors | 0 | 1 |
| music | 15 | 2 |
| pins | 27 | 7 |
| radio | 12 | 0 |
| serial | 17 | 4 |
| text | 12 | 1 |
| variables | 2 | 0 |

### By shape

| Shape | Count |
| --- | ---: |
| boolean | 28 |
| c-block | 5 |
| c-block hat | 18 |
| cap | 2 |
| reporter | 82 |
| stack | 120 |

### German coverage

MakeCode does not publish its per-block translations as a file, and the public translation endpoint that serves the editor's general UI strings returns nothing for the per-block ones (see the generator's module docstring). The loop/logic/math/variables/arrays/text/functions blocks pxt itself builds *are* reproducibly sourced (pxt routes those through the general string table instead); everything else's German comes from a hand-curated table (GERMAN_TEXT in the generator), part of it read live off the running editor and part of it well-established MakeCode wording that was not re-verified live this session.

- 233 blocks with text read off the running editors (sources/live), in both languages
- 0 blocks with no German text — kept in English: none

### Languages

Texts for every language the editors offer, from cdn.makecode.com's translation endpoint; a block whose key has no approved translation keeps its English text.

| Language | micro:bit translated | English fallback | Calliope extra translated | fallback |
| --- | ---: | ---: | ---: | ---: |
| ar | 172 | 46 | 10 | 27 |
| bg | 153 | 65 | 9 | 28 |
| ca | 213 | 5 | 21 | 16 |
| cs | 212 | 6 | 17 | 20 |
| cy | 210 | 8 | 13 | 24 |
| da | 165 | 53 | 11 | 26 |
| el | 183 | 35 | 30 | 7 |
| es-es | 213 | 5 | 28 | 9 |
| fi | 187 | 31 | 14 | 23 |
| fr | 211 | 7 | 28 | 9 |
| gn | 213 | 5 | 16 | 21 |
| he | 167 | 51 | 13 | 24 |
| hu | 213 | 5 | 23 | 14 |
| is | 171 | 47 | 12 | 25 |
| it | 209 | 9 | 30 | 7 |
| ja | 205 | 13 | 19 | 18 |
| ko | 210 | 8 | 22 | 15 |
| lo | 213 | 5 | 22 | 15 |
| nb | 190 | 28 | 14 | 23 |
| nl | 212 | 6 | 31 | 6 |
| nn-no | 196 | 22 | 13 | 24 |
| pl | 213 | 5 | 23 | 14 |
| pt-br | 213 | 5 | 17 | 20 |
| pt-pt | 209 | 9 | 14 | 23 |
| ru | 212 | 6 | 22 | 15 |
| si-lk | 102 | 116 | 6 | 31 |
| sk | 167 | 51 | 12 | 25 |
| sr | 209 | 9 | 19 | 18 |
| sv-se | 171 | 47 | 12 | 25 |
| tr | 182 | 36 | 15 | 22 |
| uk | 177 | 41 | 31 | 6 |
| vi | 210 | 8 | 13 | 24 |
| zh-cn | 203 | 15 | 16 | 21 |
| zh-tw | 195 | 23 | 20 | 17 |

- blocks whose translation key could not be found through their German text (kept English everywhere): micro:bit device_scroll_image, device_show_image_offset, math_convert_unit, soundExpression_createSoundExpression, string_substr_new; Calliope basic_show_icon, device_get_rotation, device_print_message, device_show_image_offset, device_show_number, function_call

### Twins dropped (same text as another block, first id kept)

- array_unshift_statement (same en text as array_unshift)
- lists_create_with (same en text as lists_create_empty)
- music_playable_play_default_bkg (same en text as music_playable_play)
- pxt_break (same en text as break_keyword)
- pxt_continue (same en text as continue_keyword)
- pxt_on_start (same en text as pxt-on-start)
- radio_on_string_drag (same en text as radio_on_number_drag)
- string_indexof (same en text as array_indexof)
- game_resume (same de text as continue_keyword)
- input_on_sound (same de text as device_gesture_event)
- radioRaiseEvent (same de text as control_raise_event)
- synth_get_volume (same de text as device_get_sound_level)

### Colour table

| Category | micro:bit | Calliope mini |
| --- | --- | --- |
| arrays | #E65722 | #E65722 |
| basic | #1E90FF | #54C9C9 |
| control | #333333 | (inherited) |
| functions | #3455DB | #005A9E |
| game | #007A4B | (inherited) |
| images | #7600A8 | (inherited) |
| input | #D400D4 | #C94600 |
| led | #5C2D91 | #8169E6 |
| logic | #00A4A6 | #006970 |
| loops | #00AA00 | #107C10 |
| math | #9400D3 | #712672 |
| motors | (inherited) | #008272 |
| music | #E63022 | #DF4600 |
| pins | #B22222 | (inherited) |
| radio | #E3008C | #E3008C |
| serial | #002050 | (inherited) |
| text | #B8860B | #996600 |
| variables | #DC143C | #A80000 |
