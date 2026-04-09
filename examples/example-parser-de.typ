// Example – deutscher Textparser
#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#blockst[
  #import scratch.text.de: *

  #render-scratch-text("Wenn Flag angeklickt wird
wiederhole 4 mal
gehe 40 er Schritt
drehe dich rechts um 90 Grad
ende
falls <wird [Mauszeiger v] berührt ?>
sage [Hallo!] für 1 Sekunden
sonst
sage [Noch unterwegs]
ende
ende")
]
