// Example: render a static stage/sprite screen preview from an sb3 project
#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 4mm, fill: white)

#let sb3-bytes = read("Mampf-Matze Lösung.sb3", encoding: none)

#blockst[
  #import scratch.de: *

  #text(weight: "bold", size: 9pt)[Mampf-Matze: Bildschirmvorschau]
  #v(1.5mm)
  #sb3-screen-preview(sb3-bytes, unit: 1)

  #v(4mm)

  #text(weight: "bold", size: 9pt)[Skripte aus dem Projekt]
  #v(1.5mm)
  #sb3-scripts(sb3-bytes)
]
