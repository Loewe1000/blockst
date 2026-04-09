// Example – scratchblocks-style parser stress test
#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#blockst[
  #import scratch.text.en: *

  #render-scratch-text("when flag clicked
clear graphic effects
forever
if <<mouse down?> and <touching [mouse-pointer] ?>> then
switch costume to [button]
else
add (x position) to [list]
end
move ((x position) + (10)) steps
turn left (9) degrees
end")
]
