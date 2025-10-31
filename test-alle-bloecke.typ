#import "lib.typ": blockst, set-blockst, scratch

#set page(width: 21cm, height: auto, margin: 1cm)
#set text(size: 11pt)

#set-blockst(theme: "normal", scale: 100%)

= Bewegung

#blockst[
  #import scratch.de: *
  
  #gehe()
  #drehe-rechts()
  #drehe-links()
  #gehe-zu-position("zufällige Position")
  #gehe-zu()
  #gleite-zu-position("Mauszeiger")
  #gleite-zu()
  #setze-richtung()
  #drehe-dich-zu("Mauszeiger")
  
  #aendere-x()
  #setze-x()
  #aendere-y()
  #setze-y()
  
  #pralle-vom-rand-ab()
  #setze-drehtyp("rechts-links")
  #x-position()
  #y-position()
  #richtung()
]

#pagebreak()

= Aussehen

#blockst[
  #import scratch.de: *
  
  #sage-fuer-sekunden("Hallo!")
  #sage("Hallo!")
  #denke-fuer-sekunden("Hmm...")
  #denke("Hmm...")
  
  #wechsle-zu-kostuem("Kostüm1")
  #naechstes-kostuem()
  #wechsle-zu-buehnenbild("Bühnenbild1")
  #naechstes-buehnenbild()
  
  #aendere-groesse()
  #setze-groesse()
  #aendere-effekt("Farbe")
  #setze-effekt("Farbe")
  #schalte-grafikeffekte-aus()
  #zeige-dich()
  #verstecke-dich()
  #gehe-zu-ebene("vorderste")
  #gehe-ebenen("nach vorne")
  #kostuem-eigenschaft("Nummer")
  #buehnenbild-eigenschaft("Nummer")
  #groesse()
]

#pagebreak()

= Klang

#blockst[
  #import scratch.de: *
  
  #spiele-klang("Miau")
  #spiele-klang-ganz("Miau")
  #stoppe-alle-klaenge()
  #aendere-klangeffekt("Höhe")
  #setze-klangeffekt("Höhe")
  #schalte-klangeffekte-aus()
  #aendere-lautstaerke()
  #setze-lautstaerke()
  #lautstaerke()
]

#pagebreak()

= Ereignisse

#blockst[
  #import scratch.de: *
  
  #wenn-gruene-flagge-geklickt([])
  #wenn-taste-gedrueckt("Leertaste", [])
  #wenn-diese-figur-angeklickt([])
  #wenn-buehnenbildwechsel("Bühnenbild1", [])
  #wenn-ueberschreitet("Lautstärke", "10", [])
  #wenn-nachricht-empfangen("Nachricht1", [])
  
  #sende-nachricht("Nachricht1")
  #sende-nachricht-und-warte("Nachricht1")
]

#pagebreak()

= Steuerung

#blockst[
  #import scratch.de: *
  
  #warte()
  #wiederhole(anzahl: 10, [])
  #wiederhole-fortlaufend([])
  #falls([], [])
  #falls-sonst([], [], [])
  #warte-bis([])
  #wiederhole-bis([], [])
  #stoppe("alles")
  #wenn-ich-als-klon-entstehe([])
  #erzeuge-klon("ich selbst")
  #loesche-diesen-klon()
]

#pagebreak()

= Fühlen

#blockst[
  #import scratch.de: *
  
  #wird-beruehrt("Mauszeiger")
  #wird-farbe-beruehrt(rgb(255, 0, 0))
  #farbe-beruehrt-farbe(rgb(255, 0, 0), rgb(0, 0, 255))
  #entfernung-von("Mauszeiger")
  #frage("Wie heißt du?")
  #antwort()
  #taste-gedrueckt("Leertaste")
  #maustaste-gedrueckt()
  #maus-x()
  #maus-y()
  #setze-ziehbarkeit("ziehbar")
  #lautstaerke-fuehlen()
  #stoppuhr()
  #setze-stoppuhr-zurueck()
  #eigenschaft-von("x-Position", "Figur1")
  #aktuell("Jahr")
  #tage-seit-2000()
  #benutzername()
]

#pagebreak()

= Operatoren

#blockst[
  #import scratch.de: *
  
  #addiere(1, 2)
  #subtrahiere(5, 3)
  #multipliziere(2, 3)
  #dividiere(10, 2)
  #zufallszahl()
  #groesser-als(5, 3)
  #kleiner-als(3, 5)
  #gleich(5, 5)
  #und(groesser-als(5, 3), kleiner-als(5, 10))
  #oder(groesser-als(5, 3), kleiner-als(3, 5))
  #nicht(groesser-als(5, 3))
  #verbinde("Hallo ", "Welt")
  #zeichen-von(1, "Hallo")
  #laenge-von("Hallo")
  #enthaelt("Hallo", "a")
  #modulo(10, 3)
  #runde(3.7)
  #mathematik("Betrag", -10)
]

#pagebreak()

= Variablen & Listen

#blockst[
  #import scratch.de: *
  
  #setze-variable("meine Variable", 0)
  #aendere-variable("meine Variable", 1)
  #zeige-variable("meine Variable")
  #verstecke-variable("meine Variable")
  #eigene-eingabe("meine Variable")
  
  #v(1em)
  
  #fuege-zu-liste-hinzu("Ding", "meine Liste")
  #entferne-aus-liste(1, "meine Liste")
  #entferne-alles-aus-liste("meine Liste")
  #fuege-bei-ein("Ding", 1, "meine Liste")
  #ersetze-element(1, "meine Liste", "Neues Ding")
  #element-von-liste(1, "meine Liste")
  #nummer-von-element("Ding", "meine Liste")
  #laenge-von-liste("meine Liste")
  #liste-enthaelt("meine Liste", "Ding")
  #zeige-liste("meine Liste")
  #verstecke-liste("meine Liste")
]

#pagebreak()

= Meine Blöcke

#blockst[
  #import "scratch.typ": eigener-block, definiere
  
  #let eigener = eigener-block("Zeichne ein", none, "-Eck")
  
  #definiere(eigener, [])
  #eigener(5)
]
