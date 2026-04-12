#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#blockst[
  #import scratch.en: *

  #render-text("
    when flag clicked
      repeat (4)
        move (40) steps
        if <touching [mouse-pointer] ?> then
          say [Hello parser]
        else
          turn right (15) degrees
        end
      end
  ")
]