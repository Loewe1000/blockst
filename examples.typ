#import "lib.typ": blockst, scratch

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue")

= Beispiel 1: Interaktives Quiz-Programm
Ein einfaches Quiz, das Fragen stellt, Antworten prüft und den Punktestand verwaltet.

#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #setze-variable("Punkte", 0)
    #verstecke-variable("Punkte")
    #sage-fuer-sekunden("Quiz startet!", sekunden: 2)
    #frage("Was ist 7 × 8?")
    #falls-sonst(
      gleich(antwort(), 56),
      [
        #aendere-variable("Punkte", 1)
        #sage-fuer-sekunden("Richtig!", sekunden: 2)
      ],
      [#sage-fuer-sekunden("Falsch! Es war 56.", sekunden: 2)],
    )
    #frage("Hauptstadt von Frankreich?")
    #falls-sonst(
      gleich(antwort(), "Paris"),
      [
        #aendere-variable("Punkte", 1)
        #sage-fuer-sekunden("Sehr gut!", sekunden: 2)
      ],
      [#sage-fuer-sekunden("Falsch! Es ist Paris.", sekunden: 2)],
    )
    #sage-fuer-sekunden("Quiz beendet! Punkte anzeigen", sekunden: 2)
    #zeige-variable("Punkte")
  ]
]

#pagebreak()

= Beispiel 2: Kollisionserkennung mit Farbsensor
Die Figur läuft vorwärts und dreht sich um, sobald sie eine rote Wand berührt.

#blockst[
  #import scratch.de: *
  
  #wenn-taste-gedrueckt("Leertaste")[
    #wiederhole(anzahl: 200)[
      #falls-sonst(
        wird-farbe-beruehrt(rgb("#FF0000")),
        [
          #drehe-rechts(grad: 180)
          #gehe(schritte: 5)
        ],
        [#gehe(schritte: 3)],
      )
    ]
  ]
]

#pagebreak()

= Beispiel 3: Sortier-Algorithmus (Bubble Sort Visualisierung)
Zeigt schrittweise, wie eine Liste sortiert wird.

#blockst[
  #import scratch.de: *
  
  #wenn-diese-figur-angeklickt[
    #entferne-alles-aus-liste("Zahlen")
    #fuege-zu-liste-hinzu(64, "Zahlen")
    #fuege-zu-liste-hinzu(34, "Zahlen")
    #fuege-zu-liste-hinzu(25, "Zahlen")
    #fuege-zu-liste-hinzu(12, "Zahlen")
    #fuege-zu-liste-hinzu(22, "Zahlen")
    #zeige-liste("Zahlen")
    #sage-fuer-sekunden("Unsortierte Liste!", sekunden: 2)
    #setze-variable("n", laenge-von-liste("Zahlen"))
    #wiederhole(anzahl: eigene-eingabe("n"))[
      #setze-variable("i", 1)
      #wiederhole(anzahl: subtrahiere(eigene-eingabe("n"), 1))[
        #falls-sonst(
          groesser-als(
            element-von-liste(eigene-eingabe("i"), "Zahlen"),
            element-von-liste(addiere(eigene-eingabe("i"), 1), "Zahlen"),
          ),
          [
            #setze-variable("temp", element-von-liste(eigene-eingabe("i"), "Zahlen"))
            #ersetze-element(
              eigene-eingabe("i"),
              "Zahlen",
              element-von-liste(addiere(eigene-eingabe("i"), 1), "Zahlen"),
            )
            #ersetze-element(addiere(eigene-eingabe("i"), 1), "Zahlen", eigene-eingabe("temp"))
          ],
          [],
        )
        #aendere-variable("i", 1)
      ]
    ]
    #sage-fuer-sekunden("Liste sortiert!", sekunden: 2)
  ]
]

#pagebreak()

= Beispiel 4: Namens-Generator mit Zufallselementen
Erstellt zufällige Fantasienamen aus Silben-Listen.

#blockst[
  #import scratch.de: *
  
  #wenn-diese-figur-angeklickt[
    #entferne-alles-aus-liste("Silben")
    #fuege-zu-liste-hinzu("Dra", "Silben")
    #fuege-zu-liste-hinzu("Fen", "Silben")
    #fuege-zu-liste-hinzu("Kor", "Silben")
    #fuege-zu-liste-hinzu("Mel", "Silben")
    #setze-variable(
      "Name",
      verbinde(
        element-von-liste(zufallszahl(von: 1, bis: laenge-von-liste("Silben")), "Silben"),
        element-von-liste(zufallszahl(von: 1, bis: laenge-von-liste("Silben")), "Silben"),
      ),
    )
    #sage(verbinde("Dein Heldenname: ", eigene-eingabe("Name")))
  ]
]

#pagebreak()

= Beispiel 5: Countdown-Timer
Ein visueller Timer, der von 10 herunterzählt.

#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt[
    #setze-variable("Zeit", 10)
    #zeige-variable("Zeit")
    #wiederhole(anzahl: 10)[
      #sage-fuer-sekunden(eigene-eingabe("Zeit"), sekunden: 1)
      #aendere-variable("Zeit", -1)
    ]
    #sage("Zeit abgelaufen!")
    #verstecke-variable("Zeit")
  ]
]

#pagebreak()

= Beispiel 6: Eigener Block für Polygon-Zeichnung
Wiederverwendbarer Block zum Zeichnen von Vielecken.

#blockst[
  #import scratch.de: *
  
  #let polygon-block = eigener-block("Zeichne", none, "Polygon")
  
  #definiere(polygon-block)[
    #wiederhole(anzahl: parameter("Ecken"))[
      #gehe(schritte: 50)
      #drehe-rechts(grad: dividiere(360, parameter("Ecken")))
    ]
  ]
  
  #wenn-gruene-flagge-geklickt[
    #polygon-block(6)
    #drehe-rechts(grad: 30)
    #polygon-block(5)
  ]
]
