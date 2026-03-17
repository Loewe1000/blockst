// Example 3 – Variable & list monitors
#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#blockst[
  #import scratch.de: *

  // Program blocks
  #wenn-gruene-flagge-geklickt[
    #setze-variable("Highscore", 0)
    #fuege-zu-liste-hinzu("Anna", "Spieler")
    #fuege-zu-liste-hinzu("Ben", "Spieler")
    #fuege-zu-liste-hinzu("Clara", "Spieler")
  ]

  // Visual monitors (like on the Scratch stage)
  #variable(name: "Highscore", wert: 100)

  #liste(
    name: "Spieler",
    items: ("Anna", "Ben", "Clara"),
  )
]
