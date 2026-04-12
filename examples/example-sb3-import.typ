// Example: clean SB3 import showcase
#import "../lib.typ": blockst, scratch

#set page(width: 20cm, height: auto, margin: 3mm, fill: white)

#let sb3-bytes = read("Mampf-Matze Lösung.sb3", encoding: none)

#blockst[
  #import scratch.en: *
  #text(weight: "bold", size: 9pt)[0) Stage preview (all visible assets)]
  #v(1.5mm)
  #sb3-screen-preview(sb3-bytes)

  #text(weight: "bold", size: 9pt)[1) All scripts (auto headers)]
  #v(1.5mm)
  // #sb3-scripts(sb3-bytes)

  #v(4mm)

  #text(weight: "bold", size: 9pt)[2) Stage scripts only]
  #v(1.5mm)
  #sb3-scripts(sb3-bytes, target: "Orange")

  #v(4mm)

  #text(weight: "bold", size: 9pt)[3) Stage lists]
  #v(1.5mm)
  #sb3-lists(sb3-bytes, target: "stage")

  #v(3mm)

  #text(weight: "bold", size: 9pt)[4) Stage variables]
  #v(1.5mm)
  #sb3-variables(sb3-bytes, target: "stage")
]
