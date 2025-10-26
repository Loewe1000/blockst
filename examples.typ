#import "scratch.typ": *

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue")

= Beispiel 1: Interaktives Quiz-Programm
Ein einfaches Quiz, das Fragen stellt, Antworten prüft und den Punktestand verwaltet.

#ereignis-grüne-flagge([
  #setze-variable-auf(name: "Punkte", wert: 0)
  #verstecke-variable(name: "Punkte")
  #sage(text: "Quiz startet!", sekunden: 2)
  #frage(text: "Was ist 7 × 8?")
  #falls(
    gleich(antwort(), 56),
    dann: [
      #ändere-variable-um(name: "Punkte", wert: 1)
      #sage(text: "Richtig!", sekunden: 2)
    ],
    sonst: sage(text: "Falsch! Es war 56.", sekunden: 2),
  )
  #frage(text: "Hauptstadt von Frankreich?")
  #falls(
    gleich(antwort(), "Paris"),
    dann: [
      #ändere-variable-um(name: "Punkte", wert: 1)
      #sage(text: "Sehr gut!", sekunden: 2)
    ],
    sonst: sage(text: "Falsch! Es ist Paris.", sekunden: 2),
  )
  #sage(text: "Quiz beendet! Punkte anzeigen")
  #zeige-variable(name: "Punkte")
])

#pagebreak()

= Beispiel 2: Kollisionserkennung mit Farbsensor
Die Figur läuft vorwärts und dreht sich um, sobald sie eine rote Wand berührt.

#ereignis-taste("Leertaste", [
  #wiederhole(anzahl: 200, body: [
    #falls(
      wird-farbe-berührt(color: rgb("#FF0000")),
      dann: [
        #drehe-dich-um(richtung: "rechts", grad: 180)
        #gehe-schritt(schritt: 5)
      ],
      sonst: gehe-schritt(schritt: 3),
    )
  ])
])

#pagebreak()

= Beispiel 3: Sortier-Algorithmus (Bubble Sort Visualisierung)
Zeigt schrittweise, wie eine Liste sortiert wird.

#ereignis-figur-angeklickt([
  #lösche-alles-aus(liste: "Zahlen")
  #füge-zu-hinzu(wert: 64, liste: "Zahlen")
  #füge-zu-hinzu(wert: 34, liste: "Zahlen")
  #füge-zu-hinzu(wert: 25, liste: "Zahlen")
  #füge-zu-hinzu(wert: 12, liste: "Zahlen")
  #füge-zu-hinzu(wert: 22, liste: "Zahlen")
  #zeige-liste(liste: "Zahlen")
  #sage(text: "Unsortierte Liste!", sekunden: 2)
  #setze-variable-auf(name: "n", wert: länge-von-liste("Zahlen"))
  #wiederhole(anzahl: variable("n"), body: [
    #setze-variable-auf(name: "i", wert: 1)
    #wiederhole(anzahl: minus(arg1: variable("n"), arg2: 1), body: [
      #falls(
        größer-als(
          element-von(index: variable("i"), liste: "Zahlen"),
          element-von(index: plus(arg1: variable("i"), arg2: 1), liste: "Zahlen"),
        ),
        dann: [
          #setze-variable-auf(name: "temp", wert: element-von(index: variable("i"), liste: "Zahlen"))
          #ersetze-element-von-durch(
            index: variable("i"),
            liste: "Zahlen",
            wert: element-von(index: plus(arg1: variable("i"), arg2: 1), liste: "Zahlen"),
          )
          #ersetze-element-von-durch(index: plus(arg1: variable("i"), arg2: 1), liste: "Zahlen", wert: variable("temp"))
        ],
      )
      #ändere-variable-um(name: "i", wert: 1)
    ])
  ])
  #sage(text: "Liste sortiert!", sekunden: 2)
])

#pagebreak()

= Beispiel 4: Namens-Generator mit Zufallselementen
Erstellt zufällige Fantasienamen aus Silben-Listen.

#ereignis-figur-angeklickt([
  #lösche-alles-aus(liste: "Silben")
  #füge-zu-hinzu(wert: "Dra", liste: "Silben")
  #füge-zu-hinzu(wert: "Fen", liste: "Silben")
  #füge-zu-hinzu(wert: "Kor", liste: "Silben")
  #füge-zu-hinzu(wert: "Mel", liste: "Silben")
  #setze-variable-auf(
    name: "Name",
    wert: verbinde(
      element-von(index: zufallszahl(von: 1, bis: länge-von-liste("Silben")), liste: "Silben"),
      element-von(index: zufallszahl(von: 1, bis: länge-von-liste("Silben")), liste: "Silben"),
    ),
  )
  #sage(text: verbinde("Dein Heldenname: ", variable("Name")))
])

#pagebreak()

= Beispiel 5: Countdown-Timer
Ein visueller Timer, der von 10 herunterzählt.

#ereignis-grüne-flagge([
  #setze-variable-auf(name: "Zeit", wert: 10)
  #zeige-variable(name: "Zeit")
  #wiederhole(anzahl: 10, body: [
    #sage(text: variable("Zeit"), sekunden: 1)
    #ändere-variable-um(name: "Zeit", wert: -1)
  ])
  #sage(text: "Zeit abgelaufen!")
  #verstecke-variable(name: "Zeit")
])

#pagebreak()

= Beispiel 6: Eigener Block für Polygon-Zeichnung
Wiederverwendbarer Block zum Zeichnen von Vielecken.

