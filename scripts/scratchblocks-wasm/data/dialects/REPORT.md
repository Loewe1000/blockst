# Generated block data — what came out

Regenerate with `python3 scripts/blockly-data/generate.py`.

## Counts

| Set | Blocks |
| --- | ---: |
| Blockly standard (German) | 46 |
| jwinf robot world | 38 |
| jwinf turtle world | 42 |
| standard blocks worded differently in the old Blockly | 6 |

## Shapes

| Shape | Blockly | jwinf worlds |
| --- | ---: | ---: |
| boolean | 7 | 0 |
| c-block | 8 | 0 |
| cap | 3 | 0 |
| hat | 1 | 0 |
| reporter | 21 | 22 |
| stack | 6 | 56 |

## Gaps

- 48 world blocks without a German label — these need translating by hand: connect, containerSize, dropAbove, dropBridgeInFront, dropInFront, dropNum, dropNum_noShadow, dropPlatformAbove, dropPlatformInFront, gridEdgeNorth, gridEdgeSouth, jump, jumpAmount, nbInBag, nbWithdrawables, onBlack, onBlue, onBottomArrow, onChocolate, onCircle, onClubs, onCross, onDiamonds, onDotted, onFemale, onGreen, onHearts, onHole, onLeftArrow, onMale, onOneShape, onPaint, onQuadrille, onRightArrow, onRound, onSpades, onSquare, onStar, onStriped, onTopArrow, onTriangle, onTwoShapes, onYellow, wait, waterInFront, withdrawNum, withdrawNum_noShadow, writeCode
- 14 world blocks whose German and English labels are identical, so one of the two upstream tables is wrong — checked by hand, this goes both ways (`row` is English in the German table, `turnleftamountvalue_options` is German in the English one): alert, col, colour2, inputvalue, log, peneither, row, turn, turnleftamountvalue_moreoptions, turnleftamountvalue_noround, turnleftamountvalue_options, turnrightamountvalue_moreoptions, turnrightamountvalue_noround, turnrightamountvalue_options

## Wording that differs between old and current Blockly

The old wording is what a learner sees on jwinf, so the jwinf locale keeps it.

| Block | jwinf (old) | current |
| --- | --- | --- |
| `controls_repeat_ext` | wiederhole %1 mal: | wiederhole %1-mal: |
| `lists_repeat` | erzeuge Liste mit %2 mal dem Element %1 | erzeuge Liste mit %2-mal dem Element %1 |
| `procedures_defnoreturn` | %1 | um %1 |
| `procedures_defreturn` | %1 | um %1 |
| `text_create_join_container` | Text %1 %2 | verbinden %1 %2 |
| `text_create_join_item` | etwas | Element |
