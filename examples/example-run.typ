// Example 5 – Scratch-run (turtle graphics)
#import "../lib.typ": blockst, scratch, scratch-run, set-scratch-run

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#import scratch.exec.de: *

// Simple square
#scratch-run(
  stift-ein(),
  quadrat(groesse: 70),
)

#v(4mm)

// Coloured triangle with axes
#set-scratch-run(zeige-gitter: true, zeige-achsen: true)

#scratch-run(
  setze-farbe(farbe: rgb("#4C97FF")),
  setze-dicke(dicke: 2),
  stift-ein(),
  dreieck(groesse: 70),
)

#set-scratch-run(zeige-gitter: false, zeige-achsen: false)
