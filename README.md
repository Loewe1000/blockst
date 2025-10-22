# Blockst – Scratch-Blöcke in Typst

**Blockst** ist ein Typst-Paket, das es ermöglicht, Scratch-Programmierblöcke direkt in Typst-Dokumenten zu erstellen. Perfekt für Programmier-Tutorials, Bildungsmaterialien und Dokumentationen von visuellen Programmierkonzepten.

## Features

- ✅ **Alle Scratch-Kategorien:** Bewegung, Aussehen, Klang, Ereignisse, Steuerung, Fühlen, Operatoren, Variablen, Listen und eigene Blöcke
- ✅ **Originalgetreue Farben:** Normal- und High-Contrast-Modi
- ✅ **Verschachtelte Strukturen:** Schleifen, Bedingungen (falls-dann-sonst), eigene Blöcke
- ✅ **Reporter & Operatoren:** Ovale und runde Pills, Diamant-Bedingungen
- ✅ **Deutsche Beschriftungen:** Alle Blöcke in deutscher Sprache

## Installation
Kopiere die Datei `scratch.typ` in dein Projekt-Verzeichnis und importiere sie:

```typst
#import "scratch.typ": *
```

## Schnellstart

### Beispiel 1: Einfache Bewegung

```typst
#ereignis[Wenn Flagge angeklickt][
  #wiederhole(
    anzahl: 100,
    loop-body: gehe-schritt(schritt: 10),
  )
]
```

![Beispiel 1](examples/example-1.png)

### Beispiel 2: Bedingung mit Tastendruck

```typst
#ereignis[Wenn Leertaste gedrückt][
  #falls(
    taste-gedrückt(taste: "Pfeil nach oben"),
    dann-body: gehe-schritt(schritt: 10),
    sonst-body: drehe-dich-um(richtung: "rechts", grad: 15),
  )
]
```

![Beispiel 2](examples/example-2.png)

### Beispiel 3: Variablen verwenden

```typst
#ereignis[Wenn Figur angeklickt][
  #setze-variable-auf(name: "Punkte", wert: 0)
  #ändere-variable-um(name: "Punkte", wert: 10)
  #zeige-variable(name: "Punkte")
]
```

![Beispiel 3](examples/example-3.png)

### Beispiel 4: Listen befüllen

```typst
#ereignis[Wenn Flagge angeklickt][
  #lösche-alles-aus(liste: "Namen")
  #füge-zu-hinzu(wert: "Anna", liste: "Namen")
  #füge-zu-hinzu(wert: "Ben", liste: "Namen")
  #füge-zu-hinzu(wert: "Clara", liste: "Namen")
  #zeige-liste(liste: "Namen")
]
```

![Beispiel 4](examples/example-4.png)

### Beispiel 5: Verschachtelte Bedingungen

```typst
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
```

![Beispiel 5](examples/example-5.png)

### Beispiel 6: Operatoren verwenden

```typst
#ereignis[Wenn Flagge angeklickt][
  #setze-variable-auf(name: "Ergebnis", wert: plus(mal(3, 4), 5))
  #sage(text: variable("Ergebnis"))
]
```

![Beispiel 6](examples/example-6.png)

### Beispiel 7: Farbkollision erkennen

```typst
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
```

![Beispiel 7](examples/example-7.png)

### Beispiel 8: Eigene Blöcke definieren

```typst
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
```

![Beispiel 8](examples/example-8.png)

## Verfügbare Kategorien

### 🔵 Bewegung
- `gehe-zu(x, y)`, `gleite-in-zu(sek, x, y)`, `gehe(zu)`, `gleite-in(sek, zu)`
- `drehe-dich(zu)`, `drehe-dich-um(richtung, grad)`, `setze-Richtung-auf(grad)`
- `gehe-schritt(schritt)`, `ändere-x-um(schritt)`, `setze-x-auf(x)`, `ändere-y-um(schritt)`, `setze-y-auf(y)`
- `pralle-vom-rand-ab()`

