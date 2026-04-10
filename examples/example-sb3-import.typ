// Example: clean SB3 import showcase
#import "@preview/blockst:0.2.0": blockst, scratch

#set page(width: 20cm, height: auto, margin: 3mm, fill: white)

#let sb3-bytes = read("Listen.sb3", encoding: none)

#blockst[
  #import scratch.sb3: render-sb3-lists, render-sb3-scripts, render-sb3-variables

  #text(weight: "bold", size: 9pt)[1) All scripts (auto headers)]
  #v(1.5mm)
  #render-sb3-scripts(sb3-bytes)

  #v(4mm)

  #text(weight: "bold", size: 9pt)[2) Cat target, script 1]
  #v(1.5mm)
  #render-sb3-scripts(sb3-bytes, target: "Cat", target-script-number: 1)

  #v(4mm)

  #text(weight: "bold", size: 9pt)[3) Stage lists]
  #v(1.5mm)
  #render-sb3-lists(sb3-bytes, target: "stage")

  #v(3mm)

  #text(weight: "bold", size: 9pt)[4) Stage variables]
  #v(1.5mm)
  #render-sb3-variables(sb3-bytes, target: "stage")
]
