#import "scratch.typ": *

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue")

= Beispiel 1: Endlosschleife
#ereignis-grüne-flagge([
  #wiederhole(
    anzahl: 100,
    body: gehe-schritt(schritt: 10),
  )
])

#pagebreak()

= Beispiel 2: Bedingung mit Tastendruck
#ereignis-taste("Leertaste", [
  #falls(
    taste-gedrückt(taste: "Pfeil nach oben"),
    dann: gehe-schritt(schritt: 10),
    sonst: drehe-dich-um(richtung: "rechts", grad: 15),
  )
])

#pagebreak()

= Beispiel 3: Variable ändern
#ereignis-figur-angeklickt([
  #setze-variable-auf(name: "Punkte", wert: 0)
  #ändere-variable-um(name: "Punkte", wert: 10)
  #zeige-variable(name: "Punkte")
])

#pagebreak()

= Beispiel 4: Liste befüllen
#ereignis-grüne-flagge([
  #lösche-alles-aus(liste: "Namen")
  #füge-zu-hinzu(wert: "Anna", liste: "Namen")
  #füge-zu-hinzu(wert: "Ben", liste: "Namen")
  #füge-zu-hinzu(wert: "Clara", liste: "Namen")
  #zeige-liste(liste: "Namen")
])

#pagebreak()

= Beispiel 5: Verschachtelte Bedingung
#ereignis-grüne-flagge([
  #falls(
    und(
      größer-als(maus-x-position(), 0),
      kleiner-als(maus-y-position(), 100),
      nested: true,
    ),
    dann: sage(text: "Maus im Bereich!"),
    sonst: sage(text: "Außerhalb"),
  )
])

#pagebreak()

= Beispiel 6: Operatoren nutzen
Demonstration von Operator-Blöcken (nur visuelle Darstellung).

#ereignis-grüne-flagge([
  #setze-variable-auf(name: "Ergebnis", wert: plus(arg1: mal(arg1: 3, arg2: 2), arg2: 5))
  #sage(text: variable("Ergebnis"))
])

#pagebreak()

= Beispiel 7: Farbkollision
#ereignis-grüne-flagge([
  #wiederhole(anzahl: 50, body: [
    #gehe-schritt(schritt: 5)
    #falls(
      wird-farbe-berührt(color: rgb("#FF0000")),
      dann: drehe-dich-um(richtung: "rechts", grad: 180),
    )
  ])
])

#pagebreak()

= Beispiel 8: Eigener Block
#let mein-block = eigener-block("Springe", none, "mal")

#definiere(mein-block)[
  #wiederhole(
    anzahl: variable("Anzahl"),
    body: ändere-y-um(schritt: 10),
  )
]

#ereignis-grüne-flagge([
  #mein-block(dark: false, 5)
])
