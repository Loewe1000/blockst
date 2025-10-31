#import "lib.typ": blockst, scratch

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue")

= Beispiel 1: Endlosschleife
#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #wiederhole(anzahl: 100)[
      #gehe()
    ]
  ]
]

#pagebreak()

= Beispiel 2: Bedingung mit Tastendruck
#blockst[
  #import scratch.de: *
  
  #wenn-taste-gedrueckt("Leertaste")[
    #falls-sonst(
      taste-gedrueckt("Pfeil nach oben"),
      [#gehe()],
      [#drehe-rechts()],
    )
  ]
]

#pagebreak()

= Beispiel 3: Variable ändern
#blockst[
  #import scratch.de: *
  
  #wenn-diese-figur-angeklickt[
    #setze-variable("Punkte", 0)
    #aendere-variable("Punkte", 10)
    #zeige-variable("Punkte")
  ]
]

#pagebreak()

= Beispiel 4: Liste befüllen
#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #entferne-alles-aus-liste("Namen")
    #fuege-zu-liste-hinzu("Anna", "Namen")
    #fuege-zu-liste-hinzu("Ben", "Namen")
    #fuege-zu-liste-hinzu("Clara", "Namen")
    #zeige-liste("Namen")
  ]
]

#pagebreak()

= Beispiel 5: Verschachtelte Bedingung
#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #falls-sonst(
      und(
        groesser-als(maus-x(), 0),
        kleiner-als(maus-y(), 100),
      ),
      [#sage-fuer-sekunden("Maus im Bereich!", sekunden: 2)],
      [#sage-fuer-sekunden("Außerhalb", sekunden: 2)],
    )
  ]
]

#pagebreak()

= Beispiel 6: Operatoren nutzen
Demonstration von Operator-Blöcken (nur visuelle Darstellung).

#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #setze-variable("Ergebnis", addiere(multipliziere(3, 2), 5))
    #sage-fuer-sekunden(eigene-eingabe("Ergebnis"), sekunden: 2)
  ]
]

#pagebreak()

= Beispiel 7: Farbkollision
#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #wiederhole(anzahl: 50)[
      #gehe()
      #falls(
        wird-farbe-beruehrt(rgb("#FF0000")),
        [#drehe-rechts(grad: 180)],
      )
    ]
  ]
]

#pagebreak()

= Beispiel 8: Eigener Block mit Parametern
#blockst[
  #import scratch.de: *
  
  #let springe = eigener-block("Springe", (name: "Anzahl"), "mal")
  
  #definiere(springe)[
    #wiederhole(anzahl: parameter("Anzahl"))[
      #aendere-y(dy: 10)
    ]
  ]
  
  #wenn-gruene-flagge-geklickt[
    #springe(5)
  ]
]