### 🟣 Aussehen
- `sage(text, sekunden)`, `denke(text, sekunden)`
- `wechsle-zu-kostüm(kostüm)`, `wechsle-zum-nächsten-kostüm()`
- `wechsle-zu-bühnenbild(bild)`, `wechsle-zum-nächsten-bühnenbild()`
- `ändere-größe-um(wert)`, `setze-größe-auf(wert)`
- `ändere-effekt(effekt, um)`, `setze-effekt(effekt, auf)`, `schalte-grafikeffekte-aus()`
- `zeige-dich()`, `verstecke-dich()`
- Reporter: `kostüm(eigenschaft)`, `bühnenbild(eigenschaft)`, `größe()`

### 🟡 Ereignisse
- `ereignis[Label][Body]` – Startet eine Block-Sequenz

### 🟠 Steuerung
- `wiederhole(anzahl, loop-body)` – Schleife mit festgelegter Anzahl
- `falls(bedingung, dann-body, sonst-body)` – If-else-Verzweigung

### 🔷 Fühlen
- `frage(text)`, `setze-ziehbarkeit-auf(modus)`, `setze-stoppuhr-zurück()`
- Reporter: `entfernung-von(objekt)`, `antwort()`, `maus-x-position()`, `maus-y-position()`, `stoppuhr()`, `von-bühne(eigenschaft, objekt)`, `zeit(einheit)`, `tage-seit-2000()`, `benutzername()`
- Bedingungen: `taste-gedrückt(taste, nested)`, `maustaste-gedrückt(nested)`, `wird-mauszeiger-berührt(nested)`, `wird-farbe-berührt(color, nested)`, `farbe-berührt(color, nested)`

### 🟢 Operatoren
- Arithmetik: `plus(arg1, arg2)`, `minus(arg1, arg2)`, `mal(arg1, arg2)`, `geteilt(arg1, arg2)`, `modulo(arg1, arg2)`
- Vergleiche: `größer-als(arg1, arg2, nested)`, `kleiner-als(arg1, arg2, nested)`, `gleich(arg1, arg2, nested)`
- Logik: `und(arg1, arg2, nested)`, `oder(arg1, arg2, nested)`, `nicht(arg1, nested)`
- Text: `verbinde(text1, text2)`, `zeichen(position, von)`, `länge-von(text)`, `enthält(text, zeichen, nested)`
- Mathematik: `zufallszahl(von, bis)`, `gerundet(zahl)`, `betrag-von(operation, zahl)`

### 🟠 Variablen
- `setze-variable-auf(name, wert)`, `ändere-variable-um(name, wert)`
- `zeige-variable(name)`, `verstecke-variable(name)`
- Reporter: `variable(name)`

### 🟠 Listen
- `füge-zu-hinzu(wert, liste)`, `lösche-aus(index, liste)`, `lösche-alles-aus(liste)`
- `füge-bei-in-ein(wert, index, liste)`, `ersetze-element-von-durch(index, liste, wert)`
- Reporter: `element-von(index, liste)`, `nummer-von-in(wert, liste)`, `länge-von-liste(liste)`
- Bedingung: `liste-enthält(liste, wert, nested)`
- `zeige-liste(liste)`, `verstecke-liste(liste)`

### 🩷 Eigene Blöcke
- `eigener-block(body)` – Erstellt einen eigenen Anweisungsblock
- `eigene-eingabe(text)` – Weißer Platzhalter für Argumente
- `definiere(label)[body]` – Definitionsblock (ähnlich wie Ereignis)

## Erweiterte Beispiele

Für umfangreichere Beispiele siehe:
- `examples.typ` – Komplexe Algorithmen (Quiz, Bubble Sort, Timer, etc.)
- `examples-short.typ` – Kurze, prägnante Beispiele

## High-Contrast-Modus

Um den High-Contrast-Modus zu aktivieren, setze in `scratch.typ`:

```typst
#let high-contrast = true
```

## Lizenz

Dieses Projekt steht unter der MIT-Lizenz.

## Beitragen

Beiträge, Issues und Feature-Requests sind willkommen! Erstelle einfach ein Issue oder einen Pull Request auf GitHub.

---

**Erstellt mit ❤️ für die Scratch- und Typst-Community**
