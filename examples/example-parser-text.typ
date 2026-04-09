// Example – Experimental text parser (English, v1)
#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#blockst[
  #import scratch.text.en: *

  #render-scratch-text("when flag clicked
repeat (4)
move (40) steps
turn right (90) degrees
end
if <touching [edge] ?> then
say [Bounce!] for (1) seconds
else
say [Still moving]
end
end")
]
