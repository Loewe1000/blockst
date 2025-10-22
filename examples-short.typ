#import "scratch.typ": *

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue")

= Beispiel 1: Endlosschleife
#ereignis[Wenn Flagge angeklickt][
  #wiederhole(
    anzahl: 100,
    loop-body: gehe-schritt(schritt: 10),
  )
]

#pagebreak()

= Beispiel 2: Bedingung mit Tastendruck
#ereignis[Wenn Leertaste gedrückt][
  #falls(
    taste-gedrückt(taste: "Pfeil nach oben"),
    dann-body: gehe-schritt(schritt: 10),
    sonst-body: drehe-dich-um(richtung: "rechts", grad: 15),
  )
]

#pagebreak()

= Beispiel 3: Variable ändern
#ereignis[Wenn Figur angeklickt][
  #setze-variable-auf(name: "Punkte", wert: 0)
  #ändere-variable-um(name: "Punkte", wert: 10)
  #zeige-variable(name: "Punkte")
]

#pagebreak()

= Beispiel 4: Liste befüllen
#ereignis[Wenn Flagge angeklickt][
  #lösche-alles-aus(liste: "Namen")
  #füge-zu-hinzu(wert: "Anna", liste: "Namen")
  #füge-zu-hinzu(wert: "Ben", liste: "Namen")
  #füge-zu-hinzu(wert: "Clara", liste: "Namen")
  #zeige-liste(liste: "Namen")
]

#pagebreak()

= Beispiel 5: Verschachtelte Bedingung
#ereignis[Wenn Flagge angeklickt][
  #falls(
    und(
      größer-als(maus-x-position(), 0),
      kleiner-als(maus-y-position(), 100),
      nested: true
    ),
    dann-body: sage(text: "Maus im Bereich!"),
    sonst-body: sage(text: "Außerhalb"),
  )
]

#pagebreak()

= Beispiel 6: Operatoren nutzen
#ereignis[Wenn Flagge angeklickt][
  #setze-variable-auf(name: "Ergebnis", wert: plus(mal(3, 4), 5))
  #sage(text: variable("Ergebnis"))
]

#pagebreak()

= Beispiel 7: Farbkollision
#ereignis[Wenn Flagge angeklickt][
  #wiederhole(
    anzahl: 50,
    loop-body: block[
      #gehe-schritt(schritt: 5)
      #falls(
        wird-farbe-berührt(color: rgb("#FF0000")),
        dann-body: drehe-dich-um(richtung: "rechts", grad: 180),
      )
    ],
  )
]

#pagebreak()

= Beispiel 8: Eigener Block
#let mein-block = eigener-block("Springe", none, "mal")

#definiere(mein-block)[
  #wiederhole(
    anzahl: variable("Anzahl"),
    loop-body: ändere-y-um(schritt: 10),
  )
]

#ereignis[Wenn Flagge angeklickt][
  #mein-block(dark: false, 5)
]
