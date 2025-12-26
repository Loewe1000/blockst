#import "lib.typ": blockst, scratch,icons,set-blockst

#set page(height: auto, width: auto, margin: 1cm)
#set text(font: "Helvetica Neue",lang:"fre")
// #set text(font: "Atkinson Hyperlegible Mono")

#set-blockst(
  theme: "print", // ou "high-contrast" defaut : "normal"
  scale: 100%,    // Echelle des blocs
)

= Exemple 1 : Boucle finie


// #image(icons.pen)
#blockst[
  #import scratch.fr: *
  
  #when-flag-clicked[
    #play-sound-until-done("Meow")
    #broadcast-and-wait("test")
    #repeat(val: 100)[//steps:5
      #move()
    ]
  ]
]

#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    #jouer-bout("Meow")
    #envoyer-et-attendre("test")
    #répéter(val: 100)[//steps:5
      #avancer()
    ]
  ]
]

#pagebreak()

= Exemple 2 : Condition avec touches préssées
#blockst[
  #import scratch.fr: *
  
  #when-key-pressed("flèche haut")[
    #if-then-else(
      key-pressed("flèche haut"),
      [#move()
       #goto("position aléatoire")
       #glide(val:5,"position aléatoire")
      ],
      [#turn-right()],
    )
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-touche("flèche haut")[
    #si-alors-sinon(
      touche("flèche haut"),
      [#avancer()
       #aller("position aléatoire")
       #glisser(val:5,"position aléatoire")
      ],
      [#tourner-à-droite()],
    )
  ]
]

#pagebreak()

= Exemple 3 : Changer une variable
#blockst[
  #import scratch.fr: *
  
  #when-sprite-clicked[
    #set-variable-to("score", 0)
    #change-variable-by("score", 10)
    #show-variable("score")
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    #mettre-variable("score", 0)
    #ajouter-variable("score", 10)
    #montrer-variable("score")
  ]
]

#pagebreak()

= Exemple 4 : Remplir une liste
#blockst[
  #import scratch.fr: *
  
  #when-flag-clicked[
    #delete-all-of-list("noms")
    #add-to-list("Anna", "noms")
    #add-to-list("Ben", "noms")
    #delete-of-list("Anna", "noms")
    #show-list("noms")
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    #supprimer-la-liste("noms")
    #ajouter-liste("Anna", "noms")
    #ajouter-liste("Ben", "noms")
    #supprimer-de-liste("Anna", "noms")
    #montrer-liste("noms")
  ]
]

#pagebreak()

= Exemple 5 : Conditions imbriquées
#blockst[
  #import scratch.fr: *
  
  #when-flag-clicked[
    #if-then-else(
      op-and(
        greater-than(mouse-x(), 0),
        less-than(mouse-y(), 100),
      ),
      [#say-for-secs("Souris dans la zone!", val: 2)
       #switch-costume-to("costume2")
       #change-effect-by("color")
       ],
      [#say-for-secs("En dehors", val: 2)],
    )
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    #si-alors-sinon(
      intersection(
        supérieur(souris-x(), 0),
        inférieur(souris-y(), 100),
      ),
      [#dire-pendant("Souris dans la zone!", val: 2)
       #changer-costume("costume2")
       #ajouter-effet("color",val:10)
       ],
      [#dire-pendant("En dehors", val: 2)],
    )
  ]
]

#pagebreak()

= Exemple 6 : Faire des calculs
Demonstration des blocs de calcul (visuellement seulement).

#blockst[
  #import scratch.fr: *
  
  #when-flag-clicked[
    #point-in-direction(val: 90) 
    #set-variable-to("résultat", add(multiply(3, 2), 5))
    #say-for-secs(custom-input("résultat"), val: 2)
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    #orienter-à(val: 90) 
    #mettre-variable("résultat", addition(multiplication(3, 2), 5))
    #dire-pendant(saisie-perso("résultat"), val: 2)
  ]
]

#pagebreak()

= Exemple 7 : Toucher une couleur
#blockst[
  #import scratch.fr: *
  
  #when-flag-clicked[
    #repeat(val: 50)[
      #move()
      #wait(val:5)
      #if-then(
        touching-color(rgb("#FF0000")),
        [#turn-left(val: 180)],
      )
    ]
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    #répéter(val: 50)[
      #avancer()
      #attendre(val:5)
      #si-alors(
        toucher-couleur(rgb("#FF0000")),
        [#tourner-à-gauche(val: 180)],
      )
    ]
  ]
]

#pagebreak()
= Exemple 8 : Utilisation des blocs stylo

#blockst[
  #import scratch.fr: *
  
  #when-flag-clicked[
    // #point-in-direction(direction: 90) 
    #pen-erase-all()
    // #pen-stamp()
    #pen-down()
    #pen-set-color(blue)
    #pen-change-component-by(val:5,component:"luminosité")
    #pen-set-component-to(component:"saturation")
    #pen-change-size-by()
    // #pen-set-size-to()
    #repeat(val:5)[
      #move()
      #turn-right(val:72)
    ]
    #pen-up()
  ]
]
#blockst[
  #import scratch.fr: *
  
  #quand-drapeau[
    // #point-in-direction(direction: 90) 
    #effacer-tout()
    // #pen-stamp()
    #écrire()
    #choisir-couleur(blue)
    #ajouter-stylo(val:5,component:"luminosité")
    #mettre-stylo(component:"saturation")
    #ajouter-taille()
    // #pen-set-size-to()
    #répéter(val:5)[
      #avancer()
      #tourner-à-droite(val:72)
    ]
    #relever()
  ]
]

#pagebreak()

= Exemple 9 : Bloc personnalisé avec des paramètres

#blockst[
  #import scratch.fr: *
  
  #let jump = custom-block("sauter", (name: "Count"), "fois")
  
  #define(jump)[
    #repeat(val: parameter("count"))[
      #change-y(y: 10)
    ]
  ]
  
  #when-flag-clicked[
    #jump(5)
  ]
]
#blockst[
  #import scratch.fr: *
  
  #let sauter = bloc-perso("sauter", (name: "Count"), "fois")
  
  #définir(sauter)[
    #repeat(val: parameter("count"))[
      #ajouter-y(y: 10)
    ]
  ]
  
  #quand-drapeau[
    #sauter(5)
  ]
]

// #pagebreak()

// = Exemple 10 : Using sound
// #blockst[
//   #import scratch.fr: *
  
//   #when-flag-clicked[
//     #play-sound-until-done("Meow")
//     #repeat(val: 50)[
//       #ask-and-wait("Quand ?")
//       // #pick-random(from:2,to:100)
//       #if-then(
//         touching-color(rgb("#FF0000")),
//         [#turn-right(val: 180)],
//       )
//     ]
//   ]
// ]
