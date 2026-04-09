// Example – parseur de texte français
#import "../lib.typ": blockst, scratch

#set page(width: auto, height: auto, margin: 3mm, fill: white)

#blockst[
  #import scratch.text.fr: *

  #render-scratch-text("quand drapeau est cliqué
répéter 4 fois
avancer de 40 pas
tourner droite de 90 degrés
fin
si <touche [pointeur de souris v] ?> alors
dire [Bonjour !] pendant 1 secondes
sinon
dire [Je continue]
fin
fin")
]