#let polygon-block = eigener-block("Zeichne", none, "Polygon")

#definiere(polygon-block)[
  #wiederhole(anzahl: variable("Ecken"), body: [
    #gehe-schritt(schritt: 50)
    #drehe-dich-um(richtung: "rechts", grad: geteilt(arg1: 360, arg2: variable("Ecken")))
  ])
]

#ereignis-grüne-flagge([
  #polygon-block(dark: false, 6)
  #drehe-dich-um(richtung: "rechts", grad: 30)
  #polygon-block(dark: false, 5)
])


#pagebreak()

= Beispiel 2: Kollisionserkennung mit Farbsensor
Die Figur läuft vorwärts und dreht sich um, sobald sie eine rote Wand berührt.

#ereignis[Wenn Leertaste gedrückt][
  #wiederhole(
    anzahl: 200,
    body: block[
      #falls(
        wird-farbe-berührt(color: rgb("#FF0000")),
        dann: block[
          #drehe-dich-um(richtung: "rechts", grad: 180)
          #gehe-schritt(schritt: 5)
        ],
        sonst: gehe-schritt(schritt: 3),
      )
    ],
  )
]

#pagebreak()

= Beispiel 3: Sortier-Algorithmus (Bubble Sort Visualisierung)
Zeigt schrittweise, wie eine Liste sortiert wird.

#ereignis[Wenn Figur angeklickt][
  #lösche-alles-aus(liste: "Zahlen")
  #füge-zu-hinzu(wert: 64, liste: "Zahlen")
  #füge-zu-hinzu(wert: 34, liste: "Zahlen")
  #füge-zu-hinzu(wert: 25, liste: "Zahlen")
  #füge-zu-hinzu(wert: 12, liste: "Zahlen")
  #füge-zu-hinzu(wert: 22, liste: "Zahlen")
  #zeige-liste(liste: "Zahlen")
  #sage(text: "Unsortierte Liste!", sekunden: 2)
  #setze-variable-auf(name: "n", wert: länge-von-liste("Zahlen"))
  #wiederhole(
    anzahl: variable("n"),
    body: block[
      #setze-variable-auf(name: "i", wert: 1)
      #wiederhole(
        anzahl: minus(arg1: variable("n"), arg2: 1),
        body: block[
          #falls(
            größer-als(
              element-von(index: variable("i"), liste: "Zahlen"),
              element-von(index: plus(arg1: variable("i"), arg2: 1), liste: "Zahlen"),
            ),
            dann: block[
              #setze-variable-auf(name: "temp", wert: element-von(index: variable("i"), liste: "Zahlen"))
              #ersetze-element-von-durch(
                index: variable("i"),
                liste: "Zahlen",
                wert: element-von(index: plus(arg1: variable("i"), arg2: 1), liste: "Zahlen"),
              )
              #ersetze-element-von-durch(index: plus(arg1: variable("i"), arg2: 1), liste: "Zahlen", wert: variable("temp"))
            ],
          )
          #ändere-variable-um(name: "i", wert: 1)
        ],
      )
    ],
  )
  #sage(text: "Liste sortiert!", sekunden: 2)
]

#pagebreak()

= Beispiel 4: Namens-Generator mit Zufallselementen
Erstellt zufällige Fantasienamen aus Silben-Listen.

#ereignis[Wenn Figur angeklickt][
  #lösche-alles-aus(liste: "Silben")
  #füge-zu-hinzu(wert: "Dra", liste: "Silben")
  #füge-zu-hinzu(wert: "Fen", liste: "Silben")
  #füge-zu-hinzu(wert: "Kor", liste: "Silben")
  #füge-zu-hinzu(wert: "Mel", liste: "Silben")
  #setze-variable-auf(
    name: "Name",
    wert: verbinde(
      element-von(index: zufallszahl(von: 1, bis: länge-von-liste("Silben")), liste: "Silben"),
      element-von(index: zufallszahl(von: 1, bis: länge-von-liste("Silben")), liste: "Silben"),
    ),
  )
  #sage(text: verbinde("Dein Heldenname: ", variable("Name")))
]

#pagebreak()

= Beispiel 5: Countdown-Timer
Ein visueller Timer, der von 10 herunterzählt.

#ereignis[Wenn Flagge angeklickt][
  #setze-variable-auf(name: "Zeit", wert: 10)
  #zeige-variable(name: "Zeit")
  #wiederhole(
    anzahl: 10,
    body: block[
      #sage(text: variable("Zeit"), sekunden: 1)
      #ändere-variable-um(name: "Zeit", wert: -1)
    ],
  )
  #sage(text: "Zeit abgelaufen!")
  #verstecke-variable(name: "Zeit")
]

#pagebreak()

= Beispiel 6: Eigener Block für Polygon-Zeichnung
Wiederverwendbarer Block zum Zeichnen von Vielecken.

#let polygon-block = eigener-block("Zeichne", none, "Polygon")

#definiere(polygon-block)[
  #wiederhole(
    anzahl: variable("Ecken"),
    body: block[
      #gehe-schritt(schritt: 50)
      #drehe-dich-um(richtung: "rechts", grad: geteilt(arg1: 360, arg2: variable("Ecken")))
    ],
  )
]

#ereignis[Wenn Flagge angeklickt][
  #polygon-block(dark: false, 6)
  #drehe-dich-um(richtung: "rechts", grad: 30)
  #polygon-block(dark: false, 5)
]