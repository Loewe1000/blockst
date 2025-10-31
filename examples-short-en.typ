#import "lib.typ": blockst, scratch

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue")

= Example 1: Endless Loop
#blockst[
  #import scratch.en: *
  
  #when-flag-clicked[
    #repeat(times: 100)[
      #move()
    ]
  ]
]

#pagebreak()

= Example 2: Condition with Key Press
#blockst[
  #import scratch.en: *
  
  #when-key-pressed("space")[
    #if-then-else(
      key-pressed("up arrow"),
      [#move()],
      [#turn-right()],
    )
  ]
]

#pagebreak()

= Example 3: Change Variable
#blockst[
  #import scratch.en: *
  
  #when-sprite-clicked[
    #set-variable-to("Score", 0)
    #change-variable-by("Score", 10)
    #show-variable("Score")
  ]
]

#pagebreak()

= Example 4: Fill List
#blockst[
  #import scratch.en: *
  
  #when-flag-clicked[
    #delete-all-of-list("Names")
    #add-to-list("Anna", "Names")
    #add-to-list("Ben", "Names")
    #add-to-list("Clara", "Names")
    #show-list("Names")
  ]
]

#pagebreak()

= Example 5: Nested Condition
#blockst[
  #import scratch.en: *
  
  #when-flag-clicked[
    #if-then-else(
      op-and(
        greater-than(mouse-x(), 0),
        less-than(mouse-y(), 100),
      ),
      [#say-for-secs("Mouse in range!", secs: 2)],
      [#say-for-secs("Outside", secs: 2)],
    )
  ]
]

#pagebreak()

= Example 6: Using Operators
Demonstration of operator blocks (visual representation only).

#blockst[
  #import scratch.en: *
  
  #when-flag-clicked[
    #set-variable-to("Result", add(multiply(3, 2), 5))
    #say-for-secs(custom-input("Result"), secs: 2)
  ]
]

#pagebreak()

= Example 7: Color Collision
#blockst[
  #import scratch.en: *
  
  #when-flag-clicked[
    #repeat(times: 50)[
      #move()
      #if-then(
        touching-color(rgb("#FF0000")),
        [#turn-right(degrees: 180)],
      )
    ]
  ]
]

#pagebreak()

= Example 8: Custom Block with Parameters
#blockst[
  #import scratch.en: *
  
  #let jump = custom-block("Jump", (name: "Count"), "times")
  
  #define(jump)[
    #repeat(times: parameter("Count"))[
      #change-y(dy: 10)
    ]
  ]
  
  #when-flag-clicked[
    #jump(5)
  ]
]
