#import "@preview/blockst:0.2.1": blockst, scratch,

#set page(width: 10cm, height: auto, margin: 5mm, fill: none)

#v(2mm)

#grid(
  columns: (1fr, auto),
  gutter: 6mm,
  [*Step 1*\
  Trigger the script and walk forward.],
  [#scratch(line-numbers: true, "when green flag clicked\nmove (20) steps")],
  [*Step 2*\
  Repeat a square movement.],
  [#scratch(line-numbers: true, "when green flag clicked\nrepeat (4)\nmove (40) steps\nturn cw (90) degrees\nend")],
)

#grid(columns: 2)[
  scale: 100%
  #scratch(inset-scale:100%, "
quand @greenFlag est cliqué
          mettre [i v] à (0)
          relever le stylo
          effacer tout
          aller à x:(0) y: (0)
          stylo en position d'écriture
          s'orienter à (90)
          répéter (12) fois
          ajouter (5) à [i v] ::variables
          avancer de (i) pas
          tourner droite de (90) degrés
          avancer de (i) pas
          tourner droite de (90) degrés
          end
", language: "fr")
][
  scale: 10%
  #scratch(inset-scale:10%, "
quand @greenFlag est cliqué
          mettre [i v] à (0)
          relever le stylo
          effacer tout
          aller à x:(0) y: (0)
          stylo en position d'écriture
          s'orienter à (90)
          répéter (12) fois
          ajouter (5) à [i v] ::variables
          avancer de (i) pas
          tourner droite de (90) degrés
          avancer de (i) pas
          tourner droite de (90) degrés
          end
", language: "fr")
]