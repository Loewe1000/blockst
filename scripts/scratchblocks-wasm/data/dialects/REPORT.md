# Generated block data — what came out

Regenerate with `python3 scripts/blockly-data/generate.py`.

## Counts

| Set | Blocks |
| --- | ---: |
| Blockly standard (German) | 44 |
| jwinf robot world | 38 |
| jwinf turtle world | 42 |
| standard blocks worded differently in the old Blockly | 4 |
| world blocks dropped as socket-twins of a field variant | 23 |

## Shapes

| Shape | Blockly | jwinf worlds |
| --- | ---: | ---: |
| boolean | 12 | 0 |
| c-block | 8 | 0 |
| cap | 2 | 0 |
| hat | 1 | 0 |
| reporter | 16 | 22 |
| stack | 5 | 56 |

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
| micro:bit (incl. shared loops/logic/math/variables/arrays/text/functions) | 217 |
| Calliope mini, extra/overridden | 39 |
| dropped as placeholder-only (micro:bit) | 3 |
| dropped as placeholder-only (Calliope) | 0 |

### By category

| Category | micro:bit | Calliope extra |
| --- | ---: | ---: |
| arrays | 17 | 2 |
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
| music | 14 | 2 |
| pins | 27 | 7 |
| radio | 12 | 0 |
| serial | 17 | 4 |
| text | 12 | 1 |
| variables | 1 | 2 |

### By shape

| Shape | Count |
| --- | ---: |
| boolean | 28 |
| c-block | 5 |
| c-block hat | 18 |
| cap | 2 |
| reporter | 81 |
| stack | 122 |

### German coverage

MakeCode does not publish its per-block translations as a file, and the public translation endpoint that serves the editor's general UI strings returns nothing for the per-block ones (see the generator's module docstring). The loop/logic/math/variables/arrays/text/functions blocks pxt itself builds *are* reproducibly sourced (pxt routes those through the general string table instead); everything else's German comes from a hand-curated table (GERMAN_TEXT in the generator), part of it read live off the running editor and part of it well-established MakeCode wording that was not re-verified live this session.

- 235 blocks with text read off the running editors (sources/live), in both languages
- 0 blocks with no German text — kept in English: none

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
