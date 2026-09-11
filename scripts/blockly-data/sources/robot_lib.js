/*blocklyRoboy_lib-1.0.0 by Arthur Léonard*/

/*TODO : un changement de taille de la fenetre à la fin d'une execution duplique les items*/
/*TODO : traduire le context wiring*/
var robotCommands = [];
// var imgPath = modulesPath+"img/algorea/";
var BLOCKLY_ROBOT_VERSION = '1.1';

var initArray = function(n, elem) {
   var res = [];
   for(var i = 0;i < n;i++) {
      res.push(elem);
   }
   return res;
}

var getContext = function(display, infos, curLevel) {
   var localLanguageStrings = {
      fr: {
         label: {},
         code: {},
         messages: {},
         description: {}
      },
      en: {
         label: {},
         code: {},
         messages: {},
         description: {}
      },
      es: {
         label: {},
         code: {},
         messages: {},
         description: {}
      },
      it: {
         label: {},
         code: {},
         messages: {},
         description: {}
      },
      de: {
        label: {},
        code: {},
        messages: {},
        description: {}
      }
   };
   
   var contextStrings = {
      none: {
         fr: {
            label: {
               row: "ligne du robot",
               col: "colonne du robot",
               wait: "attendre",
               north: "avancer vers le nord",
               south: "avancer vers le sud",
               east: "avancer vers l'est",
               west: "avancer vers l'ouest",
               left: "pivoter à gauche",
               right: "pivoter à droite",
               turnAround: "faire demi-tour",
               forward: "avancer",
               backwards: "reculer",
               jump: "sauter",
               obstacleInFront: "obstacle devant",
               obstacleEast: "obstacle à l'est",
               obstacleWest: "obstacle à l'ouest",
               obstacleNorth: "obstacle au nord",
               obstacleSouth: "obstacle au sud",
               obstacleRight: "obstacle à droite",
               obstacleLeft: "obstacle à gauche",
               gridEdgeEast: "bord de la grille à l'est",
               gridEdgeWest: "bord de la grille à l'ouest",
               gridEdgeNorth: "bord de la grille au nord",
               gridEdgeSouth: "bord de la grille au sud",
               platformInFront: "plateforme devant",
               platformAbove: "plateforme au-dessus",
               withdrawObject: "ramasser l'objet",
               dropObject: "déposer l'objet",
               onObject: "sur un objet",
               onContainer: "sur un conteneur",
               onExit: "sur une sortie",
               onNumber: "sur un nombre",
               onWritable: "sur un tableau",
               onLauncher: "sur un lanceur laser",
               writeNumber: "écrire le nombre",
               readNumber: "nombre de la case",
               pushObject: "pousser l'objet",
               pushableInFront: "poussable devant",
               dropNum: "déposer %1 objets",
               dropNum_noShadow: "déposer %1 objets",
               nbWithdrawables: "nombre d'objets sur la case",
               nbInBag: "nombre d'objets dans le sac",
               containerSize: "nombre d'objets à déposer sur la case",
               withdrawNum: "ramasser %1 objets",
               withdrawNum_noShadow: "ramasser %1 objets",
               shoot: "tirer au laser dans la direction %1",
               shoot_noShadow: "tirer au laser dans la direction %1",
               shootCondition: "retour départ tir direction %1",
               shootCondition_noShadow: "retour départ tir direction %1",
               connect: "brancher un câble",
               onMale: "sur une prise mâle",
               onFemale: "sur une prise femelle",
               dropPlatformInFront: "construire une plateforme devant",
               dropPlatformAbove: "construire une plateforme au-dessus"
               
            },
            code: {
               row: "ligneRobot",
               col: "colonneRobot",
               wait: "attendre",
               north: "nord",
               south: "sud",
               east: "est",
               west: "ouest",
               left: "pivoterGauche",
               right: "pivoterDroite",
               turnAround: "demiTour",
               forward: "avancer",
               backwards: "reculer",
               jump: "sauter",
               obstacleInFront: "obstacleDevant",
               obstacleEast: "obstacleEst",
               obstacleWest: "obstacleOuest",
               obstacleNorth: "obstacleNord",
               obstacleSouth: "obstacleSud",
               obstacleRight: "obstacleDroite",
               obstacleLeft: "obstacleGauche",
               gridEdgeEast: "bordGrilleEst",
               gridEdgeWest: "bordGrilleOuest",
               gridEdgeNorth: "bordGrilleNord",
               gridEdgeSouth: "bordGrilleSud",
               platformInFront: "plateformeDevant",
               platformAbove: "plateformeDessus",
               withdrawObject: "ramasserObjet",
               dropObject: "deposerObjet",
               onObject: "surObjet",
               onContainer: "surConteneur",
               onNumber: "surNombre",
               onWritable: "surTableau",
               onLauncher: "surLanceur",
               writeNumber: "ecrireNombre",
               readNumber: "nombreSurCase",
               pushObject: "pousserObjet",
               pushableInFront: "poussableDevant",
               dropNum: "deposer",
               dropNum_noShadow: "deposer",
               containerSize: "nbObjetsADeposer",
               nbInBag: "nbObjetsDansSac",
               nbWithdrawables: "nbObjetsSur",
               withdrawNum: "ramasser",
               withdrawNum_noShadow: "ramasser",
               shoot: "tirerLaser",
               shoot_noShadow: "tirerLaser",
               shootCondition: "tirerCondition",
               shootCondition_noShadow: "tirerCondition",
               connect: "brancherCable",
               onMale: "surMale",
               onFemale: "surFemelle",
               dropPlatformInFront: "construirePlateformeDevant",
               dropPlatformAbove: "construirePlateformeAuDessus"
            },
            description: {
               forward: "avancer() fait avancer le robot d'une case",
               backwards: "reculer() fait reculer le robot d'une case",
               left: "pivoterGauche() fait pivoter le robot d'un quart de tour à gauche",
               right: "pivoterDroite() fait pivoter le robot d'un quart de tour à droite",
               row: "ligneRobot() retourne le numéro de la ligne du robot",
               col: "colonneRobot() retourne le numéro de la colonne du robot",
               turnAround: "demiTour() fait faire demi-tour au robot",
               east: "est() déplace le robot d'une case vers l'est",
               west: "ouest() déplace le robot d'une case vers l'ouest",
               north: "nord() déplace le robot d'une case vers le nord",
               south: "sud() déplace le robot d'une case vers le sud",
               shoot: "tirerLaser(direction) fait tirer un rayon laser au robot, dans la direction indiquée en paramètre",
               shoot_noShadow: "tirerLaser() fait tirer un rayon laser au robot, dans la direction indiquée en paramètre",
               readNumber: "nombreSurCase() capteur qui lit le nombre sur la case où se trouve le robot",
               writeNumber: "ecrireNombre(nombre) écrit le nombre en paramètre sur la case où se trouve le robot",
               withdrawObject: "ramasserObjet() ramasse l'objet sur la case du robot",
               dropObject: "deposerObjet() dépose l'objet sur la case du robot",
               platformInFront: "plateformeDevant() teste s'il y a une plateforme devant le robot",
               platformAbove: "plateformeDessus() teste s'il y a une plateforme au-dessus du robot",
               dropPlatformInFront: "construirePlateformeDevant() construit une plateforme sur la case devant le robot",
               jump: "sauter() fait sauter le robot sur la plateforme juste au-dessus de lui"
            },
            messages: {
               leavesGrid: "Le robot sort de la grille !",
               obstacle: "Le robot essaie de se déplacer sur un obstacle !",
               nothingToPickUp: "Il n'y a rien à ramasser !",
               nothingToLookAt: "Il n'y a ni carte ni conteneur sur cette case",
               falls: "Le robot va se jeter dans le vide !",
               willFallAndCrash: "Le robot va tomber de haut et s'écraser !",
               jumpOutsideGrid: "Le robot essaie de sauter en dehors de la grille !",
               jumpObstacleBlocking: "Le robot essaie de sauter mais il y a un obstacle qui le bloque",
               jumpNoPlatform: "Le robot essaie de sauter mais il n'y a pas de plateforme au dessus !",
               tooManyObjects: "Le robot essaie de transporter trop d'objets à la fois !",
               emptyBag: "Le robot essaie de déposer un objet alors qu'il n'en transporte pas !",
               successReachExit: "Bravo, votre robot a atteint la sortie !",
               failureReachExit: "Votre robot n'a pas atteint la sortie.",
               successPickedAllWithdrawables: "Bravo, votre robot a tout ramassé !",
               failurePickedAllWithdrawables: "Votre robot n'a pas tout ramassé.",
               successContainersFilled: "Bravo, votre robot a rempli tous les conteneurs",
               failureContainersFilled: "Il y a un objet hors des conteneurs",
               failureContainersFilledLess: "Votre robot n'a pas rempli tous les conteneurs",
               failureContainersFilledBag: "Votre robot n'a pas posé tous les objets",
               failureUnfilteredObject: "Votre robot a ramassé un objet invalide",
               failureTooManyMoves: "Votre robot a effectué trop de déplacements.",
               failureWriteHere: "Votre robot ne peut pas écrire ici !",
               failureReadHere: "Il n'y a pas de nombre écrit ici !",
               successNumbersWritten: "Bravo, votre robot a écrit les bons nombres !",
               failureNumbersWritten: "Votre robot n'a pas écrit les bons nombres !",
               failureMissingNumber: "Le robot n'a pas écrit tous les nombres.",
               failureNothingToPush: "Il n'y a pas d'objet à pousser !",
               failureWhilePushing: "Le robot ne peut pas pousser cet objet !",
               failureDropObject: "On ne peut pas poser d'objet ici",
               failureDropPlatform: "Il y a déjà une plateforme ici",
               failureDropWater: "Il y a déjà de l'eau ici",
               failureDropToken: "Il y a déjà un jeton ici",
               failureDropOutside: "Votre robot essaie de poser un objet hors de la grille",
               failureNotEnoughPlatform: "Pas assez de plateformes",
               failureLights: "Il reste des spots à allumer.",
               successLights: "Bravo, votre robot a allumé tous les spots !",
               failureLaser: "Le robot doit se trouver sur une borne laser pour pouvoir tirer !",
               failureNoPlug: "Le robot doit se trouver sur une prise pour pouvoir brancher un câble !",
               failureAlreadyWired: "Cette prise est déjà connectée à un câble !",
               failureWrongPlugType: "On ne peut pas connecter ces prises ensemble !",
               successPlugsWired: "La machine est réparée !",
               failurePlugsWired: "La machine ne fonctionne pas car des prises n'ont pas été connectées !",
               failureWireCrossing: "Impossible de relier ces deux prises, deux câbles vont s'intersecter !",
               failureWireTooLong: "Impossible de relier ces deux prises car elles sont trop éloignées !",
               failureTotalLengthExceeded: "Vous n'avez pas assez de longueur de câble pour relier ces deux prises !",
               failureProjectile: "Le robot s'est pris un projectile !",
               failureRewrite: "Le robot a essayé de repeindre une case.",
               noContainer: "Il n'y a pas de conteneur ici !",
               failureLineBreak: "Le robot ne peut pas changer de ligne.",
               failureFuse: "Le robot a tiré sur un fusible !"
            },
            cardinals: {
               north: "Nord",
               south: "Sud",
               west: "Ouest",
               east: "Est"
            },
            startingBlockName: "Programme du robot"
         },
         en: {
            label: {
               row: "robot's row",
               col: "robot's column",
               wait: "wait",
               north: "move up",
               south: "move down",
               east: "move right",
               west: "move left",
               left: "turn left",
               right: "turn right",
               turnAround: "turn around",
               forward: "move forward",
               backwards: "move backwards",
               jump: "jump",
               obstacleInFront: "obstacle ahead",
               obstacleEast: "obstacle on the right",
               obstacleWest: "obstacle on the left",
               obstacleNorth: "obstacle above",
               obstacleSouth: "obstacle below",
               obstacleRight: "obstacle on the right",
               obstacleLeft: "obstacle on the left",
               gridEdgeEast: "grid edge on the right",
               gridEdgeWest: "grid edge on the left",
               gridEdgeNorth: "grid edge above",
               gridEdgeSouth: "grid edge below",
               platformInFront: "platform ahead",
               platformAbove: "platform above",
               withdrawObject: "pick the object",
               dropObject: "drop the object",
               onObject: "on an object",
               onContainer: "on a container",
               onNumber: "on a number",
               onWritable: "on a blackboard",
               onLauncher: "on a laser emitter",
               writeNumber: "write the number",
               readNumber: "number in the cell",
               pushObject: "push the object",
               pushableInFront: "pushable object ahead",
               shoot: "shoot a laser in direction %1",
               shoot_noShadow: "shoot a laser in direction %1",
               shootCondition: "laser shot returning to starting point in direction %1",
               shootCondition_noShadow: "laser shot returning to starting point in direction %1",
               connect: "plug a wire",
               onMale: "to a male plug",
               onFemale: "to a female plug",
               dropPlatformInFront: "drop platform in front",
               dropPlatformAbove: "drop platform above"
               
            },
            code: {
               row: "robotRow",
               col: "robotColumn",
               wait: "wait",
               north: "up",
               south: "down",
               east: "right",
               west: "left",
               left: "turnLeft",
               right: "turnRight",
               turnAround: "turnAround",
               forward: "forward",
               backwards: "backwards",
               jump: "jump",
               obstacleInFront: "obstacleAhead",
               obstacleEast: "obstacleRight",
               obstacleWest: "obstacleLeft",
               obstacleNorth: "obstacleAbove",
               obstacleSouth: "obstacleBelow",
               obstacleRight: "obstacleRightRel",
               obstacleLeft: "obstacleLeftRel",
               gridEdgeEast: "gridEdgeRight",
               gridEdgeWest: "gridEdgeLeft",
               gridEdgeNorth: "gridEdgeAbove",
               gridEdgeSouth: "gridEdgeBelow",
               platformInFront: "plateformAhead",
               platformAbove: "platformAbove",
               withdrawObject: "pickObject",
               dropObject: "dropObject",
               onObject: "onObject",
               onContainer: "onContainer",
               onNumber: "onNumber",
               onWritable: "onWritable",
               onLauncher: "onLauncher",
               writeNumber: "writeNumber",
               readNumber: "numberOnCell",
               pushObject: "pushObject",
               pushableInFront: "pushableAhead",
               shoot: "shootLaser",
               shoot_noShadow: "shootLaser",
               shootCondition: "shootOnCondition",
               shootCondition_noShadow: "shootOnCondition",
               connect: "plugCable",
               onMale: "onMalePlug",
               onFemale: "onFemalePlug",
               dropPlatformInFront: "dropPlatformInFront",
               dropPlatformAbove: "dropPlatformAbove"
            },
            messages: {
               leavesGrid: "The robot exits the grid!",
               obstacle: "The robot attempts to move towards an obstacle!",
               nothingToPickUp: "There is nothing to puck up!",
               nothingToLookAt: "There is no card or container in this cell",
               falls: "The robot will leap into the void",
               willFallAndCrash: "The robot will jump from a high point and crash!",
               jumpOutsideGrid: "The robot tries to jump outside of the grid!",
               jumpObstacleBlocking: "The robot tries to jump but an obstacle blacks it",
               jumpNoPlatform: "The robot tries to jump but there is no platform above!",
               tooManyObjects: "The robot tries to transport too many objects at a time!",
               emptyBag: "The robot tries to drop an object but it doesn't carry one!",
               successReachExit: "Congratulations, your robot reached the exit!",
               failureReachExit: "Your robot didn't reach the exit.",
               successPickedAllWithdrawables: "Congratulations, your robot picked up everything!",
               failurePickedAllWithdrawables: "Your robot didn't pick up everything.",
               successContainersFilled: "Congratulations, your robot filled every container",
               failureContainersFilled: "An object was left outside of containers",
               failureContainersFilledLess: "Yout robot didn't fill every container",
               failureContainersFilledBag: "Your robot didn't drop all the objects",
               failureUnfilteredObject: "Your robot picked an invalid object",
               failureTooManyMoves: "Your robot made too many moves.",
               failureWriteHere: "Your robot can't write here!",
               failureReadHere: "There is no number written here!",
               successNumbersWritten: "Congratulations, your robot wrote all the correct numbers!",
               failureNumbersWritten: "Your robot didn't write the correct numbers!",
               failureNothingToPush: "There is no object to push!",
               failureWhilePushing: "The robot can't push this object!",
               failureDropObject: "You can't drop an platform here",
               failureDropPlatform: "You can't drop an object here",
               failureDropOutside: "Your robot tries to drop an object outside of the grid",
               failureNotEnoughPlatform: "Not enough platforms",
               failureLights: "There are still lights to turn on.",
               successLights: "Congratulations, your robot turned on all the lights!",
               failureLaser: "The robot has to be on a launcher to be able to shoot!",
               failureNoPlug: "The robot has to be on a plug to plug in a cable!",
               failureAlreadyWired: "This plug is already connected to a cable!",
               failureWrongPlugType: "You can't connect these plugs together!",
               successPlugsWired: "The machine is fixed!",
               failurePlugsWired: "The machine doesn't work because some plugs are not connected!",
               failureWireCrossing: "Impossible to connect these two plugs: two cables would intersect!",
               failureWireTooLong: "Impossible to connect these two plugs: they are too far from eachother!",
               failureTotalLengthExceeded: "You don't have enough length of cable to connect these two plugs!",
               failureProjectile: "The robot got hit by a projectile!"
            },
            cardinals: {
               north: "North",
               south: "South",
               west: "West",
               east: "East"
            },
            startingBlockName: "Program of the robot"
         },
         
         es: {
            label: {
               row: "fila del robot",
               col: "columna del robot",
               north: "avanzar hacia arriba",
               south: "avanzar hacia abajo",
               east: "avanzar hacia la derecha",
               west: "avanzar hacia la izquierda",
               left: "girar a la izquierda",
               right: "girar a la derecha",
               turnAround: "dar media vuelta",
               forward: "avanzar",
               backwards: "retroceder",
               jump: "saltar",
               obstacleInFront: "obstáculo adelante",
               obstacleEast: "obstáculo a la derecha",
               obstacleWest: "obstáculo a la izquierda",
               obstacleNorth: "obstáculo arriba",
               obstacleSouth: "obstáculo abajo",
               obstacleRight: "obstáculo a la derecha",
               obstacleLeft: "obstáculo a la izquierda",
               gridEdgeEast: "borde de la cuadrícula a la derecha",
               gridEdgeWest: "borde de la cuadrícula a la izquierda",
               gridEdgeNorth: "borde de la cuadrícula arriba",
               gridEdgeSouth: "borde de la cuadrícula abajo",
               platformInFront: "plataforma adelante",
               platformAbove: "plataforma arriba",
               withdrawObject: "recoger el objeto",
               dropObject: "soltar el objeto",
               onObject: "sobre un objeto",
               onContainer: "sobre un contenedor",
               onNumber: "sobre un número",
               onWritable: "sobre un cuadro",
               onLauncher: "sobre un lanzador láser",
               writeNumber: "escribir el número",
               readNumber: "número en la casilla",
               pushObject: "empujar el objeto",
               pushableInFront: "objeto empujable adelante",
               shoot: "disparar el láser en la dirección %1",
               shoot_noShadow: "disparar el laser en la dirección %1",
               shootCondition: "dirección del tiro de retorno %1",
               shootCondition_noShadow: "dirección del tiro de retorno %1",
               dropPlatformInFront: "construir una plataforma adelante",
               dropPlatformAbove: "construir una plataforma arriba"
            },
            code: {
               row: "filaRobot",
               col: "columnaRobot",
               north: "arriba",
               south: "abajo",
               east: "derecha",
               west: "izquierda",
               left: "girarIzquierda",
               right: "girarDerecha",
               turnAround: "mediaVuelta",
               forward: "avanzar",
               backwards: "retroceder",
               jump: "saltar",
               obstacleInFront: "obstaculoAdelante",
               obstacleRight: "obstaculoDerechaRel",
               obstacleLeft: "obstaculoIzquierdaRel",
               obstacleEast: "obstaculoDerecha",
               obstacleWest: "obstaculoIzquierda",
               obstacleNorth: "obstaculoArriba",
               obstacleSouth: "obstaculoAbajo",
               gridEdgeEast: "bordeCuadriculaDerecha",
               gridEdgeWest: "bordeCuadriculaIzquierda",
               gridEdgeNorth: "bordeCuadriculaArriba",
               gridEdgeSouth: "bordeCuadriculaAbajo",
               platformInFront: "plataformaAdelante",
               platformAbove: "plataformaArriba",
               withdrawObject: "recogerObjeto",
               dropObject: "soltarObjeto",
               onObject: "sobreObjeto",
               onContainer: "sobreContenedor",
               onNumber: "sobreNumero",
               onWritable: "sobreCuadro",
               onLauncher: "sobreLanzador",
               writeNumber: "escribirNumero",
               readNumber: "leerNumero",
               pushObject: "empujarObjeto",
               pushableInFront: "empujableAdelante",
               shoot: "dispararLaser",
               shoot_noShadow: "dispararLaser",
               shootCondition: "condicionDisparo",
               shootCondition_noShadow: "condicionDisparo",
               dropPlatformInFront: "construirPlataformaAdelante",
               dropPlatformAbove: "construirPlataformaArriba"
            },
            messages: {
               leavesGrid: "¡El robot salió de la cuadrícula!",
               obstacle: "¡El robot intenta moverse sobre un obstáculo!",
               nothingToPickUp: "No hay algo para recoger",
               nothingToLookAt: "No hay carta ni contenedor en esta casilla",
               falls: "¡El robot va a caer al vacío!",
               willFallAndCrash: "¡El robot va a caer y chocar!",
               jumpOutsideGrid: "¡El robot intenta saltar fuera de la cuadrícula!",
               jumpObstacleBlocking: "El robot intenta saltar pero hay un obstáculo que lo bloquea",
               jumpNoPlatform: "¡El robot intenta saltar pero no hay una plataforma arriba!",
               tooManyObjects: "¡El robot intenta transportar demasiados objetos al mismo tiempo!",
               emptyBag: "El robot intenta soltar un objeto, ¡pero no carga nada!",
               successReachExit: "Bravo, ¡su robot a llegado a la salida!",
               failureReachExit: "Su robot no ha llegado a la salida.",
               successPickedAllWithdrawables: "Bravo, su robot ha recogido todo!",
               failurePickedAllWithdrawables: "Su robot no ha recogido todo.",
               successContainersFilled: "Bravo, su robot ha llenado todos los contenedores",
               failureContainersFilled: "Hay un objeto fuera de los contenedores",
               failureContainersFilledLess: "Su robot no ha llenado todos los contenedores",
               failureContainersFilledBag: "Su robot no ha puesto todos los objetos",
               failureUnfilteredObject: "Su robot ha recogido un objeto inválido",
               failureTooManyMoves: "Su robot ha realizado demasiados desplazamientos.",
               failureWriteHere: "¡Su robot no puede escribir aquí!",
               failureReadHere: "¡No hay un número aquí!",
               successNumbersWritten: "Bravo, su robot ha escrito los números correctos!",
               failureNumbersWritten: "Su robot no ha escrito los números correctos!",
               failureNothingToPush: "¡No hay un objeto que empujar!",
               failureWhilePushing: "¡El robot no puede empujar este objeto!",
               failureDropObject: "No es posible poner el objeto aquí",
               failureDropPlatform: "No es posible poner el objeto aquí",
               failureDropOutside: "Su robot intenta poner un objeto fuera de la cuadrícula",
               failureNotEnoughPlatform: "No hay suficiente plataforma",
               failureLights: "Aún faltan lugares que iluminar.",
               successLights: "Bravo, ¡su robot ha iluminado todos los lugares!",
               failureLaser: "¡El robot debe encontrarse sobre una terminal láser para poder disparar!"
            },
            cardinals: {
               north: "Norte",
               south: "Sur",
               west: "Oeste",
               east: "Este"
            },
            startingBlockName: "Programa del robot"
         },
         de: {
            label: {
               row: "Zeile des Roboters",
               col: "Spalte des Roboters",
               north: "gehe nach oben",
               south: "gehe nach unten",
               east: "gehe nach rechts",
               west: "gehe nach links",
               left: "drehe nach links",
               right: "drehe nach rechts",
               turnAround: "drehe um",
               forward: "gehe",
               backwards: "gehe rückwärts",
               jump: "springe",
               obstacleInFront: "vor Hindernis",
               obstacleEast: "Hindernis rechts",
               obstacleWest: "Hindernis links",
               obstacleNorth: "Hindernis oben",
               obstacleSouth: "Hindernis unten",
               obstacleRight: "Hindernis rechts",
               obstacleLeft: "Hindernis links",
               gridEdgeAbove: "unter Rand des Gitters",
               gridEdgeBelow: "über Rand des Gitters",
               gridEdgeEast: "links vom Gitterrand",
               gridEdgeWest: "rechts vom Gitterrand",
               platformInFront: "vor Plattform",
               platformAbove: "Plattform darüber",
               withdrawObject: "hebe Objekt auf",
               dropObject: "lege Objekt ab",
               onObject: "auf Objekt",
               onContainer: "auf Kiste",
               onNumber: "auf Zahl",
               onWritable: "auf Tafel",
               onLauncher: "sur un lanceur laser", // TODO :: translate
               writeNumber: "schreibe Zahl",
               readNumber: "Zahl auf dem Feld",
               pushObject: "schiebe Kiste",
               pushableInFront: "vor Kiste",
               shoot: "schieße Laser in Richtung %1",
               shoot_noShadow: "schieße Laser in Richtung %1",
               shootCondition: "Rückkehr von der Schießrichtung %1",
               shootCondition_noShadow: "Rückkehr von der Schießrichtung %1"
            },
            code: {
               row: "ligneRobot",
               col: "colonneRobot",
               north: "haut",
               south: "bas",
               east: "droite",
               west: "gauche",
               left: "pivoterGauche",
               right: "pivoterDroite",
               turnAround: "demiTour",
               forward: "avancer",
               backwards: "reculer",
               jump: "sauter",
               obstacleInFront: "obstacleDevant",
               obstacleEast: "obstacleDroite",
               obstacleWest: "obstacleGauche",
               obstacleNorth: "obstacleHaut",
               obstacleSouth: "obstacleBas",
               obstacleRight: "obstacleDroiteRel",
               obstacleLeft: "obstacleGaucheRel",
               gridEdgeEast: "bordGrilleDroite",
               gridEdgeWest: "bordGrilleGauche",
               gridEdgeNorth: "bordGrilleHaut",
               gridEdgeSouth: "bordGrilleBas",
               platformInFront: "plateformeDevant",
               platformAbove: "plateformeDessus",
               withdrawObject: "ramasserObjet",
               dropObject: "deposerObjet",
               onObject: "surObjet",
               onContainer: "surConteneur",
               onNumber: "surNombre",
               onWritable: "surTableau",
               onLauncher: "surLanceur", 
               writeNumber: "ecrireNombre",
               readNumber: "nombreSurCase",
               pushObject: "pousserObjet",
               pushableInFront: "poussableDevant",
               shoot: "tirerLaser",
               shoot_noShadow: "tirerLaser",
               shootCondition: "tirerCondition",
               shootCondition_noShadow: "tirerCondition"
            },
            messages: {
               leavesGrid: "Der Roboter hat das Gitter verlassen!",
               obstacle: "Der Roboter ist gegen ein Hindernis gelaufen!",
               nothingToPickUp: "An dieser Stelle gibt es nichts zum aufheben!",
               nothingToLookAt: "An dieser Stelle gibt es nichts zum betrachten!",
               falls: "Der Roboter fällt in den Abgrund!",
               willFallAndCrash: "Der Roboter würde hier runterfallen und kaputt gehen!",
               jumpOutsideGrid: "Der Roboter versucht, aus dem Gitter zu springen!",
               jumpObstacleBlocking: "Der Roboter kann hier nicht springen, weil ein Hindernis ihn blockiert.",
               jumpNoPlatform: "Der Roboter kann hier nicht springen, weil über ihm keine Plattform ist.",
               tooManyObjects: "Der Roboter kann nicht so viele Objekte auf einmal tragen!",
               emptyBag: "Der Roboter kann nichts ablegen, weil er gar nichts transportiert!",
               successReachExit: "Bravo! Der Roboter hat den Ausgang erreicht.",
               failureReachExit: "Der Roboter hat den Ausgang nicht erreich!",
               successPickedAllWithdrawables: "Bravo! Der Roboter hat alles eingesammelt.",
               failurePickedAllWithdrawables: "Der Roboter hat nicht alles eingesammelt!",
               successContainersFilled: "Gut gemacht! Der Roboter hat alle Behälter gefüllt.",
               failureContainersFilled: "Es befindet sich ein Objekt außerhalb der Behälter.",
               failureContainersFilledLess: "Der Roboter hat nicht alle Behälter gefüllt.",
               failureContainersFilledBag: "Der Roboter hat nicht alle Objekte platziert.",
               failureUnfilteredObject: "Dein Roboter hat ein nicht erlaubtes Objekt aufgehoben!",
               failureTooManyMoves: "Votre robot a effectué trop de déplacements.",
               failureWriteHere: "Der Roboter kann an dieser Stelle nicht schreiben!",
               failureReadHere: "An dieser Stelle steht keine Zahl!",
               successNumbersWritten: "Bravo! Der Roboter hat die richtigen Zahlen geschrieben.",
               failureNumbersWritten: "Dein Roboter hat nicht die richtigen Zahlen geschrieben!",
               failureNothingToPush: "An dieser Stelle gibt es nichts zum Schieben!",
               failureWhilePushing: "Der Roboter hat es nicht geschafft, das Objekt zu schieben!",
               failureDropObject: "An dieser Stelle kann kein Objekt abgelegt werden!",
               failureDropPlatform: "An dieser Stelle kann kein Objekt abgelegt werden!",
               failureDropOutside: "Der Roboter hat versucht ein Objekt vom Gitterrand zu schieben!",
               failureNotEnoughPlatform: "Nicht genügend Plattformen!",
               failureLights: "Der Roboter hat nicht alles beleuchtet!",
               successLights: "Bravo! Der Roboter hat alles beleuchtet.",
               failureLaser: "Der Roboter muss auf einem Laser stehen, um schießen zu können!",
            },
            cardinals: {
               north: "Norden",
               south: "Süden",
               west: "Westen",
               east: "Osten"
            },
            startingBlockName: "Roboter-Programm"
         },
         it: {
            label: {
               row: "ligne du robot",
               col: "colonne du robot",
               wait: "attendre",
               north: "andare avanti",
               south: "avanzare verso il basso",
               east: "avanza a destra",
               west: "avancer vers la gauche",
               left: "pivoter à gauche",
               right: "pivoter à droite",
               turnAround: "faire demi-tour",
               forward: "avancer",
               backwards: "reculer",
               jump: "sauter",
               obstacleInFront: "obstacle devant",
               obstacleEast: "obstacle à droite",
               obstacleWest: "obstacle à gauche",
               obstacleNorth: "obstacle en haut",
               obstacleSouth: "obstacle en bas",
               obstacleRight: "obstacle à droite",
               obstacleLeft: "obstacle à gauche",
               gridEdgeEast: "bord de la grille à droite",
               gridEdgeWest: "bord de la grille à gauche",
               gridEdgeNorth: "bord de la grille en haut",
               gridEdgeSouth: "bord de la grille en bas",
               platformInFront: "plateforme devant",
               platformAbove: "plateforme au-dessus",
               withdrawObject: "ramasser l'objet",
               dropObject: "dipingi la scatola",
               onObject: "su un marmo",
               onContainer: "su un buco",
               onNumber: "sur un nombre",
               onWritable: "sur un tableau",
               onLauncher: "sur un lanceur laser",
               writeNumber: "scrivere numero",
               readNumber: "numero sulla casella",
               pushObject: "pousser l'objet",
               pushableInFront: "poussable devant",
               shoot: "tirer au laser dans la direction %1",
               shoot_noShadow: "tirer au laser dans la direction %1",
               shootCondition: "retour départ tir direction %1",
               shootCondition_noShadow: "retour départ tir direction %1",
               connect: "brancher un câble",
               onMale: "sur une prise mâle",
               onFemale: "sur une prise femelle"
            },
            code: {
               row: "ligneRobot",
               col: "colonneRobot",
               wait: "attendre",
               north: "haut",
               south: "bas",
               east: "droite",
               west: "gauche",
               left: "pivoterGauche",
               right: "pivoterDroite",
               turnAround: "demiTour",
               forward: "avancer",
               backwards: "reculer",
               jump: "sauter",
               obstacleInFront: "obstacleDevant",
               obstacleEast: "obstacleDroite",
               obstacleWest: "obstacleGauche",
               obstacleNorth: "obstacleHaut",
               obstacleSouth: "obstacleBas",
               obstacleRight: "obstacleDroiteRel",
               obstacleLeft: "obstacleGaucheRel",
               gridEdgeEast: "bordGrilleDroite",
               gridEdgeWest: "bordGrilleGauche",
               gridEdgeNorth: "bordGrilleHaut",
               gridEdgeSouth: "bordGrilleBas",
               platformInFront: "plateformeDevant",
               platformAbove: "plateformeDessus",
               withdrawObject: "ramasserObjet",
               dropObject: "deposerObjet",
               onObject: "surObjet",
               onContainer: "surConteneur",
               onNumber: "surNombre",
               onWritable: "surTableau",
               onLauncher: "surLanceur",
               writeNumber: "ecrireNombre",
               readNumber: "nombreSurCase",
               pushObject: "pousserObjet",
               pushableInFront: "poussableDevant",
               shoot: "tirerLaser",
               shoot_noShadow: "tirerLaser",
               shootCondition: "tirerCondition",
               shootCondition_noShadow: "tirerCondition",
               connect: "brancherCable",
               onMale: "surMale",
               onFemale: "surFemelle"
            },
            messages: {
               leavesGrid: "Le robot sort de la grille !",
               obstacle: "Le robot essaie de se déplacer sur un obstacle !",
               nothingToPickUp: "Il n'y a rien à ramasser !",
               nothingToLookAt: "Il n'y a ni carte ni conteneur sur cette case",
               falls: "Le robot va se jeter dans le vide !",
               willFallAndCrash: "Le robot va tomber de haut et s'écraser !",
               jumpOutsideGrid: "Le robot essaie de sauter en dehors de la grille !",
               jumpObstacleBlocking: "Le robot essaie de sauter mais il y a un obstacle qui le bloque",
               jumpNoPlatform: "Le robot essaie de sauter mais il n'y a pas de plateforme au dessus !",
               tooManyObjects: "Le robot essaie de transporter trop d'objets à la fois !",
               emptyBag: "Le robot essaie de déposer un objet alors qu'il n'en transporte pas !",
               successReachExit: "Bravo, votre robot a atteint la sortie !",
               failureReachExit: "Votre robot n'a pas atteint la sortie.",
               successPickedAllWithdrawables: "Bravo, votre robot a tout ramassé !",
               failurePickedAllWithdrawables: "Votre robot n'a pas tout ramassé.",
               successContainersFilled: "Bravo, votre robot a rempli tous les conteneurs",
               failureContainersFilled: "Il y a un objet hors des conteneurs",
               failureContainersFilledLess: "Votre robot n'a pas rempli tous les conteneurs",
               failureContainersFilledBag: "Votre robot n'a pas posé tous les objets",
               failureUnfilteredObject: "Votre robot a ramassé un objet invalide",
               failureTooManyMoves: "Votre robot a effectué trop de déplacements.",
               failureWriteHere: "Votre robot ne peut pas écrire ici !",
               failureReadHere: "Il n'y a pas de nombre écrit ici !",
               successNumbersWritten: "Bravo, votre robot a écrit les bons nombres !",
               failureNumbersWritten: "Votre robot n'a pas écrit les bons nombres !",
               failureNothingToPush: "Il n'y a pas d'objet à pousser !",
               failureWhilePushing: "Le robot ne peut pas pousser cet objet !",
               failureDropObject: "On ne peut pas poser d'objet ici",
               failureDropPlatform: "On ne peut pas construire de plateforme ici",
               failureDropOutside: "Votre robot essaie de poser un objet hors de la grille",
               failureNotEnoughPlatform: "Pas assez de plateformes",
               failureLights: "Il reste des spots à allumer.",
               successLights: "Bravo, votre robot a allumé tous les spots !",
               failureLaser: "Le robot doit se trouver sur une borne laser pour pouvoir tirer !",
               failureNoPlug: "Le robot doit se trouver sur une prise pour pouvoir brancher un câble !",
               failureAlreadyWired: "Cette prise est déjà connectée à un câble !",
               failureWrongPlugType: "On ne peut pas connecter ces prises ensemble !",
               successPlugsWired: "La machine est réparée !",
               failurePlugsWired: "La machine ne fonctionne pas car des prises n'ont pas été connectées !",
               failureWireCrossing: "Impossible de relier ces deux prises, deux câbles vont s'intersecter !",
               failureWireTooLong: "Impossible de relier ces deux prises car elles sont trop éloignées !",
               failureTotalLengthExceeded: "Vous n'avez pas assez de longueur de câble pour relier ces deux prises !",
               failureProjectile: "Le robot s'est pris un projectile !",
               failureRewrite: "Le robot a essayé de repeindre une case."
            },
            cardinals: {
               north: "Nord",
               south: "Sud",
               west: "Ovest",
               east: "Est"
            },
            startingBlockName: "Programme du robot"
         },
      },
      arrows: {
         fr: {
            messages: {
               obstacle: "Le robot va sortir du parcours fléché !",
               successReachExit: "Bravo, votre robot a récupéré le coffre !",
               failureReachExit: "Votre robot s'est perdu en chemin."
            }
         },
         en: {
            messages: {
               obstacle: "The robot will get out of the marked track!",
               successReachExit: "Congratulations, your robot collected the safe!",
               failureReachExit: "Yout robot got lost."
            }
         },

         es: {
            messages: {
               obstacle: "¡El robot va a salirse del camino marcado!",
               successReachExit: "Bravo, ¡su robot ha recuperado el cofre!",
               failureReachExit: "Su robot se perdió en el camino."
            }
         }  
      },
      cards: {
         fr: {
            label: {
               withdrawObject: "ramasser la carte",
               dropObject: "déposer la carte",
               onObject: "sur une carte",
               onContainer: "sur un emplacement de dépôt",
            },
             messages: {
               successContainersFilled: "Bravo, votre robot a rangé les cartes au bon endroit !",
               successContainersFilledSingular: "Bravo, votre robot a rangé la carte au bon endroit !",
               failureContainersFilled: "Il y a des cartes mal rangées",
               failureContainersFilledLess: "Il y a encore des cartes à ranger.",
               failureContainersFilledBag: "Votre robot doit déposer sa carte."
            }
         },
         en: {
            label: {
               withdrawObject: "pick up the card",
               dropObject: "drop the card",
               onObject: "on a card",
               onContainer: "on a placeholder",
            },
             messages: {
               successContainersFilled: "Congratulations, your robot placed all the cards at the right location!",
               failureContainersFilled: "Some cards are misplaced",
               failureContainersFilledLess: "There are still misplaced cards.",
               failureContainersFilledBag: "Your robot must drop its card."
            }
         },

         es: {
            label: {
               withdrawObject: "recoger la carta",
               dropObject: "soltar la carta",
               onObject: "sobre una carta",
               onContainer: "en un sitio de depósito",
            },
             messages: {
               successContainersFilled: "Bravo, su robot ha ordenado las cartas en un buen lugar!",
               failureContainersFilled: "Hay cartas ordenadas incorrectamente.",
               failureContainersFilledLess: "Aún quedan cartas sin ordenar.",
               failureContainersFilledBag: "Su robot debe depositar su carta."
            }
         }
      },
      castle: {
         fr: {
            label: {
               withdrawObject: "ramasser le bois",
               dropObject: "déposer le bois",
               onObject: "sur du bois",
               onContainer: "sur une cheminée"
            },
            code: {
               withdrawObject: "ramasserBois",
               dropObject: "deposerBois",
               onObject: "surBois",
               onContainer: "surCheminee"
            },
            messages: {
               successContainersFilled: "Bravo, le château est bien chauffé !",
               successContainersFilledSingular: "Bravo, il y a un bon feu dans la cheminée !",
               failureContainersFilled: "Votre robot n'a pas mis du bois dans toutes les cheminées.",
               failureContainersFilledSingular: "Votre robot n'a pas mis le bois dans la cheminée.",
               failureContainersFilledLess: "Votre robot n'a pas mis du bois dans toutes les cheminées..",
               failureContainersFilledLessSingular: "Votre robot n'a pas mis le bois dans la cheminée.",
               failureContainersFilledBag: "Votre robot doit déposer le bois dans la cheminée.",
               failureDropOutside: "Votre robot essaie de construire une plateforme hors de la grille.",
               failureDropObject: "Le robot essaie de déploser du bois alors qu'il y en a déjà dans cette cheminée !",
               failureDropPlatform: "Il y a déjà une plateforme ici !",
               emptyBag: "Le robot essaie de déposer du bois alors qu'il n'en transporte pas !",
               tooManyObjects: "Le robot essaie de transporter trop de bois à la fois !"
            }
         },

         en: {
            label: {
               withdrawObject: "pick up the gear",
               dropObject: "accrocher la roue dentée",
               onObject: "sur une roue dentée",
               onContainer: "sur une machine"
            },
            code: {
               withdrawObject: "pickGear",
               dropObject: "attachGear",
               onObject: "onGear",
               onContainer: "onMachine"
            },
            messages: {
               successContainersFilled: "Bravo, les machines sont prêtes à fonctionner !",
               failureContainersFilled: "Votre robot n'a pas replacé toutes les roues dentées au bon endroit.",
               failureContainersFilledLess: "Votre robot n'a pas replacé toutes les roues dentées au bon endroit.",
               failureContainersFilledBag: "Votre robot doit déposer la roue dentée sur la machine.",
               failureDropOutside: "Votre robot essaie de construire une plateforme hors de la grille.",
               failureDropObject: "Il y a déjà une plateforme ici !",
               failureDropPlatform: "Il y a déjà une plateforme ici !",
               emptyBag: "Le robot essaie d'accrocher une roue dentée alors qu'il n'en transporte pas !"
            }
         },

         es: {
            label: {
               withdrawObject: "recoger el engranaje",
               dropObject: "soltar el engranaje",
               onObject: "sobre engranaje",
               onContainer: "sobre una máquina"
            },
            code: {
               withdrawObject: "recogerEngranaje",
               dropObject: "soltarEngranaje",
               onObject: "sobreEngranaje",
               onContainer: "sobreMaquina"
            },
            messages: {
               successContainersFilled: "Bravo, ¡la máquina está lista para funcionar!",
               failureContainersFilled: "Su robot no ha puesto todos los engranajes en el lugar correcto.",
               failureContainersFilledLess: "Su robot no ha puesto todos los engranajes en el lugar correcto.",
               failureContainersFilledBag: "Su robot debe colocar el engranaje en la máquina.",
               failureDropOutside: "Su robot intenta construir una plataforma fuera de la cuadrícula."
            } 
         }
      },
      chticode_abs: {
         fr: {
            label: {
               
            },
             messages: {
               successPickedAllWithdrawables: "Bravo, votre robot a réussi la mission !",
               failureReachExit: "Votre robot n'a pas atteint la case verte."
            }
         },
         en: {
            label: {
               
            },
             messages: {
               successPickedAllWithdrawables: "Congratulations, your robot succeeded this mission!",
               failureReachExit: "Your robot didn't reach the green cell."
            }
         },

         es: {
            label: {
               
            },
             messages: {
               successPickedAllWithdrawables: "Bravo, ¡su robot ha completado la misión!",
               failureReachExit: "Su robot no llegó a la casilla verde."
            }
         }
      },
      chticode_rel: {
         fr: {
            label: {
               
            },
             messages: {
               successReachExit: "Bravo, votre robot a atteint la case verte !",
               failureReachExit: "Votre robot n'a pas atteint la case verte."
            }
         },
         en: {
            label: {
               
            },
             messages: {
               successReachExit: "Congratulations, your robot reached the green cells!",
               failureReachExit: "Your robot didn't reach the green cell."
            }
         },

         es: {
            label: {
               
            },
             messages: {
               successReachExit: "Bravo, ¡su robot llegó a la casilla verde!",
               failureReachExit: "Su robot no ha llegado a la casilla verde."
            }
         }
      },
      cones: {
         fr: {
            label: {
               dropObject: "déposer un plot",
               onContainer: "sur une case marquée",
               obstacleInFront: "plot devant"
            },
            code: {
               dropObject: "deposerPlot",
               onContainer: "surCaseMarquee",
               obstacleInFront: "plotDevant"
            },
            messages: {
               successContainersFilled: "Bravo, votre robot a déposé des plots sur les bonnes cases !",
               failureContainersFilled: "Il manque des plots ou ils ne sont pas au bon endroit."
            }
         },

         en: {
            label: {
               dropObject: "drop a cone",
               onContainer: "on a marked cell",
               obstacleInFront: "cone ahead"
            },
            code: {
               dropObject: "dropCone",
               onContainer: "onMarkedCell",
               obstacleInFront: "coneAhead"
            },
            messages: {
               successContainersFilled: "Congratulations, your robot dropped cones on the marked cells!",
               failureContainersFilled: "Some cones are missing or are misplaced."
            }
         },

         es: {
            label: {
               dropObject: "soltar un cono",
               onContainer: "sobre una casilla marcada",
               obstacleInFront: "cono adelante"
            },
            code: {
               dropObject: "soltarCono",
               onContainer: "sobreCasillaMarcada",
               obstacleInFront: "conoAdelante"
            },
            messages: {
               successContainersFilled: "Bravo, ¡Su robot ha puesto los conos en las casillas correctas!",
               failureContainersFilled: "Aún hay conos en lugares incorrectos."
            }
         }
      },   
      flowers: {
         fr: {
            label: {
               dropObject: "semer une graine",
               onContainer: "terre sur la case",
               obstacleInFront: "fleur devant"
            },
            code: {
               dropObject: "semerGraine",
               onContainer: "terreSurCase",
               obstacleInFront: "fleurDevant"
            },
            description: {
               dropObject: "semerGraine() sème une graine sur la case du robot",
               onContainer: "terreSurCase() indique s'il y a de la terre sur la case du robot",
               obstacleInFront: "fleurDevant() indique s'il y a une fleur sur la case devant le robot"
            },
            messages: {
               successContainersFilled: "Bravo, votre robot est un bon jardinier !",
               failureContainersFilled: "Votre robot a semé hors des zones de terre.",
               failureContainersFilledLess: "Il reste de la terre vide de fleur !",
               failureDropObject: "Il y a déjà une fleur ici !",
					obstacle: "Attention à la fleur !",
            }
         },
         en: {
            label: {
               dropObject: "plant a seed",
               onContainer: "soil on the cell",
               obstacleInFront: "flower ahead"
            },
            code: {
               dropObject: "plantSeed",
               onContainer: "soilOnCell",
               obstacleInFront: "flowerAhead"
            },
            messages: {
               successContainersFilled: "Congratulations, your robot is a good gardener!",
               failureContainersFilled: "Your robot dropped seeds where there is no soil.",
               failureContainersFilledLess: "Some soil spots don't have any flower!",
               failureDropObject: "There is already a flower here !",
					obstacle: "Be careful, there's a flower!",
            }
         },

         es: {
            label: {
               dropObject: "sembrar una semilla",
               onContainer: "tierra en la casilla",
               obstacleInFront: "flor adelante"
            },
            code: {
               dropObject: "sembrarSemilla",
               onContainer: "tierraEnCasilla",
               obstacleInFront: "florAdelante"
            },
            messages: {
               successContainersFilled: "Bravo, ¡el robot es un gran jardinero!",
               failureContainersFilled: "El robot ha sembrado fuera de las casillas con tierra",
               failureDropObject: "Ya hay una flor aqui !",
               failureContainersFilledLess: "¡Aún hay tierra sin flores!"
            }
         }
      },   
      course: {
         fr: {
            messages: {
               successReachExit: "Bravo, le robot a atteint le drapeau !",
               failureReachExit: "Le robot n'est pas arrivé sur la case du drapeau.",
               obstacle: "Le robot tente de foncer dans un buisson !"
            }
         },
         en: {
            messages: {
               successReachExit: "Congratulations, the robot reached the flag!",
               failureReachExit: "The robot didn't reach the flag.",
               obstacle: "The robot is attemting to run into a bush!"
            }
         },

         es: {
            messages: {
               successReachExit: "Bravo, ¡El robot llegó a la bandera!",
               failureReachExit: "El robot no llegó a la bandera.",
               obstacle: "¡El robot intenta traspasar un arbusto!"
            }
         }
      },
      dominoes: {
         fr: {
            label: {
               withdrawObject: "ramasser le domino",
               onObject: "sur un domino"
            },
            code: {
               withdrawObject: "ramasserDomino",
               onObject: "surDomino"
            },
            description: {
               withdrawObject: "ramasserDomino() ramasse le domino sur la case du robot",
               onObject: "surDomino() teste si le robot est sur la case d'un domino"
            },
            messages: {
               "successPickedAllWithdrawables": "Bravo, le robot a ramassé tous les dominos demandés !",
               "successPickedAllWithdrawablesSingular": "Bravo, le robot a ramassé le domino demandé !",
               "failurePickedAllWithdrawables": "Le robot n'a pas ramassé les dominos demandés.",
               "failurePickedAllWithdrawablesSingular": "Le robot n'a pas ramassé le domino demandé.",
               failureUnfilteredObject: "Le robot a ramassé un domino non demandé."
            }
         },
         en: {
            label: {
               withdrawObject: "pick the domino",
            },
            code: {
               withdrawObject: "pickDomino"
            },
            messages: {
               "successPickedAllWithdrawables": "Congratulations, the robot picked all the requested dominoes!",
               "failurePickedAllWithdrawables": "The robot didn't pick up the requested dominoes!"
            }
         },
         es: {
            label: {
               withdrawObject: "recoger el dominó",
            },
            code: {
               withdrawObject: "recogerDomino"
            },
            messages: {
               "successPickedAllWithdrawables": "Bravo, ¡el robot recogió todos los dominó requeridos!",
               "failurePickedAllWithdrawables": "El robot no recogió todos los dominó requeridos."
            }
         }
      },
      biscuits: {
         fr: {
            label: {
               withdrawObject: "ramasser le biscuit",
               dropObject: "déposer le biscuit"
            },
            code: {
               withdrawObject: "ramasserBiscuit",
               dropObject: "deposerBiscuit"
            },
            description: {
               withdrawObject: "ramasserBiscuit() ramasse le biscuit sur la case du robot",
               dropObject: "deposerBiscuit() dépose le dernier biscuit ramassé, sur la case du robot"
            },
            messages: {
               "successPickedAllWithdrawables": "Bravo, le robot a ramassé tous les biscuits demandés !",
               "failurePickedAllWithdrawables": "Le robot n'a pas ramassé les biscuits demandés."
            }
         },
      },
      poker: {
         fr: {
            label: {
               withdrawObject: "ramasser la carte",
            },
            code: {
               withdrawObject: "ramasserCarte"
            },
            description: {
               withdrawObject: "ramasserCarte() ramasse la carte sur la case du robot"
            },
            messages: {
               "successPickedAllWithdrawables": "Bravo, le robot a ramassé toutes les cartes demandées !",
               "failurePickedAllWithdrawables": "Le robot n'a pas ramassé les cartes demandées."
            }
         },
      },
      pearls: {
         fr: {
            label: {
               withdrawObject: "ramasser la perle",
            },
            code: {
               withdrawObject: "ramasserPerle"
            },
            description: {
               withdrawObject: "ramasserPerle() ramasse la perle sur la case du robot"
            },
            messages: {
               "successPickedAllWithdrawables": "Bravo, le robot a ramassé toutes les perles demandées !",
               "failurePickedAllWithdrawables": "Le robot n'a pas ramassé les perles demandées."
            }
         },
      },
      fishing: {
         fr: {
            label: {
               withdrawObject: "prendre le poisson",
               dropObject: "déposer le poisson",
               withdrawNum: "prendre %1 poissons",
               withdrawNum_noShadow: "prendre %1 poissons",
               dropNum: "déposer %1 poissons",
               dropNum_noShadow: "déposer %1 poissons",
               nbWithdrawables: "nombre de poissons sur la case",
               containerSize: "nombre de poissons commandés",
               onObject: "poisson sur la case",
               onContainer: "sur une île",
            },
            code: {
               withdrawObject: "prendrePoisson",
               dropObject: "deposerPoisson",
               onObject: "surPoisson",
               onContainer: "surIle",
               nbWithdrawables: "nbPoissonsSur",
               containerSize: "nbPoissonsCommandes",
               withdrawNum: "prendre",
               withdrawNum_noShadow: "prendre"
            },
            description: {
               withdrawObject: "prendrePoisson() prend le poisson qui se trouve sur la case",
               dropObject: "deposerPoisson() dépose sur la case un poisson transporté",
               onObject: "surPoisson() indique s'il y a un ou des poissons sur la case",
               onContainer: "surIle() indique s'il y a une île sur la case",
               nbWithdrawables: "nbPoissonsSur() indique combien de poissons sont sur la case",
               containerSize: "nbPoissonsCommandes() indique il faut livrer de poissons sur l'île",
               dropNum: "deposer(nbPoissons) dépose nbPoissons poissons sur la case",
               withdrawNum: "prendre(nbPoissons) prend nbPoissons poissons sur la case",
            },
            messages: {
               emptyBag: "Le robot ne porte pas de poisson !",

               tooManyObjects: "Le robot porte déjà un poisson !",
               successContainersFilled: "Bravo, votre robot a apporté du poisson sur toutes les îles !",
               failureContainersFilled: "Votre robot a remis du poisson à l'eau !",
               failureContainersFilledLess: "Il reste encore au moins un poisson à apporter sur une île.",
               failureContainersFilledBag: "Il faut apporter les poissons sur une île !",
               failureDropObject: "Cette île a déjà reçu assez de poissons.",
               nothingToPickUp: "Pas de poisson ici !",
               failureContainersFilledBag: "Votre robot a pris trop de poissons.",
               failureDropObject: "Votre robot essaie de déposer trop de poissons sur cette île."
            }
         },

         es: {
            label: {
               withdrawObject: "recoger la bola",
               dropObject: "soltar la bola",
               onObject: "sobre una bola",
               onContainer: "sobre un agujero",
            },
            code: {
               withdrawObject: "recogerBola",
               dropObject: "soltarBola",
               onObject: "sobreBola",
               onContainer: "sobreAgujero",
            },   
            messages: {
               emptyBag: "¡El robot no carga ninguna bola!",
               tooManyObjects: "¡El robot ya está cargando una bola!",
               successContainersFilled: "Bravo, ¡el robot ordenó las bolas!",
               failureContainersFilled: "Hay bolas en lugares incorrectos.",
               failureContainersFilledLess: "Aún falta colocar una bola.",
               failureContainersFilledBag: "¡Debe colocar la bola en un agujero!",
            }
         }
      },
      marbles: {
         fr: {
            label: {
               withdrawObject: "ramasser la bille",
               dropObject: "déposer la bille",
               onObject: "sur une bille",
               onContainer: "sur un trou",
            },
            code: {
               withdrawObject: "ramasserBille",
               dropObject: "deposerBille",
               onObject: "surBille",
               onContainer: "surTrou",
            },   
            description: {
               withdrawObject: "ramasserBille() ramasse la bille de la case du robot",
               dropObject: "deposerBille() dépose la bille transportée sur la case du robot",
               onObject: "surBille() indique une bille est sur la case du robot",
               onContainer: "surTrou() indique s'il y a un trou sur la case du robot",
            },   
            messages: {
               emptyBag: "Le robot ne porte pas de bille !",
               tooManyObjects: "Le robot porte déjà une bille !",
               successContainersFilled: "Bravo, vous avez rangé les billes !",
               successContainersFilledSingular: "Bravo, vous avez rangé la bille !",
               failureContainersFilled: "Les billes ne sont pas toutes bien rangées.",
               failureContainersFilledSingular: "La bille n'est pas rangée dans le trou.",
               failureContainersFilledLess: "Il reste une bille à ranger.",
               failureContainersFilledBag: "Il faut déposer la bille dans le trou !",
            }
         },

         es: {
            label: {
               withdrawObject: "recoger la bola",
               dropObject: "soltar la bola",
               onObject: "sobre una bola",
               onContainer: "sobre un agujero",
            },
            code: {
               withdrawObject: "recogerBola",
               dropObject: "soltarBola",
               onObject: "sobreBola",
               onContainer: "sobreAgujero",
            },   
            messages: {
               emptyBag: "¡El robot no carga ninguna bola!",
               tooManyObjects: "¡El robot ya está cargando una bola!",
               successContainersFilled: "Bravo, ¡el robot ordenó las bolas!",
               failureContainersFilled: "Hay bolas en lugares incorrectos.",
               failureContainersFilledLess: "Aún falta colocar una bola.",
               failureContainersFilledBag: "¡Debe colocar la bola en un agujero!",
            }
         }
      },
      objects_in_space: {
         fr: {
            label: {
               obstacleInFront: "astéroïde devant"
            },
            code: {
               obstacleInFront: "asteroideDevant"
            },
            messages: {
               obstacle: "Attention à l'astéroïde !"
            }
         },

         es: {
            label: {
               obstacleInFront: "asteroide adelante"
            },
            code: {
               obstacleInFront: "asteroideAdelante"
            },
            messages: {
               obstacle: "¡Cuidado con el asteroide!"
            }
         }
      },
      packages: {
         fr: {
            label: {
               withdrawObject: "ramasser le livre",
               dropObject: "déposer le livre",
               withdrawNum: "ramasser %1 livres",
               withdrawNum_noShadow: "ramasser %1 livres",
               dropNum: "déposer %1 livres",
               dropNum_noShadow: "déposer %1 livres",
               nbWithdrawables: "nombre de livres sur la case",
               containerSize: "nombre de livres commandés",
               onObject: "livres sur la case",
               onContainer: "sur un carton",
            },
            code: {
               withdrawObject: "ramasserLivre",
               dropObject: "deposerLivre",
               onObject: "surLivres",
               onContainer: "surCarton",
               nbWithdrawables: "nbLivresSur",
               containerSize: "nbLivresCommandes",
            },
            description: {
               withdrawObject: "ramasserLivre() ramasse le livre qui se trouve sur la case",
               dropObject: "deposerLivre() dépose sur la case le livre transporté",
               onObject: "surLivres() indique s'il y a un ou des livres sur la case",
               onContainer: "surCarton() indique s'il y a un carton sur la case",
               nbWithdrawables: "nbLivresSur() indique combien de livres sont sur la case",
               containerSize: "nbLivresCommandes() indique combien de livres sont commandés pour ce carton",
               dropNum: "deposer(nbLivres) dépose nLivres livres sur la case",
               withdrawNum: "ramasser(nbLivres) ramasse nbLivres livres sur la case",
            },
            messages: {
               emptyBag: "Le robot ne porte pas de livre !",
               tooManyObjects: "Le robot porte déjà un livre !",
               successContainersFilled: "Bravo, tous les cartons sont pleins. Les colis vont pouvoir être livrés !",
               failureContainersFilled: "Tous les livres n'ont pas été mis en carton.",
               failureContainersFilledLess: "Il y a au moins un carton qui n'est pas plein.",
               failureContainersFilledBag: "Votre robot a ramassé trop de livres.",
               failureDropObject: "Ce colis est déjà plein. Votre robot essaie de déposer trop de livres.",
               nothingToPickUp: "Il n'y a plus de livre ici !"
            }
         },

         es: {
            label: {
               withdrawObject: "recoger la bola",
               dropObject: "soltar la bola",
               onObject: "sobre una bola",
               onContainer: "sobre un agujero",
            },
            code: {
               withdrawObject: "recogerBola",
               dropObject: "soltarBola",
               onObject: "sobreBola",
               onContainer: "sobreAgujero",
            },   
            messages: {
               emptyBag: "¡El robot no carga ninguna bola!",
               tooManyObjects: "¡El robot ya está cargando una bola!",
               successContainersFilled: "Bravo, ¡el robot ordenó las bolas!",
               failureContainersFilled: "Hay bolas en lugares incorrectos.",
               failureContainersFilledLess: "Aún falta colocar una bola.",
               failureContainersFilledBag: "¡Debe colocar la bola en un agujero!",
            }
         }
      },
      paint: {
         fr: {
             label: {
               dropObject: "peindre la case",
               onContainer: "sur une case marquée",
               readNumber: "nombre de la case",
             },
             code: {
                dropObject: "peindreCase",
                onContainer: "surCaseMarquee",
                readNumber: "nombreSurCase"
            },
            description: {
               dropObject: "peindreCase() peint la case sous le robot",
               onContainer: "surCaseMarquee() indique si le robot est sur une case marquée",
               readNumber: "nombreSurCase() renvoie le nombre écrit sur la case sous le robot"
             },
             messages: {
               successContainersFilled: "Bravo, votre robot a peint le motif !",
               failureContainersFilledSingular: "Votre robot n'a pas peint la bonne case.",
               failureContainersFilled: "Votre robot n'a pas peint les bonnes cases.",
               failureContainersFilledLess: "Votre robot n'a pas peint toutes les cases marquées.",
               failureContainersFilledLessSingular: "Votre robot n'a pas peint la case.",
               failureContainersFilledBag: "Votre robot n'a pas posé tous les objets",
             }
         },

         es: {
             label: {
               dropObject: "pintar la casilla",
               onContainer: "sobre una casilla marcada",
               readNumber: "número en la casilla",
             },
             code: {
                dropObject: "pintarCasilla",
                onContainer: "sobreCasillaMarcada",
                readNumber: "númeroEnCasilla",
             },
            description: {
               dropObject: "pintarCasilla() pinta la casilla bajo el robot",
               onContainer: "sobreCasillaMarcada() indica si el robot está sobre una casilla marcada",
               readNumber: "númeroEnCasilla() devuelve el número escrito en la casilla bajo el robot",
            },
             messages: {
               successContainersFilled: "Bravo, ¡su robot ha pintado el patrón!",
               failureContainersFilled: "El robot no pintó las casillas correctas.",
               failureContainersFilledLess: "Su robot no ha pintado todas las casillas marcadas.",
               failureContainersFilledBag: "Su robot no ha puesto todos los objetos",
             }
         }  
      },
      pixelArt: {
         fr: {
            messages: {
               successContainersFilled: "Bravo, votre dessin est identique au modèle. Quel artiste !",
               failureContainersFilled: "Regardez bien le modèle, votre dessin n'est pas identique.",
				   failureContainersFilledLess: "Le dessin n'est pas fini !",
				   failureDropObject: "Le robot n'a pas peint la case de la bonne couleur.",
            }
         }
      },
      rocket: {
         fr: {
            label: {
               obstacleRight: "asteroïde à droite",
               obstacleInFront: "asteroïde devant",
               onExit: "sur une fusée"
            },
            code: {
               onExit: "surFusee"
            },
            messages: {
               successReachExit: "Bravo, le robot a rejoint la fusée !",
               failureReachExit: "Le robot est perdu dans l'espace. Recommencez pour l'aider à rejoindre la fusée.",
               obstacle: "Attention à l'astéroïde !"
            }
         },

         es: {
            label: {
               obstacleRight: "asteroide a la derecha",
               obstacleInFront: "asteroide adelante",
            },
            messages: {
               successReachExit: "Bravo, ¡el robot llegó al cohete!",
               failureReachExit: "El robot está perdido en el espacio. Vuelva a comenzar para ayudarle a llegar al cohete.",
               obstacle: "¡Cuidado con el asteroide!"
            }
         }
      },
      seashells: {
         fr: {
            label: {
               withdrawObject: "ramasser le coquillage",
               dropObject: "déposer les coquillages",
               withdrawNum: "prendre %1 coquillages",
               withdrawNum_noShadow: "prendre %1 coquillages",
               dropNum: "déposer %1 coquillages",
               dropNum_noShadow: "déposer %1 coquillages",
               nbWithdrawables: "nombre de coquillages sur la case",
               containerSize: "taille du panier",
               onObject: "sur un coquillage",
               onContainer: "sur un panier",
               obstacleNorth: "rocher au nord",
               obstacleSouth: "rocher au sud",
               obstacleEast: "rocher à l'est",
               obstacleWest: "rocher à l'ouest"
            },
            code: {
               withdrawObject: "ramasserCoquillage",
               dropObject: "deposerCoquillages",
               onObject: "surCoquillage",
               onContainer: "surPanier",
               nbWithdrawables: "nbCoquillages",
               containerSize: "taillePanier",
               withdrawNum: "ramasser",
               withdrawNum_noShadow: "ramasser",
               obstacleNorth: "rocherNord",
               obstacleSouth: "rocherSud",
               obstacleEast: "rocherEst",
               obstacleWest: "rocherOuest"
            },
            description: {
               withdrawObject: "ramasserCoquillage() ramasse le coquillage qui se trouve sur la case",
               dropObject: "deposerCoquillages() dépose tous les coquillages transportés",
               onObject: "surCoquillage() indique s'il y a un ou des coquillages sur la case",
               onContainer: "surPanier() indique s'il y a un panier sur la case",
               nbWithdrawables: "nbCoquillages() indique combien de coquillages sont sur la case",
               containerSize: "taillePanier() indique combien le panier doit contenir de coquillages",
               dropNum: "deposer(nbCoquillages) dépose nbCoquillages coquillagess sur la case",
               withdrawNum: "ramasser(nbCoquillages) ramasse nbCoquillages coquillages sur la case",
               obstacleNorth: "rocherNord() teste s'il y a un rocher sur la case au nord du robot",
               obstacleSouth: "rocherSud() teste s'il y a un rocher sur la case au nord du robot",
               obstacleEast: "rocherEst() teste s'il y a un rocher sur la case au nord du robot",
               obstacleWest: "rocherOuest() teste s'il y a un rocher sur la case au nord du robot"
            },
            messages: {
               emptyBag: "Le robot ne porte pas de coquillage !",
               successContainersFilled: "Bravo, votre robot a collecté tous les coquillages !",
               failureContainersFilledLess: "Il reste encore au moins un coquillage à ramasser.",
               failureContainersFilledBag: "Il faut apporter les coquillages dans le panier !",
               failureDropObject: "Ce panier a déjà assez de coquillages.",
               nothingToPickUp: "Pas de coquillage ici !",
               failureContainersFilledBag: "Votre robot a ramassé trop de coquillages.",
               failureDropObject: "Votre robot essaie de déposer trop de coquillages dans ce panier.",
               obstacle: "Le robot essaie de foncer dans un rocher.",
               failureReachExit: "Le robot n'a pas déposé les coquillages dans le panier."
            }
         },
      },
      sokoban: {
         fr: {
            label: {
               pushObject: "pousser la caisse",
               onContainer: "sur une case marquée",
               pushableInFront: "caisse devant",
               obstacleInFront: "obstacle devant",
               readNumber: "nombre de la case"
            },
            code: {
               pushObject: "pousserCaisse",
               onContainer: "surCaseMarquee",
               pushableInFront: "caisseDevant",
               obstacleInFront: "obstacleDevant",
               readNumber: "nombreSurCase"
            },
            description: {
               onContainer: "surCaseMarquee() indique si le robot se trouve sur une case marquée",
               pushableInFront: "caisseDevant() indique si le robot est juste devant une caisse",
               pushObject: "pousserCaisse() avance le robot en poussant la caisse qui est devant",               
            },
            messages: {
               successContainersFilled: "Bravo, les caisses sont bien rangées !",
               successContainersFilledSingular: "Bravo, la caisse est bien rangée !",
               failureContainersFilled: "Il y a encore des caisses qui ne sont pas à leur place.",
               failureContainersFilledSingular: "La caisse n'est pas à sa place !",
               failureNothingToPush: "Il n'y a pas de caisse à pousser ici !",
               failureWhilePushing: "Le robot ne peut pas pousser ici !",
               obstacle: "Le robot essaie de foncer dans un mur ou dans une caisse !"
            }
         },

         en: {
            label: {
               pushObject: "push the box",
               onContainer: "on a marked cell",
               pushableInFront: "in front of a box",
               obstacleInFront: "in front of an obstacle",
               readNumber: "number on the cell"
            },
            code: {
               pushObject: "pushBox",
               onContainer: "onMarkedCell",
               pushableInFront: "boxAhead",
               obstacleInFront: "obstacleAhead",
               readNumber: "numberOnCell"
            },   
            messages: {
               successContainersFilled: "Congratulations, the boxes are correctly placed!",
               failureContainersFilled: "There are still boxes that are not places correctly.",
               failureNothingToPush: "There is no box to push here!",
               failureWhilePushing: "The robot can't push here!",
               obstacle: "The robot is trying to move into a wall or a box!"
            }
         },

         es: {
            label: {
               pushObject: "empujar la caja",
               onContainer: "sobre una casilla marcada",
               pushableInFront: "caja adelante",
               obstacleInFront: "obstáculo adelante",
               readNumber: "número en la casilla"
            },
            code: {
               pushObject: "empujarCaja",
               onContainer: "sobreCasillaMarcada",
               pushableInFront: "cajaAdelante",
               obstacleInFront: "obstáculoAdelante",
               readNumber: "númeroEnCasilla"
            },   
            messages: {
               successContainersFilled: "Bravo, ¡las cajas están bien ordenadas!",
               failureContainersFilled: "Aún hay cajas que no están en su lugar.",
               failureNothingToPush: "¡Aquí no hay caja para empujar!",
               failureWhilePushing: "¡El robot no puede empujar aquí!",
               obstacle: "El robot intenta traspasar un muro o una caja!"
            }
         }
      },
      veterinary: {
         fr: {
            label: {
               withdrawObject: "ramasser le bois",
               dropObject: "donner le bois au castor",
               withdrawNum: "ramasser %1 bûches",
               withdrawNum_noShadow: "ramasser %1 bûches",
               dropNum: "donner %1 bûches",
               dropNum_noShadow: "donner %1 bûches",
               nbWithdrawables: "nombre de bûches sur la case",
               containerSize: "nombre de bûches demandé",
               onObject: "sur du bois",
               onContainer: "chez un castor",
            },
            code: {
               withdrawObject: "ramasserBois",
               dropObject: "donnerBois",
               onObject: "surBois",
               onContainer: "chezCastor",
               nbWithdrawables: "nbBuchesSur",
               containerSize: "nbBuchesADeposer",
            },
            description: {
               withdrawObject: "ramasserBois() ramasse le bois qui se trouve sur la case",
               dropObject: "donnerBois() dépose sur la case le bois transporté",
               onObject: "surBois() indique s'il y a du bois sur la case",
               onContainer: "chezCastor() indique s'il y a un castor sur la case",
               nbWithdrawables: "nbBuchesSur() indique combien de bûches sont sur la case",
               containerSize: "nbBuchesADeposer() indique combien de bûches sont demandées sur la case",
               dropNum: "deposer(nbBuches) dépose nbBuches bûches sur la case",
               withdrawNum: "ramasser(nbBuches) ramasse nbBuches bûches sur la case",
            },
            messages: {
               emptyBag: "Le robot ne porte pas de bois !",
               tooManyObjects: "Le robot porte déjà du bois !",
               successContainersFilled: "Bravo, tous les castors ont du bois. Ils remercient votre robot !",
               failureContainersFilled: "Tout le bois n'a pas été distribué.",
               failureContainersFilledLess: "Il reste du bois à distribuer.",
               failureContainersFilledBag: "Il faut donner le bois au castor !",
               failureDropObject: "Ce castor a déjà du bois.",
            }
         },

         es: {
            label: {
               withdrawObject: "recoger la bola",
               dropObject: "soltar la bola",
               onObject: "sobre una bola",
               onContainer: "sobre un agujero",
            },
            code: {
               withdrawObject: "recogerBola",
               dropObject: "soltarBola",
               onObject: "sobreBola",
               onContainer: "sobreAgujero",
            },   
            messages: {
               emptyBag: "¡El robot no carga ninguna bola!",
               tooManyObjects: "¡El robot ya está cargando una bola!",
               successContainersFilled: "Bravo, ¡el robot ordenó las bolas!",
               failureContainersFilled: "Hay bolas en lugares incorrectos.",
               failureContainersFilledLess: "Aún falta colocar una bola.",
               failureContainersFilledBag: "¡Debe colocar la bola en un agujero!",
            }
         }
      },
   };
   
   var iconSrc = $("img[src$='icon.png']").attr("src");
   var imgPrefix = iconSrc.substring(0, iconSrc.length - 8);
   function imgUrlWithPrefix(url) {
      return /^https?:\/\//.exec(url) ? url : imgPrefix + url;
   };
   function getImgPath(url) {
      if (window.modulesPath != undefined) {
         return window.modulesPath + "img/algorea/" + url;
      }
      return imgUrlWithPrefix(url)
   };

   var contextParams = {
      none: {
         hideSaveOrLoad: true,
         actionDelay: 200,
         ignoreInvalidMoves: false,
         checkEndEveryTurn: false,
         cellSide: 60
      },
      arrows: {
         newBlocks: [
            {
               name: "onRightArrow",
               strings: {
                  fr: {
                     label: "sur une flèche vers la droite",
                     code: "surFlecheDroite",
                     description: "surFlecheDroite(): Le robot est-il sur une flèche vers la droite ?"
                  },
                  es: {
                     label: "sobre una flecha hacia la derecha",
                     code: "sobreFlechaHaciaLaDerecha",
                     description: "sobreFlechaHaciaLaDerecha(): ¿Se encuentra el robot sobre una flecha hacia la derecha?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onRightArrow",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.forwardsRight===true;}));
               }
            },
            {
               name: "onLeftArrow",
               strings: {
                  fr: {
                     label: "sur une flèche vers la gauche",
                     code: "surFlecheGauche",
                     description: "surFlecheGauche(): Le robot est-il sur une flèche vers la gauche ?"
                  },

                  es: {
                     label: "sobre una flecha hacia la izquierda",
                     code: "sobreFlechaHaciaLaIzquierda",
                     description: "sobreFlechaHaciaLaIzquierda(): ¿Se encuentra el robot sobre una flecha hacia la izquierda?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onLeftArrow",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.forwardsLeft===true;}));
               }
            },
            {
               name: "onTopArrow",
               strings: {
                  fr: {
                     label: "sur une flèche vers le haut",
                     code: "surFlecheHaut",
                     description: "surFlecheHaut(): Le robot est-il sur une flèche vers le haut ?"
                  },

                  es: {
                     label: "sobre una flecha hacia arriba",
                     code: "sobreFlechaHaciaArriba",
                     description: "sobreFlechaHaciaArriba(): ¿Se encuentra el robot sobre una flecha hacia arriba?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onTopArrow",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.forwardsTop===true;}));
               }
            },
            {
               name: "onBottomArrow",
               strings: {
                  fr: {
                     label: "sur une flèche vers le bas",
                     code: "surFlecheBas",
                     description: "surFlecheBas(): Le robot est-il sur une flèche vers le bas ?"
                  },

                  es: {
                     label: "sobre una flecha hacia abajo",
                     code: "sobreFlechaHaciaAbajo",
                     description: "sobreFlechaHaciaAbajo(): ¿Se encuentra el robot sobre una flecha hacia abajo?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onBottomArrow",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.forwardsBottom===true;}));
               }
            }
         ],
         backgroundColor: "#d3e7b6",
         itemTypes: {
            red_robot: { img: "red_robot_alt.png", side: 90, nbStates: 1, isRobot: true, offsetX: -15, offsetY: 15, zOrder: 2 },
            cell: {num: 1, color: "#d3e7b6", side: 60, isObstacle: true, zOrder: 0 },
            box: { num: 3, img: "chest.png", side: 60, isExit: true },
            leftArrow: { num: 4, img: "leftArrow.png", side: 60, forwardsLeft: true, zOrder: 0},
            rightArrow: { num: 5, img: "rightArrow.png", side: 60, forwardsRight: true, zOrder: 0},
            topArrow: { num: 6, img: "topArrow.png", side: 60, forwardsTop: true, zOrder: 0},
            bottomArrow: { num: 7, img: "bottomArrow.png", side: 60, forwardsBottom: true, zOrder: 0}
         },
         checkEndCondition: robotEndConditions.checkReachExit
      },
      cards: {
         newBlocks: [
            {
               name: "onRound",
               strings: {
                  fr: {
                     label: "rond sur la carte",
                     code: "rondCarte",
                     description: "rondCarte(): Le robot est-il sur une carte qui contient un rond ?"
                  },

                  es: {
                     label: "círculo sobre la carta",
                     code: "círculoCarta",
                     description: "círculoCarta(): ¿está el robot sobre una carta que contiene un círculo?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onRound",
                  yieldsValue: true
               },
               func: function(callback) {
                  if(!this.isOn(function(obj) {return obj.isWithdrawable===true || obj.isContainer===true;}))
                     throw(strings.messages.nothingToLookAt);
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isRound===true;}));
               }
            },
            {
               name: "onSquare",
               strings: {
                  fr: {
                     label: "carré sur la carte",
                     code: "carreCarte",
                     description: "carreCarte(): Le robot est-il sur une carte qui contient un carré ?"
                  },
                  es: {
                     label: "cuadrado sobre la carta",
                     code: "cuadradoCarta",
                     description: "cuadradoCarta(): ¿está el robot sobre una carta que contiene un cuadrado?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onSquare",
                  yieldsValue: true
               },
               func: function(callback) {
                  if(!this.isOn(function(obj) {return obj.isWithdrawable===true || obj.isContainer===true;}))
                     throw(strings.messages.nothingToLookAt);
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isSquare===true;}));
               }
            },
            {
               name: "onTriangle",
               strings: {
                  fr: {
                     label: "triangle sur la carte",
                     code: "triangleCarte",
                     description: "rondCarte(): Le robot est-il sur une carte qui contient un triangle ?"
                  },
                  es: {
                     label: "triángulo sobre la carta",
                     code: "triánguloCarta",
                     description: "triánguloCarta(): ¿está el robot sobre una carta que contiene un triángulo?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onTriangle",
                  yieldsValue: true
               },
               func: function(callback) {
                  if(!this.isOn(function(obj) {return obj.isWithdrawable===true || obj.isContainer===true;}))
                     throw(strings.messages.nothingToLookAt);
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isTriangle===true;}));
               }
            },
            {
               name: "onQuadrille",
               strings: {
                  fr: {
                     label: "sur un motif quadrillé",
                     code: "surQuadrille",
                     description: "surQuadrille(): Le robot est-il sur une carte quadrillée ?"
                  },
                  es: {
                     label: "patrón cuadriculado",
                     code: "sobreCudarícula",
                     description: "sobreCudarícula(): ¿Está el robot sobre una carta cuadriculada?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onQuadrille",
                  yieldsValue: true
               },
               func: function(callback) {
                  if(!this.isOn(function(obj) {return obj.isWithdrawable===true || obj.isContainer===true;}))
                     throw(strings.messages.nothingToLookAt);
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isQuadrille===true;}));
               }
            },
            {
               name: "onStriped",
               strings: {
                  fr: {
                     label: "sur un motif rayé",
                     code: "surRaye",
                     description: "surRaye(): Le robot est-il sur une carte rayée ?"
                  },
                  es: {
                     label: "patrón rayado",
                     code: "sobreRayado",
                     description: "sobreRayado(): ¿Está el robot sobre una carta rayada?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onStriped",
                  yieldsValue: true
               },
               func: function(callback) {
                  if(!this.isOn(function(obj) {return obj.isWithdrawable===true || obj.isContainer===true;}))
                     throw(strings.messages.nothingToLookAt);
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isStriped===true;}));
               }
            },
            {
               name: "onDotted",
               strings: {
                  fr: {
                     label: "sur un motif à pois",
                     code: "surPois",
                     description: "surPois(): Le robot est-il sur une carte à pois ?"
                  },
                  es: {
                     label: "patrón con puntos",
                     code: "sobrePuntos",
                     description: "sobrePuntos(): ¿Está el robot sobre una carta con puntos?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onDotted",
                  yieldsValue: true
               },
               func: function(callback) {
                  if(!this.isOn(function(obj) {return obj.isWithdrawable===true || obj.isContainer===true;}))
                     throw(strings.messages.nothingToLookAt);
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isDotted===true ;}));
               }
            }
         ],
         bagSize: 1,
         backgroundColor: "#abeaf4",
         itemTypes: {
            red_robot: { img: "red_robot_alt.png", side: 90, nbStates: 1, isRobot: true, offsetX: -15, offsetY: 15, zOrder: 2 },
            square: { num: 2, img: "card_square.png", side: 60, isContainer: true, containerFilter: function(obj) { return obj.isSquare === true; }, zOrder: 0 },
            round: { num: 3, img: "card_round.png", side: 60, isContainer: true, containerFilter: function(obj) { return obj.isRound === true; }, zOrder: 0 },
            triangle: { num: 4, img: "card_triangle.png", side: 60, isContainer: true, containerFilter: function(obj) { return obj.isTriangle === true; }, zOrder: 0 },
            dotted: { num: 5, img: "card_dotted.png", side: 60, isContainer: true, containerFilter: function(obj) { return obj.isDotted === true; }, zOrder: 0 },
            striped: { num: 6, img: "card_striped.png", side: 60, isContainer: true, containerFilter: function(obj) { return obj.isStriped === true; }, zOrder: 0 },
            quadrille: { num: 7, img: "card_quadrille.png", side: 60, isContainer: true, containerFilter: function(obj) { return obj.isQuadrille === true; }, zOrder: 0 },
            roundQuadrille: { img: "card_roundQuadrille.png", side: 60, isWithdrawable: true, isRound: true, isQuadrille: true, zOrder: 1 },
            squareQuadrille: { img: "card_squareQuadrille.png", side: 60, isWithdrawable: true, isSquare: true, isQuadrille: true, zOrder: 1 },
            triangleQuadrille: { img: "card_triangleQuadrille.png", side: 60, isWithdrawable: true, isTriangle: true, isQuadrille: true, zOrder: 1 },
            roundStriped: { img: "card_roundStriped.png", side: 60, isWithdrawable: true, isRound: true, isStriped: true, zOrder: 1 },
            squareStriped: { img: "card_squareStriped.png", side: 60, isWithdrawable: true, isSquare: true, isStriped: true, zOrder: 1 },
            triangleStriped: { img: "card_triangleStriped.png", side: 60, isWithdrawable: true, isTriangle: true, isStriped: true, zOrder: 1 },
            roundDotted: { img: "card_roundDotted.png", side: 60, isWithdrawable: true, isRound: true, isDotted: true, zOrder: 1 },
            squareDotted: { img: "card_squareDotted.png", side: 60, isWithdrawable: true, isSquare: true, isDotted: true, zOrder: 1 },
            triangleDotted: { img: "card_triangleDotted.png", side: 60, isWithdrawable: true, isTriangle: true, isDotted: true, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      castle: {
         backgroundColor: "#1f003c",
         borderColor: "#2d0073",
         hasGravity: true,
         bagSize: 1,
         containerSize: 1,
         itemTypes: {
            robot: { img: "castle_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, offsetY: 3, zOrder: 3 },
            platform: { num: 2, img: "platform.png", side: 60, isObstacle: true, zOrder: 0 },
            hearth: { num: 4, img: "hearth.png", side: 60, isContainer: true, zOrder: 1},
            wood: { num:5, img: "firewood.png", side: 60, isWithdrawable: true, zOrder: 2},
            // projectile: {num: 6, img: "projectile.png", side: 60, zOrder: 4, action: function(item, time) { this.moveProjectile(item); }, isProjectile: true},
            // door: { num: 8, img: "door.png", side: 60, isExit: true, zOrder: 1},
            // dispersion: {img: "dispersion.png", side: 60, zOrder: 4, action: function(item, time) { this.destroy(item); }, isProjectile: true},
            // dispersion_robot: {img: "dispersion.png", side: 60, zOrder: 4, offsetY: -15, action: function(item, time) { this.destroy(item); }, isProjectile: true},
            projectile_generator: {num: 7, side: 60, action: function(item, time) {
                  if(item.period == undefined)
                     item.period = 1;
                  if(item.start == undefined)
                     item.start = 1;
                  if(time % item.period == item.start)
                     this.dropObject({type: "projectile"}, {row: item.row, col: item.col});
               }}
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      chticode_abs: {
         itemTypes: {
            red_robot: { img: "red_robot.png", side: 90, nbStates: 1, isRobot: true, offsetX: -15, offsetY: 15, zOrder: 2 },
            obstacle: { num: 2, img: "obstacle.png", side: 60, isObstacle: true },
            green: { num: 3, color: "#b5e61d", side: 60, isExit: true, zOrder: 0 },
            gem: { num: 4, img: "gem.png", side: 60, isWithdrawable: true, autoWithdraw: true, zOrder: 1 },
            north: { num: 5, img: "north.png", side: 60, zOrder: 0 },
            south: { num: 6, img: "south.png", side: 60, zOrder: 0 },
            east: { num: 7, img: "east.png", side: 60, zOrder: 0 },
            west: { num: 8, img: "west.png", side: 60, zOrder: 0 },
         },
         checkEndCondition: robotEndConditions.checkBothReachAndCollect
      },
      chticode_rel: {
         backgroundColor: "#BF5E47",
         borderColor: "#96413B",
         itemTypes: {
            robot: { img: "yellow_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -14, zOrder: 2 },
            obstacle: { num: 2, img: "brick_wall.png", side: 60, isObstacle: true },
            green: { num: 3, color: "#b5e61d", side: 60, isExit: true},
            gem: { num: 4, img: "gem.png", side: 60, isWithdrawable: true, autoWithdraw: true, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkReachExit
      },
      cones: {
         bagInit: {
            count: 200,
            type: "cone"
         },
         backgroundColor: "#f9f9c1",
         itemTypes: {
            green_robot: { img: "green_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            marker: { num: 2, img: "marker.png", side: 60, isContainer: true, zOrder: 0 },
            cone: { num: 3, img: "cone.png", side: 60, isWithdrawable: true, isObstacle: true, zOrder: 1 },
            contour: { num: 4, img: "contour.png", side: 60, zOrder: 1 },
            fixed_cone: { num: 5, img: "cone.png", side: 60, isObstacle: true, zOrder: 1 },
            number: { num: 6, side: 60, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      course: {
         backgroundColor: "#fff1c4",
         borderColor: "#d4eda5",
         itemTypes: {
            red_robot: { img: "red_robot.png", side: 70, nbStates: 1, offsetX: -5, offsetY: 5, isRobot: true, zOrder: 2 },
            red_robot_alt: { img: "red_robot_alt.png", side: 70, nbStates: 1, offsetX: -5, offsetY: 5, isRobot: true, zOrder: 2 },
            bush: { num: 2, img: "bush.png", side: 60, isObstacle: true, zOrder: 0 },
            flag: { num: 3, img: "flag.png", side: 60, isExit: true, zOrder: 0},
            wall: { num: 4, img: "wall.png", side: 60, isObstacle: true, zOrder: 0 },
            number: { num: 5, side: 60, zOrder: 1 },
            horizontal_closed_door: { num: 7, img:  "horizontal_closed_door.png", side: 60, isObstacle: true, zOrder: 1 },
            vertical_closed_door: { num: 8, img:  "vertical_closed_door.png", side: 60, isObstacle: true, zOrder: 1 },
            horizontal_open_door: { num: 11, img:  "horizontal_open_door.png", side: 60, zOrder: 1},
            vertical_open_door: { num: 12, img:  "vertical_open_door.png", side: 60, zOrder: 1},
            water: { num: 13, img: "water.png", side: 60, isObstacle: true, zOrder: 1 },
            board: { num: 14, img: "water+board.png", side: 60, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkReachExit
      },
      dominoes: {
         newBlocks: [
            {
               name: "onCross",
               strings: {
                  fr: {
                     label: "sur croix",
                     code: "surCroix",
                     description: "surCroix(): Le robot est-il sur une croix ?"
                  },
                  es: {
                     label: "sobre cruz",
                     code: "sobreCruz",
                     description: "sobreCruz(): ¿Se encuentra el robot sobre una cruz?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onCross",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isCross===true;}));
               }
            },
            {
               name: "onStar",
               strings: {
                  fr: {
                     label: "sur étoile",
                     code: "surEtoile",
                     description: "surEtoile(): Le robot est-il sur une étoile ?"
                  },
                  es: {
                     label: "sobre estrella",
                     code: "sobreEstrella",
                     description: "sobreEstrella(): ¿Se encuentra el robot sobre una estrella?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onStar",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isStar===true;}));
               }
            },
            {
               name: "onSquare",
               strings: {
                  fr: {
                     label: "sur carré",
                     code: "surCarre",
                     description: "surCarre(): Le robot est-il sur un carré ?"
                  },
                  es: {
                     label: "sobre cuadrado",
                     code: "sobreCuadrado",
                     description: "sobreCuadrado(): ¿Se encuentra el robot sobre un cuadrado?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onSquare",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isSquare===true;}));
               }
            },
            {
               name: "onTriangle",
               strings: {
                  fr: {
                     label: "sur triangle",
                     code: "surTriangle",
                     description: "surTriangle(): Le robot est-il sur un triangle ?"
                  },
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onTriangle",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isTriangle===true;}));
               }
            }
         ],
         backgroundColor: "#4B254E",
         noBorders: false,
         borderColor: "#675B6E",
         itemTypes: {
            pink_robot: { img: "pink_robot_above.png", side: 70, nbStates: 9, isRobot: true, offsetX: -5, offsetY: 5, zOrder: 2 },
            green_robot: { img: "green_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            contour: { num: 2, img: "contour.png", side: 60, zOrder: 0 },
            GG: { num: 3, img: "GG.png", side: 60, isWithdrawable: true, isCross: true, zOrder: 1 },
            GO: { num: 4, img: "GO.png", side: 60, isWithdrawable: true, isCross: true, isStar: true, zOrder: 1 },
            GB: { num: 5, img: "GB.png", side: 60, isWithdrawable: true, isCross: true, isSquare: true, isTriangle: true, zOrder: 1 },
            OG: { num: 6, img: "OG.png", side: 60, isWithdrawable: true, isStar: true, isCross: true, zOrder: 1 },
            OO: { num: 7, img: "OO.png", side: 60, isWithdrawable: true, isStar: true, zOrder: 1 },
            OB: { num: 8, img: "OB.png", side: 60, isWithdrawable: true, isStar: true, isSquare: true, isTriangle: true, zOrder: 1 },
            BG: { num: 9, img: "BG.png", side: 60, isWithdrawable: true, isSquare: true, isTriangle: true, isCross: true, zOrder: 1 },
            BO: { num: 10, img: "BO.png", side: 60, isWithdrawable: true, isSquare: true, isTriangle: true, isStar: true, zOrder: 1 },
            BB: { num: 11, img: "BB.png", side: 60, isWithdrawable: true, isSquare: true, isTriangle: true, zOrder: 1 },
            board_background: { num: 12, color: "#ffffff", side: 60, zOrder: 0 },
            board: {num: 13, side: 60, isWritable: true, zOrder: 1 },
            obstacle: { num: 14, img: "wall.png", side: 60, isObstacle: true, zOrder: 0 }
         }
      },
      biscuits: {
         newBlocks: [
            {
               name: "onChocolate",
               strings: {
                  fr: {
                     label: "sur chocolat",
                     code: "surChocolat",
                     description: "surChocolat(): Le robot est-il sur un biscuit au chocolat ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onChocolate",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isChocolate===true;}));
               }
            },
            {
               name: "onHole",
               strings: {
                  fr: {
                     label: "sur trou",
                     code: "surTrou",
                     description: "surTrou(): Le robot est-il sur un biscuit troué ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onHole",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isHole===true;}));
               }
            },
            {
               name: "onCircle",
               strings: {
                  fr: {
                     label: "sur rond",
                     code: "surRond",
                     description: "surRond(): Le robot est-il sur un biscuit rond ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onCircle",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isCircle===true;}));
               }
            },
            {
               name: "onTriangle",
               strings: {
                  fr: {
                     label: "sur triangle",
                     code: "surTriangle",
                     description: "surTriangle(): Le robot est-il sur un biscuit triangle ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onTriangle",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isTriangle===true;}));
               }
            },
            {
               name: "onSquare",
               strings: {
                  fr: {
                     label: "sur carré",
                     code: "surCarre",
                     description: "surCarre(): Le robot est-il sur un biscuit carré ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onSquare",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isSquare===true;}));
               }
            }
         ],
         noBorders: true,
         backgroundColor: "#F9E6C3",
         itemTypes: {
            red_robot: { img: "red_robot.png", side: 90, nbStates: 1, isRobot: true,  offsetX: -15, offsetY: 15, zOrder: 2 },
            CCF: { num: 3, img: "biscuit_circle_choc.png", side: 60, isWithdrawable: true, isCircle: true, isChocolate: true, Order: 1 },
            CCH: { num: 4, img: "biscuit_circle_choc_hole.png", side: 60, isWithdrawable: true, isCircle: true, isChocolate: true, isHole: true, zOrder: 1 },
            CPF: { num: 5, img: "biscuit_circle.png", side: 60, isWithdrawable: true, isCircle: true, zOrder: 1 },
            CPH: { num: 6, img: "biscuit_circle_hole.png", side: 60, isWithdrawable: true, isCircle: true, isHole: true, zOrder: 1 },
            SCF: { num: 7, img: "biscuit_square_choc.png", side: 60, isWithdrawable: true, isSquare: true, isChocolate: true, zOrder: 1 },
            SCH: { num: 8, img: "biscuit_square_choc_hole.png", side: 60, isWithdrawable: true, isSquare: true, isChocolate: true, isHole: true, zOrder: 1 },
            SPF: { num: 9, img: "biscuit_square.png", side: 60, isWithdrawable: true, isSquare: true, zOrder: 1 },
            SPH: { num: 10, img: "biscuit_square_hole.png", side: 60, isWithdrawable: true, isSquare: true, isHole: true, zOrder: 1 },
            TCF: { num: 11, img: "biscuit_triangle_choc.png", side: 60, isWithdrawable: true, isTriangle: true, isChocolate: true, zOrder: 1 },
            TCH: { num: 12, img: "biscuit_triangle_choc_hole.png", side: 60, isWithdrawable: true, isTriangle: true, isChocolate: true, isHole: true, zOrder: 1 },
            TPF: { num: 13, img: "biscuit_triangle.png", side: 60, isWithdrawable: true, isTriangle: true, zOrder: 1 },
            TPH: { num: 14, img: "biscuit_triangle_hole.png", side: 60, isWithdrawable: true, isTriangle: true, isHole: true, zOrder: 1 },
            board_background: { num: 15, color: "#ffffff", side: 60, zOrder: 0 },
            board: {num: 16, side: 60, isWritable: true, zOrder: 1 },
            obstacle: { num: 17, img: "obstacle.png", side: 60, isObstacle: true, zOrder: 0 }
         },
         checkEndCondition: robotEndConditions.checkSpecificCollection
      },
      poker: {
         newBlocks: [
            {
               name: "onHearts",
               strings: {
                  fr: {
                     label: "sur du coeur",
                     code: "surCoeur",
                     description: "surCoeur(): Le robot est-il sur une carte coeur ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onHearts",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isHearts===true;}));
               }
            },
            {
               name: "onSpades",
               strings: {
                  fr: {
                     label: "sur du pique",
                     code: "surPique",
                     description: "surPique(): Le robot est-il sur un carte pique ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onSpades",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isSpades===true;}));
               }
            },
            {
               name: "onClubs",
               strings: {
                  fr: {
                     label: "sur du trèfle",
                     code: "surTrefle",
                     description: "surTrefle(): Le robot est-il sur une carte trèfle ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onClubs",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isClubs===true;}));
               }
            },
            {
               name: "onDiamonds",
               strings: {
                  fr: {
                     label: "sur du carreau",
                     code: "surCarreau",
                     description: "surCarreau(): Le robot est-il sur une case carreau ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onDiamonds",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isDiamonds===true;}));
               }
            },
            {
               name: "onOneShape",
               strings: {
                  fr: {
                     label: "sur une seule forme",
                     code: "surUneForme",
                     description: "surUneForme(): Le robot est-il sur une carte contenant une seule forme ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onOneShape",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isOneShape===true;}));
               }
            },
            {
               name: "onTwoShapes",
               strings: {
                  fr: {
                     label: "sur deux formes",
                     code: "surDeuxFormes",
                     description: "surDeuxFormes(): Le robot est-il sur une carte contenant deux formes ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onTwoShapes",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isTwoShapes===true;}));
               }
            }
         ],
         noBorders: true,
         backgroundColor: "#880015",
         itemTypes: {
            red_robot: { img: "red_robot.png", side: 90, nbStates: 1, isRobot: true,  offsetX: -15, offsetY: 15, zOrder: 2 },
            H1: { num: 3, img: "hearts_1.png", side: 60, isWithdrawable: true, isHearts: true, isOneShape: true, Order: 1 },
            H2: { num: 4, img: "hearts_2.png", side: 60, isWithdrawable: true, isHearts: true, isTwoShapes: true, Order: 1 },
            S1: { num: 5, img: "spades_1.png", side: 60, isWithdrawable: true, isSpades: true, isOneShape: true, Order: 1 },
            S2: { num: 6, img: "spades_2.png", side: 60, isWithdrawable: true, isSpades: true, isTwoShapes: true, Order: 1 },
            C1: { num: 7, img: "clubs_1.png", side: 60, isWithdrawable: true, isClubs: true, isOneShape: true, Order: 1 },
            C2: { num: 8, img: "clubs_2.png", side: 60, isWithdrawable: true, isClubs: true, isTwoShapes: true, Order: 1 },
            D1: { num: 9, img: "diamonds_1.png", side: 60, isWithdrawable: true, isDiamonds: true, isOneShape: true, Order: 1 },
            D2: { num: 10, img: "diamonds_2.png", side: 60, isWithdrawable: true, isDiamonds: true, isTwoShapes: true, Order: 1 },
            board_background: { num: 15, color: "#ffffff", side: 60, zOrder: 0 },
            board: {num: 16, side: 60, isWritable: true, zOrder: 1 },
            obstacle: { num: 17, img: "obstacle.png", side: 60, isObstacle: true, zOrder: 0 }
         },
         checkEndCondition: robotEndConditions.checkSpecificCollection
      },
      pearls: {
         newBlocks: [
            {
               name: "onBlue",
               strings: {
                  fr: {
                     label: "sur du bleu",
                     code: "surBleu",
                     description: "surBleu(): Le robot est-il sur un objet bleu ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onBlue",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isBlue===true;}));
               }
            },
            {
               name: "onHole",
               strings: {
                  fr: {
                     label: "sur trou",
                     code: "surTrou",
                     description: "surTrou(): Le robot est-il sur un objet troué ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onHole",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isHole===true;}));
               }
            },
            {
               name: "onYellow",
               strings: {
                  fr: {
                     label: "sur du jaune",
                     code: "surJaune",
                     description: "surJaune(): Le robot est-il sur un objet jaune ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onYellow",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isYellow===true;}));
               }
            },
            {
               name: "onBlack",
               strings: {
                  fr: {
                     label: "sur du noir",
                     code: "surNoir",
                     description: "surNoir(): Le robot est-il sur un objet noir ?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onBlack",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isBlack===true;}));
               }
            }
         ],
         noBorders: true,
         backgroundColor: "#E9E9E9",
         itemTypes: {
            red_robot: { img: "red_robot.png", side: 90, nbStates: 1, isRobot: true,  offsetX: -15, offsetY: 15, zOrder: 2 },
            PB: { num: 3, img: "pearl_blue.png", side: 60, isWithdrawable: true, isBlue: true, isHole: true, Order: 1 },
            PY: { num: 4, img: "pearl_yellow.png", side: 60, isWithdrawable: true, isYellow: true, isHole: true, zOrder: 1 },
            PBK: { num: 5, img: "pearl_black.png", side: 60, isWithdrawable: true, isBlack: true, isHole: true, zOrder: 1 },
            BB: { num: 6, img: "marble_blue.png", side: 60, isWithdrawable: true, isBlue: true, zOrder: 1 },
            BY: { num: 7, img: "marble_yellow.png", side: 60, isWithdrawable: true, isYellow: true, zOrder: 1 },
            BBK: { num: 8, img: "marble_black.png", side: 60, isWithdrawable: true, isBlack: true, zOrder: 1 },
            board_background: { num: 15, color: "#ffffff", side: 60, zOrder: 0 },
            board: {num: 16, side: 60, isWritable: true, zOrder: 1 },
            obstacle: { num: 17, img: "obstacle.png", side: 60, isObstacle: true, zOrder: 0 }
         },
         checkEndCondition: robotEndConditions.checkSpecificCollection
      },
      bridges: {
         newBlocks: [
            {
               name: "dropBridgeInFront",
               strings: {
                  fr: {
                     label: "construire un pont devant",
                     code: "construirePontDevant",
                     description: "construirePontDevant(): construit un pont sur la case devant le robot"
                  }
               },
               category: "robot",
               type: "actions",
               block: {
                  name: "dropBridgeInFront"
               },
               func: function(callback) {
                  var coords = context.coordsInFront();
                  if(this.getItemsOn(coords.row, coords.col, function(item) { return item.isOccupied === true; }).length != 0) {
                     this.leave(window.languageStrings.messages.failureDropPlatform);
                  }
                  this.destroyMask(coords.row, coords.col);
                  this.dropObject({type: "bridge"}, coords);
                  var obstacles = context.getItemsOn(coords.row, coords.col, function(obj) { return obj.isObstacle === true; });
                  for(var iObstacle = 0;iObstacle < obstacles.length;iObstacle++) {
                     obstacles[iObstacle].isObstacle = false;
                  }
                  this.callCallback(callback);
               }
            },
            {
               name: "waterInFront",
               strings: {
                  fr: {
                     label: "eau devant",
                     code: "eauDevant",
                     description: "eauDevant(): indique s'il y a de l'eau devant le robot"
                  }
               },
               category: "robot",
               type: "sensors",
               block: { name: "waterInFront", yieldsValue: true },
               func: function(callback) {
                  var coords = context.coordsInFront();
                  this.callCallback(callback, this.hasOn(coords.row, coords.col, function(item) { return ((item.isObstacle === true) && (item.isOccupied === false)); }));
               }
            }
         ],

         backgroundColor: "#5cb1d6",
         borderColor: "#fff",
         itemTypes: {
            green_robot: { img: "green_robot.png", side: 70, nbStates: 9, isRobot: true, offsetX: -5, offsetY: 5, zOrder: 3 },
            treasure: { num: 2, img: "treasure.png", side: 70, isExit: true, isOccupied: true, offsetX: -5, zOrder: 0},
            water : { num: 1, side: 60, isObstacle: true, isOccupied: false, zOrder: 0 },
            island : { num: 3, img: "island.png", side: 60, isOccupied: true, zOrder: 0 },
            bridge: { num: 4, img: "bridge.png", side: 60, isOccupied: true, zOrder: 2 },
            wave : { num: 5, img: "wave.png", side: 60, zOrder: 0},
            obstacle : { num: 6, img: "obstacle.png", side: 60, isObstacle: true, isOccupied: true, zOrder: 0 },
            mask: {num: 7, img: "mask.png", side: 60, isMask: true, zOrder: 2 },
         },
         checkEndCondition: robotEndConditions.checkReachExit

      },
      fishing: {
         backgroundColor: "#57b8bf",
         borderColor: "#489a9c",
         bagSize: 1,
         containerSize: 1,
         itemTypes: {
            robot: { img: "buoy_robot.png", side: 60, nbStates: 1, isRobot: true, zOrder: 4, customDisplay: function(obj) {
                  if(context.bag.length != 0)
                     obj.img = "buoy_robot_fishes.png";
                  else
                     obj.img = "buoy_robot.png";
               } },
            island: { num: 2, img: "island.png", side: 75, isContainer: true, offsetX: -7, offsetY: 0, zOrder: 0, containerFilter: function(obj) { return obj.isWithdrawable === true; } },
            fishes: { num: 3, img: "fishes.png", side: 60, isWithdrawable: true, offsetY: 2, zOrder: 1 },
            fishes: { num: 4, img: "fishes.png", side: 60, isWithdrawable: true, offsetY: 8, offsetX: 2, zOrder: 1, canBeOutside: true, customDisplay: function(obj) {
                  if(context.hasOn(obj.row, obj.col, function(item) { return item.num == 2; }))
                     obj.offsetX = 0;
               } },
            count_fishes: { num: 5, value: function(obj) {
                  return context.getItemsOn(obj.row, obj.col, function(item) {
                     return item.isWithdrawable === true;
                  }).length;
               }, side: 60, isWritable: true, fontColor: "#ffffff", fontBold: true, zOrder: 3, offsetX: -14, offsetY: -14},
            count_needs: { num: 6, value: function(obj) {
                  return context.getItemsOn(obj.row, obj.col, function(item) {
                     return item.isContainer === true;
                  })[0].containerSize;
               }, side: 60, isWritable: true, fontColor: "#ffffff", fontBold: true, zOrder: 3, offsetX: -15, offsetY: -14},
            obstacle: { num: 7, img: "reef.png", side: 60, isObstacle: true, zOrder: 0 },
            net: { num: 8, img: "net.png", side: 60, zOrder: 2 },
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      flowers: {
         bagInit: {
            count: 200,
            type: "flower"
         },
         backgroundColor: "#BFF4A6",
         borderColor: "#A5D88B",
         itemTypes: {
            green_robot: { img: "green_robot_above.png", side: 70, nbStates: 9, isRobot: true, offsetX: -5, offsetY: 5, zOrder: 2 },
            garden_robot: { img: "garden_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            earth: { num: 2, img: "earth.png", side: 60, isContainer: true, zOrder: 0 },
            flower: { num: 3, img: "flower.png", side: 60, isWithdrawable: true, isObstacle: true, zOrder: 1 },
            flag: { num: 4, img: "flag.png", side: 60, isExit: true, zOrder: 0},
            fixed_flower: { num: 5, img: "fixed_flower.png", side: 60, isObstacle: true, zOrder: 1 },
            number: { num: 6, side: 60, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },

      gems: {
         newBlocks: [
            {
               name: "writeCode",
               strings: {
                  fr: {
                     label: "écrire le code",
                     code: "ecrireCode",
                     description: "ecrireCode(): écrit le code sur la case du robot et ouvre la porte si le code est correct"
                  }
               },
               category: "robot",
               type: "actions",
               block: {
                  name: "writeCode",
                  params: [null]
               },
               func: function(value, callback) {
                  var robot = this.getRobot();
                  var answer = this.getItemsOn(robot.row, robot.col, function(obj) { return obj.answer !== undefined; })[0].answer;
                  var row_door = this.getItemsOn(robot.row, robot.col, function(obj) { return obj.answer !== undefined; })[0].row_door;
                  var col_door = this.getItemsOn(robot.row, robot.col, function(obj) { return obj.answer !== undefined; })[0].col_door;
                  if (answer == value) {
                     var doors = this.getItemsOn(row_door, col_door, function(obj) { return obj.isDoor === true; });
                     for (var iDoor = 0;iDoor < doors.length;iDoor++) {
                        this.destroy(doors[iDoor]);
                     }
                  }
                  this.writeNumber(robot.row, robot.col, value);
                  this.waitDelay(callback);
               }
            }
         ],
         backgroundColor: "#BF5E47",
         borderColor: "#96413B",
         itemTypes: {
            yellow_robot: { img: "yellow_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            gem: { num: 3, img: "gem.png", side: 60, isWithdrawable: true, autoWithdraw: true, zOrder: 1 },
            obstacle: { num: 4, img: "obstacle.png", side: 60, isObstacle: true, zOrder: 0 },
            number: { num: 5, side: 60, zOrder: 1 },
            door: { num: 6, img: "door.png", side: 60, isObstacle: true, isDoor:true, zOrder: 0 },
            mask: {num:7, img: "mask.png", isMask: true, side: 60, zOrder: 2},
            board_background: { num: 8, img: "board.png", side: 60, zOrder: 0 },
            code_background: { num: 9, img: "code.png", side: 60, zOrder: 0 },
            board: { side: 60, isWritable: true, zOrder: 1 },
         },
         checkEndCondition: robotEndConditions.checkPickedAllWithdrawables
      },
      help: {
         newBlocks: [
            {
               name: "onGreen",
               strings: {
                  fr: {
                     label: "sur la case verte",
                     code: "surCaseVerte",
                     description: "surCaseVerte(): Le robot est-il sur la case verte ?"
                  },
                  es: {
                     label: "sobre la casilla verde",
                     code: "sobreCasillaVerde",
                     description: "sobreCasillaVerde(): ¿Se encuentra el robot sobre la casilla verde?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onGreen",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isGreen===true;}));
               }
            },
         ],
         itemTypes: {
            green_robot: { img: "green_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            obstacle: { num: 2, img: "wall.png", side: 60, isObstacle: true },
            green: { num: 3, color: "#b5e61d", side: 60, isGreen: true, isExit: true},
            number: { num: 4, side: 60, zOrder: 1 },
            board: {num: 5, side: 60, isWritable: true, zOrder: 1 },
            object: {num: 6, img: "red_cube.png", side : 40, isWithdrawable: true, autoWithdraw: true, offsetX: 10, offsetY: -10, zOrder: 1}
         },
         checkEndCondition: robotEndConditions.checkReachExit
      },
      laser: {
         backgroundColor: "#00015e",
         borderColor: "#40405e",
         itemTypes: {
            robot: { img: "laser_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2, isOpaque: true },
            obstacle: { num: 2, img: "obstacle.png", side: 60, isObstacle: true, isOpaque: true },
            light: { num: 3, img: "off_spot.png", states: ["off_spot.png", "on_spot.png"], isLight: true, state: 0, side: 60 },
            launcher: { num: 5, img: "launcher.png", isLaser: true, side: 60 },
            mirrorN: { num: 6, img: "mirrorN.png", isMirror: true, mirrorFunction: function(dir) { return (14 - dir) % 8; }, side: 60 },
            mirrorZ: { num: 7, img: "mirrorZ.png", isMirror: true, mirrorFunction: function(dir) { return (10 - dir) % 8; }, side: 60 },
            mirrorH: { num: 8, img: "mirrorH.png", isMirror: true, mirrorFunction: function(dir) { return (12 - dir) % 8; }, side: 60 },
            mirrorI: { num: 9, img: "mirrorI.png", isMirror: true, mirrorFunction: function(dir) { return (8 - dir) % 8; }, side: 60 },
            number: { side: 60, zOrder: 1 },
            board_background: { num: 4, color: "#685aa6", side: 60, zOrder: 0 },
         },
         checkEndCondition: robotEndConditions.checkLights
      },
      marbles: {
         bagSize: 1,
         backgroundColor: "#dadada",
         itemTypes: {
            blue_robot: { img: "blue_robot.png", side: 90, nbStates: 1, isRobot: true,  offsetX: -15, offsetY: 15, zOrder: 2 },
            red_robot: { img: "red_robot_alt.png", side: 90, nbStates: 1, isRobot: true,  offsetX: -15, offsetY: 15, zOrder: 2 },
            hole: { num: 3, img: "hole.png", side: 60, isContainer: true, zOrder: 0 },
            marble: { num: 4, img: "marble.png", side: 60, isWithdrawable: true, zOrder: 1 },
            number: { num: 5, side: 60, zOrder: 1 },
            board: { num: 6, side: 60, isWritable: true, zOrder: 1 },
            white: { num: 7, color: "shadow.png", side: 60, zOrder: 0 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      objects_in_space: {
         backgroundColor: "#666699",
         itemTypes: {
            green_robot: { img: "green_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            stars: { num: 3, img: "stars.png", side: 60, zOrder: 0},
            objet1: { num: 4, img: "objet1.png", side: 60, isWithdrawable: true, zOrder: 1 },
            objet2: { num: 5, img: "objet2.png", side: 60, isWithdrawable: true, zOrder: 1 },
            obstacle: { num: 6, img: "asteroide.png", side: 60, isObstacle: true, zOrder: 0 }
         },
         checkEndCondition: robotEndConditions.checkPickedAllWithdrawables
      },
      packages: {
         bagSize: 1,
         containerSize: 1,
         hasGravity: true,
         backgroundColor: "#a0cc97",
         borderColor: "#81a279",
         itemTypes: {
            robot: { img: "package_robot.png", side: 90, nbStates: 9, isRobot: true,  offsetX: -11, zOrder: 4, customDisplay: function(obj) {
                  if(context.bag.length != 0)
                     obj.img = "package_robot_book.png";
                  else
                     obj.img = "package_robot.png";
               } },
            box: { num: 2, img: "cardboard_box.png", side: 60, isContainer: true, zOrder: 2, containerFilter: function(obj) { return obj.isWithdrawable === true; } },
            books: { num: 3, img: "books.png", side: 60, isWithdrawable: true, offsetY: 12, zOrder: 1 },
            books_outside: { num: 4, img: "books.png", side: 60, isWithdrawable: true, offsetY: 12, zOrder: 1, canBeOutside: true, customDisplay: function(obj) {
                  if(context.hasOn(obj.row, obj.col, function(item) { return item.num == 2; }))
                     obj.offsetY = -5;
               } },
            count_books: { num: 5, value: function(obj) {
                  return context.getItemsOn(obj.row, obj.col, function(item) {
                     return item.isWithdrawable === true;
                  }).length;
               }, side: 60, isWritable: true, fontColor: "#752a43", fontBold: true, zOrder: 2, offsetX: -20, offsetY: -12},
            count_needs: { num: 6, value: function(obj) {
                  return context.getItemsOn(obj.row, obj.col, function(item) {
                     return item.isContainer === true;
                  })[0].containerSize;
               }, side: 60, isWritable: true, fontColor: "#666666", fontBold: true, zOrder: 3, offsetX: 0, offsetY: 13},
            platform: { num: 7, img: "shelf.png", side: 60, isObstacle: true, zOrder: 0 },
            square_platform: { num: 8, img: "square_shelf.png", side: 60, isObstacle: true, zOrder: 0 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      paint: {
         newBlocks: [
            {
               name: "onPaint",
               strings: {
                  fr: {
                     label: "peinture sur la case",
                     code: "surPeinture",
                     description: "surPeinture(): Le robot est-il sur une case déjà peinte ?"
                  },
                  es: {
                     label: "casilla pintada",
                     code: "casillaPintada",
                     description: "casillaPintada(): ¿El robot se encuentra sobre una casilla pintada?"
                  }
               },
               category: "robot",
               type: "sensors",
               block: {
                  name: "onPaint",
                  yieldsValue: true
               },
               func: function(callback) {
                  this.callCallback(callback, this.isOn(function(obj) {return obj.isWithdrawable===true;}));
               }
            }
         ],
         bagInit: {
            count: 200,
            type: "paint"
         },
         ignoreBag: true,
         backgroundColor: "#c5e2dd",
         borderColor: "#b4ccc7",
         itemTypes: {
            blue_robot: { img: "blue_robot.png", side: 90, nbStates: 1, isRobot: true, offsetX: -15, offsetY: 15, zOrder: 3 },
            initialPaint: { num: 2, color: "#e3568c", side: 60, isPaint: true, zOrder: 1 },
            marker: { num: 3, img: "dot_black.png", side: 60, isContainer: true, containerFilter: function(item) {return item.type === "paint";}, zOrder: 0 },
            marker_white: { num: 4, img: "dot_white.png", isContainer: true, isFake: true, side: 60, zOrder: 0 },
            paint: { img: "paint.png", side: 60, isWithdrawable: true, zOrder: 1 },
            // paint: { color: "#2e1de5", side: 60, isWithdrawable: true, zOrder: 1 },
            number: { side: 60, zOrder: 2 },
            board_background: { num: 5, color: "#ffffff", side: 60, zOrder: 0 },
            board: { side: 60, isWritable: true, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      pixelArt: {
         newBlocks: (function(names, colors, colorsSecondary, colorsTertiary, translations) {
            var blocks = [];
            for(var iColor = 0;iColor < colors.length;iColor++) {
               blocks.push({
                  name: names[iColor],
                  strings: {
                     fr: {
                        label: translations["fr"][iColor],
                        code: translations["fr"][iColor],
                        description: translations["fr"][iColor] + "(): Peint la case en " + translations["fr"][iColor]
                     }
                  },
                  category: "robot",
                  type: "actions",
                  block: {
                     name: names[iColor], blocklyJson: {"colour": colors[iColor], "colourSecondary": colorsSecondary[iColor], "colourTertiary": colorsTertiary[iColor]}
                  },
                  func: (function(cur_color) { return function(callback) {
                     var robot = this.getRobot();
                     if(infos.allowRewrite === true) {
                        this.withdraw(undefined, false);
                     }
                     else if(this.isOn(function(obj) { return obj.isWithdrawable === true;})) {
                        throw(window.languageStrings.messages.failureRewrite);
                     }

                     this.dropObject({type: "paint", color: cur_color});
                     if (robot.col == context.nbCols - 1) {
                        robot.row = (robot.row + 1) % context.nbRows;
                        robot.col = 0;
                        redisplayItem(robot);
                        this.callCallback(callback);
                     } else {
                        this.forward(callback);
                     };
                  } })(colors[iColor])
               });
            }
            return blocks;
         })(["red", "blue", "yellow", "white", "green", "orange", "pink", "purple", "brown", "grey", "black"],
             ["#ff0000", "#0000ff", "#ffff00", "#ffffff", "#00ff00", "#ff8000", "#ff80ff", "#800080", "#804d00", "#808080", "#000000"],
             ["#efa2a2", "#a2a2ef", "#efefa2", "#efefef", "#a2efa2", "#efb6a2", "#efb6ef", "#b6a2b6", "#b6a9a2", "#b6b6b6", "#a2a2a2"],
             ["#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd", "#dddddd"],
             {fr: ["rouge", "bleu", "jaune", "blanc", "vert", "orange", "rose", "violet", "marron", "gris", "noir"]}),
         backgroundColor: "#ece4ce",
         ignoreBag: true,
         blockingFilter: false,
         itemTypes: {
            green_robot: { img: "cursor.png", side: 60, nbStates: 9, isRobot: true, zOrder: 2 },
            marker_red: { num: 2, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#ff0000";} },
            marker_blue: { num: 3, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#0000ff";} },
            marker_yellow: { num: 4, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#ffff00";} },
            marker_white: { num: 5, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#ffffff";} },
            marker_green: { num: 6, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#00ff00";} },
            marker_orange: { num: 7, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#ff8000";} },
            marker_pink: { num: 8, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#ff80ff";} },
            marker_purple: { num: 9, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#800080";} },
            marker_brown: { num: 10, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#804d00";} },
            marker_grey: { num: 11, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#808080";} },
            marker_black: { num: 12, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.color === "#000000";} },
            paint: { side: 60, isWithdrawable: true, zOrder: 1 },
            marker_paint: { num: 1, side: 60, isContainer: true, zOrder: 0, containerFilter: function(item) {return item.type === "paint";} },
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      rocket: {
         backgroundColor: "#2c293e",
         itemTypes: {
            robot: { img: "space_robot_above.png", side: 74, nbStates: 9, isRobot: true, offsetX: -7, offsetY: 7, zOrder: 2 },
            board_background: { num: 2, color: "#8d8dbe", side: 60, zOrder: 0},
            stars: { num: 3, img: "stars.png", side: 60, zOrder: 1},
            asteroide: { num: 4, img: "asteroide.png", side: 60, isObstacle: true, zOrder: 1 },
            rocket: { num: 5, img: "rocket.png", side: 60, isExit: true, zOrder: 1 },
            // obstacle: { num: 6, img: "obstacle.png", side: 60, isObstacle: true, zOrder: 1 },
            objet1: { num: 7, img: "solar_panel.png", side: 60, isWithdrawable: true, zOrder: 1 },
            objet2: { num: 8, img: "solar_panel.png", side: 60, isWithdrawable: true, zOrder: 1 },
            number: { side: 60, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkBothReachAndCollect
      },
      sokoban: {
         backgroundColor: "#c2c6f2",
         borderColor: "#a4aacd",
         itemTypes: {
            robot: { img: "orange_robot.png", side: 80, nbStates: 9, isRobot: true, offsetX: -11, zOrder: 2 },
            wall: { num: 2, img: "grey_brick_wall.png", side: 60, isObstacle: true, zOrder: 0 },
            marker: { num: 3, img: "marker.png", side: 60, isContainer: true, zOrder: 0 },
            box: { num: 4, img: "box.png", side: 60, isObstacle: true, isPushable: true, isWithdrawable: true, zOrder: 1 },
            number: { num: 5, side: 60, zOrder: 1 }
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      veterinary: {
         bagSize: 1,
         containerSize: 1,
         backgroundColor: "#e8c999",
         borderColor: "#a67d40",
         itemTypes: {
            robot: { img: "veterinary_robot.png", side: 90, nbStates: 9, isRobot: true,  offsetX: -11, zOrder: 2 },
            beaver: { num: 2, img: "beaver.png", side: 70, isContainer: true, zOrder: 0, containerFilter: function(obj) { return obj.isWithdrawable === true; } },
            wood: { num: 3, img: "wood.png", side: 60, isWithdrawable: true,  offsetY: 10, zOrder: 1 },
            wood_outside: { num: 5, img: "wood.png", side: 60, isWithdrawable: true,  offsetY: 10, zOrder: 1, canBeOutside: true },
            tree: { num: 4, img: "tree.png", side: 70, isObstacle: true, offsetY: 5, zOrder: 0 },
            count_wood: { num: 6, value: function(obj) {
                  return context.getItemsOn(obj.row, obj.col, function(item) {
                     return item.isWithdrawable === true;
                  }).length;
               }, side: 60, isWritable: true, fontColor: "#01a665", fontBold: true, zOrder: 1, offsetX: 20, offsetY: 17},
            count_needs: { num: 7, value: function(obj) {
                  return context.getItemsOn(obj.row, obj.col, function(item) {
                     return item.isContainer === true;
                  })[0].containerSize;
               }, side: 60, isWritable: true, fontColor: "#4a90e2", fontBold: true, zOrder: 1, offsetX: -20, offsetY: -17},
         },
         checkEndCondition: robotEndConditions.checkContainersFilled
      },
      wiring: {
         backgroundColor: "#00733f",
         maxWireLength: 100,
         maxTotalLength: 100000,
         itemTypes: {
            red_robot: { img: "red_robot.png", side: 90, nbStates: 1, isRobot: true, offsetX: -15, offsetY: 15, zOrder: 3 },
            wire: { img: "wire.png", side: 60, isWire: true, zOrder: 1},
            black_male: { num: 2, img: "black_male.png", side: 60, zOrder: 0, plugType: 1},
            black_female: { num: 3, img: "black_female.png", side: 60, zOrder: 0, plugType: -1},
            white_male: { num: 4, img: "white_male.png", side: 60, zOrder: 0, plugType: 2},
            white_female: { num: 5, img: "white_female.png", side: 60, zOrder: 0, plugType: -2},
         },
         checkEndCondition: robotEndConditions.checkPlugsWired
      }
   };

   if(infos.newBlocks == undefined)
      infos.newBlocks = [];
   if(infos.maxFallAltitude == undefined)
      infos.maxFallAltitude = 2;
   
   var loadContext = function(name) {
      for(var language in contextStrings[name]) {
         var ctx = contextStrings[name][language];
         for(var type in ctx) {
            if((typeof ctx[type]) === "string") {
               localLanguageStrings[language][type] = ctx[type];
            }
            else {
               if(localLanguageStrings[language][type] == undefined)
                  localLanguageStrings[language][type] = {};
               for(var line in ctx[type]) {
                  localLanguageStrings[language][type][line] = ctx[type][line];
               }
            }
         }
      }
      
      for(var param in contextParams[name]) {
         if(infos[param] === undefined || param == "newBlocks") {
            infos[param] = contextParams[name][param];
         }
         if (param === 'itemTypes') {
            for (var itemType in contextParams[name].itemTypes) {
               if (infos.itemTypes[itemType] === undefined) {
                  infos.itemTypes[itemType] = contextParams[name].itemTypes[itemType];
               }
            }
         }
      }
   };
   
   loadContext("none");
   if(infos.contextType != undefined) {
      loadContext(infos.contextType);
   }
   
   infos.newBlocks.push({
      name: "row",
      type: "sensors",
      block: { name: "row", yieldsValue: 'int' },
      func: function(callback) {
         this.callCallback(callback, 1 + this.getRobot().row);
      }
   });
   
   infos.newBlocks.push({
      name: "col",
      type: "sensors",
      block: { name: "col", yieldsValue: 'int' },
      func: function(callback) {
         this.callCallback(callback, 1 + this.getRobot().col);
      }
   });
   
   infos.newBlocks.push({
      name: "wait",
      type: "actions",
      block: { name: "wait" },
      func: function(callback) {
         this.advanceTime(1);
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "north",
      type: "actions",
      block: { name: "north" },
      func: function(callback) {
         this.north(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "south",
      type: "actions",
      block: { name: "south" },
      func: function(callback) {
         this.south(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "east",
      type: "actions",
      block: { name: "east" },
      func: function(callback) {
         this.east(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "west",
      type: "actions",
      block: { name: "west" },
      func: function(callback) {
         this.west(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "left",
      type: "actions",
      block: { name: "left" },
      func: function(callback) {
         this.turnLeft(callback);
      }
   });

   infos.newBlocks.push({
      name: "right",
      type: "actions",
      block: { name: "right" },
      func: function(callback) {
         this.turnRight(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "turnAround",
      type: "actions",
      block: { name: "turnAround" },
      func: function(callback) {
         this.turnAround(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "forward",
      type: "actions",
      block: { name: "forward" },
      func: function(callback) {
         this.forward(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "backwards",
      type: "actions",
      block: { name: "backwards" },
      func: function(callback) {
         this.backwards(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "jump",
      type: "actions",
      block: { name: "jump" },
      func: function(callback) {
         this.jump(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleInFront",
      type: "sensors",
      block: { name: "obstacleInFront", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.obstacleInFront());
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleEast",
      type: "sensors",
      block: { name: "obstacleEast", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, this.hasOn(robot.row, robot.col + 1, function(obj) { return obj.isObstacle === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleWest",
      type: "sensors",
      block: { name: "obstacleWest", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, this.hasOn(robot.row, robot.col - 1, function(obj) { return obj.isObstacle === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleNorth",
      type: "sensors",
      block: { name: "obstacleNorth", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, this.hasOn(robot.row - 1, robot.col, function(obj) { return obj.isObstacle === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleSouth",
      type: "sensors",
      block: { name: "obstacleSouth", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, this.hasOn(robot.row + 1, robot.col, function(obj) { return obj.isObstacle === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleRight",
      type: "sensors",
      block: { name: "obstacleRight", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         var coords = this.coordsInFront(1);
         this.callCallback(callback, this.hasOn(coords.row, coords.col, function(obj) { return obj.isObstacle === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "obstacleLeft",
      type: "sensors",
      block: { name: "obstacleLeft", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         var coords = this.coordsInFront(3);
         this.callCallback(callback, this.hasOn(coords.row, coords.col, function(obj) { return obj.isObstacle === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "gridEdgeEast",
      type: "sensors",
      block: { name: "gridEdgeEast", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, !this.isInGrid(robot.row, robot.col + 1));
      }
   });
   
   infos.newBlocks.push({
      name: "gridEdgeWest",
      type: "sensors",
      block: { name: "gridEdgeWest", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, !this.isInGrid(robot.row, robot.col - 1));
      }
   });
   
   infos.newBlocks.push({
      name: "gridEdgeNorth",
      type: "sensors",
      block: { name: "gridEdgeNorth", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, !this.isInGrid(robot.row - 1, robot.col));
      }
   });
   
   infos.newBlocks.push({
      name: "gridEdgeSouth",
      type: "sensors",
      block: { name: "gridEdgeSouth", yieldsValue: true },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, !this.isInGrid(robot.row + 1, robot.col));
      }
   });
      
   infos.newBlocks.push({
      name: "platformInFront",
      type: "sensors",
      block: { name: "platformInFront", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.platformInFront());
      }
   });
   
   infos.newBlocks.push({
      name: "platformAbove",
      type: "sensors",
      block: { name: "platformAbove", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.platformAbove());
      }
   });
   
   infos.newBlocks.push({
      name: "withdrawObject",
      type: "actions",
      block: { name: "withdrawObject" },
      func: function(callback) {
         this.withdraw();
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "dropObject",
      type: "actions",
      block: { name: "dropObject" },
      func: function(callback) {
         this.drop();
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "onObject",
      type: "sensors",
      block: { name: "onObject", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.isWithdrawable === true;}));
      }
   });
   
   infos.newBlocks.push({
      name: "onContainer",
      type: "sensors",
      block: { name: "onContainer", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.isContainer === true;}));
      }
   });
   
   infos.newBlocks.push({
      name: "onNumber",
      type: "sensors",
      block: { name: "onNumber", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.value !== undefined;}));
      }
   });
	
	infos.newBlocks.push({
      name: "onLauncher",
      type: "sensors",
      block: { name: "onLauncher", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.isLaser === true;}));
      }
   });
   
   infos.newBlocks.push({
      name: "onWritable",
      type: "sensors",
      block: { name: "onWritable", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.isWritable === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "writeNumber",
      type: "actions",
      block: { name: "writeNumber", params: [null] },
      func: function(value, callback) {
         var robot = this.getRobot();
         this.writeNumber(robot.row, robot.col, value);
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "readNumber",
      type: "sensors",
      block: { name: "readNumber", yieldsValue: 'int' },
      func: function(callback) {
         var robot = this.getRobot();
         this.destroyMask(robot.row, robot.col);
         this.callCallback(callback, this.readNumber(robot.row, robot.col));
      }
   });
   
   infos.newBlocks.push({
      name: "nbWithdrawables",
      type: "sensors",
      block: { name: "nbWithdrawables", yieldsValue: 'int' },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, this.getItemsOn(robot.row, robot.col, function(obj) { return obj.isWithdrawable === true; }).length);
      }
   });
   
   infos.newBlocks.push({
      name: "nbInBag",
      type: "sensors",
      block: { name: "nbInBag", yieldsValue: 'int' },
      func: function(callback) {
         var robot = this.getRobot();
         this.callCallback(callback, context.bag.length);
      }
   });
   
   infos.newBlocks.push({
      name: "containerSize",
      type: "sensors",
      block: { name: "containerSize", yieldsValue: 'int' },
      func: function(callback) {
         var robot = this.getRobot();
         var containers = this.getItemsOn(robot.row, robot.col, function(obj) { return obj.isContainer === true; });
         
         if(containers.length == 0) {
            this.callCallback(callback, 0);
            return;
         }
         
         this.callCallback(callback, containers[0].containerSize);
      }
   });
   
   infos.newBlocks.push({
      name: "pushObject",
      type: "actions",
      block: { name: "pushObject" },
      func: function(callback) {
         this.pushObject(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "pushableInFront",
      type: "sensors",
      block: { name: "pushableInFront", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isInFront(function(obj) { return obj.isPushable === true; }));
      }
   });
   
   infos.newBlocks.push({
      name: "dropInFront",
      type: "actions",
      block: { name: "dropInFront" },
      func: function(callback) {
         this.drop(1, this.coordsInFront());
         this.callCallback(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "dropAbove",
      type: "actions",
      block: { name: "dropAbove" },
      func: function(callback) {
         this.drop(1, {row: this.getRobot().row - 1, col: this.getRobot().col});
         this.callCallback(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "withdrawNum_noShadow",
      type: "actions",
      block: { 
         name: "withdrawNum_noShadow", 
         params: [null]
      },
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         for(var i = 0;i < value;i++) {
            this.withdraw();
         }
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "withdrawNum",
      type: "actions",
      block: { name: "withdrawNum", params: [null], blocklyXml: "<block type='withdrawNum_noShadow'>" +
                              "  <value name='PARAM_0'>" +
                              "    <shadow type='math_number'>" +
                              "      <field name='NUM'>0</field>" +
                              "    </shadow>" +
                              "  </value>" +
                              "</block>"},
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         for(var i = 0;i < value;i++) {
            this.withdraw();
         }
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "dropNum_noShadow",
      type: "actions",
      block: { 
         name: "dropNum_noShadow", 
         params: [null]
      },
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         this.drop(value);
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "dropNum",
      type: "actions",
      block: { name: "dropNum", params: [null], blocklyXml: "<block type='dropNum_noShadow'>" +
                              "  <value name='PARAM_0'>" +
                              "    <shadow type='math_number'>" +
                              "      <field name='NUM'>0</field>" +
                              "    </shadow>" +
                              "  </value>" +
                              "</block>"},
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         this.drop(value);
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "shoot_noShadow",
      type: "actions",
      block: { 
         name: "shoot_noShadow", 
         params: [null]
      },
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         if(this.isOn(function(obj) { return obj.isLaser === true; })) {
            this.shoot(this.getRobot().row, this.getRobot().col, value);
            if(this.display) {
               var robot = this.getRobot();
               var lasers = context.getItemsOn(robot.row, robot.col, function(obj) {
                  return obj.isLaser === true;
               });
               
               if(lasers.length != 0) {
                  lasers[0].element.toFront();
               }
               
               robot.element.toFront();
            }
         }
         else {
            throw(window.languageStrings.messages.failureLaser);
         }
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "shoot",
      type: "actions",
      block: { name: "shoot", params: [null], blocklyXml: "<block type='shoot_noShadow'>" +
                              "  <value name='PARAM_0'>" +
                              "    <shadow type='math_number'>" +
                              "      <field name='NUM'>0</field>" +
                              "    </shadow>" +
                              "  </value>" +
                              "</block>"},
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         if(this.isOn(function(obj) { return obj.isLaser === true; })) {
            this.shoot(this.getRobot().row, this.getRobot().col, value);
            if(this.display) {
               var robot = this.getRobot();
               var lasers = context.getItemsOn(robot.row, robot.col, function(obj) {
                  return obj.isLaser === true;
               });
               
               if(lasers.length != 0) {
                  lasers[0].element.toFront();
               }
               
               robot.element.toFront();
            }
         }
         else {
            throw(window.languageStrings.messages.failureLaser);
         }
         this.waitDelay(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "shootCondition_noShadow",
      type: "actions",
      block: { 
         name: "shootCondition_noShadow", 
         params: [null],
         yieldsValue: true
      },
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         
         if(this.isOn(function(obj) { return obj.isLaser === true; })) {
            var retour = this.shoot(this.getRobot().row, this.getRobot().col, value);
            if(this.display) {
               var robot = this.getRobot();
               var lasers = context.getItemsOn(robot.row, robot.col, function(obj) {
                  return obj.isLaser === true;
               });
               
               if(lasers.length != 0) {
                  lasers[0].element.toFront();
               }
               
               robot.element.toFront();
            }
            this.waitDelay(callback, retour);
         }
         else {
            throw(window.languageStrings.messages.failureLaser);
            this.callCallback(callback);
         }
      }
   });
   
   infos.newBlocks.push({
      name: "shootCondition",
      type: "actions",
      block: { name: "shootCondition", blocklyXml: "<block type='shootCondition_noShadow'>" +
                              "  <value name='PARAM_0'>" +
                              "    <shadow type='math_number'>" +
                              "      <field name='NUM'>0</field>" +
                              "    </shadow>" +
                              "  </value>" +
                              "</block>"},
      func: function(value, callback) {
         if((typeof value) == "function") {
            this.callCallback(value);
            return;
         }
         
         if(this.isOn(function(obj) { return obj.isLaser === true; })) {
            var retour = this.shoot(this.getRobot().row, this.getRobot().col, value);
            if(this.display) {
               var robot = this.getRobot();
               var lasers = context.getItemsOn(robot.row, robot.col, function(obj) {
                  return obj.isLaser === true;
               });
               
               if(lasers.length != 0) {
                  lasers[0].element.toFront();
               }
               
               robot.element.toFront();
            }
            this.waitDelay(callback, retour);
         }
         else {
            throw(window.languageStrings.messages.failureLaser);
            this.callCallback(callback);
         }
      }
   });
   
   infos.newBlocks.push({
      name: "connect",
      type: "actions",
      block: { name: "connect" },
      func: function(callback) {
         this.connect();
         this.callCallback(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "onMale",
      type: "sensors",
      block: { name: "onMale", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.plugType > 0; }));
      }
   });
   
   infos.newBlocks.push({
      name: "onFemale",
      type: "sensors",
      block: { name: "onFemale", yieldsValue: true },
      func: function(callback) {
         this.callCallback(callback, this.isOn(function(obj) { return obj.plugType < 0; }));
      }
   });

   infos.newBlocks.push({
      name: "dropPlatformInFront",
      type: "actions",
      block: { name: "dropPlatformInFront" },
      func: function(callback) {
         if(this.nbPlatforms == 0)
            throw(window.languageStrings.messages.failureNotEnoughPlatform);
            
         var coords = {row: this.coordsInFront().row + 1, col: this.coordsInFront().col};
         if(this.getItemsOn(coords.row, coords.col, function(item) { return item.isObstacle === true; }).length != 0) {
            throw(window.languageStrings.messages.failureDropPlatform);
         }
         this.nbPlatforms -= 1;
         this.dropObject({type: "platform"}, coords);
         this.callCallback(callback);
      }
   });
   
   infos.newBlocks.push({
      name: "dropPlatformAbove",
      type: "actions",
      block: { name: "dropPlatformAbove" },
      func: function(callback) {
         if(this.nbPlatforms == 0)
            throw(window.languageStrings.messages.failureNotEnoughPlatform);
            
         var coords = {row: this.getRobot().row - 1, col: this.getRobot().col};
         if(this.getItemsOn(coords.row, coords.col, function(item) { return item.isObstacle === true; }).length != 0) {
            throw(window.languageStrings.messages.failureDropPlatform);
         }
         this.nbPlatforms -= 1;
         this.dropObject({type: "platform"}, coords);
         this.callCallback(callback);
      }
   });
   
   var context = quickAlgoContext(display, infos);
   context.robot = {};
   context.customBlocks = {
      robot: {
         actions: [],
         sensors: []
      }
   };

   // Export context params and strings so that they are available for migration purposes
   context.contextParams = contextParams;
   context.contextStrings = contextStrings;

   for(var command in infos.newBlocks) {
      cmd = infos.newBlocks[command];
      context.customBlocks.robot[cmd.type].push(cmd.block);
      if(cmd.strings) {
         for(var language in cmd.strings) {
            for(var type in cmd.strings[language]) {
               localLanguageStrings[language][type][cmd.name] = cmd.strings[language][type];
            }
         }
      }
      if(cmd.func) {
         context.robot[cmd.name] = cmd.func.bind(context);
      }
   }
   
   var strings = context.setLocalLanguageStrings(localLanguageStrings);
   
   if(infos.languageStrings != undefined) {
      context.importLanguageStrings(infos.languageStrings.blocklyRobot_lib, strings);
   }
   
   var cells = [];
   var colsLabels = [];
   var rowsLabels = [];
   var cardLabels = [];
   var scale = 1;
   var paper;
   
   if(infos.leftMargin === undefined) {
      infos.leftMargin = 10;
   }
   if(infos.rightMargin === undefined) {
      infos.rightMargin = 10;
   }
   if(infos.bottomMargin === undefined) {
      infos.bottomMargin = 10;
   }
   if(infos.topMargin === undefined) {
      if(infos.showLabels) {
         infos.topMargin = 10;
      }
      else {
         infos.topMargin = infos.cellSide / 2;
      }
   }
   if (infos.showLabels) {
      infos.leftMargin += infos.cellSide;
      infos.topMargin += infos.cellSide;
   }
   if (infos.showCardinals) {
      infos.leftMargin += infos.cellSide * 1.8;
      infos.topMargin += infos.cellSide;
      infos.rightMargin += infos.cellSide;
      infos.bottomMargin += infos.cellSide;
   }

   var innerState = {};

   switch(infos.blocklyColourTheme) {
      case "bwinf":
         context.provideBlocklyColours = function() {
            return {
               categories: {
                  logic: 100,
                  loops: 180,
                  math: 220,
                  text: 250,
                  lists: 60,
                  colour: 310,
                  variables: 340,
                  functions: 20,
                  actions: 260,
                  sensors : 340,
                  _default: 280
               },
               blocks: {}
            };
         }
         break;
      default:
   }
   
   context.reset = function(gridInfos) {
      if(gridInfos) {
         context.gridInfos = gridInfos;
         context.tiles = gridInfos.tiles;
         context.initItems = gridInfos.initItems;
         context.nbRows = context.tiles.length;
         context.nbCols = context.tiles[0].length;
      }
      context.nbPlatforms = infos.nbPlatforms;
      
      context.items = [];
      context.multicell_items = [];
      
      context.last_connect = undefined;
      context.wires = [];
      
      context.lost = false;
      context.success = false;
      context.nbMoves = 0;
      context.time = 0;
      context.bag = [];
      
      if(infos.bagInit != undefined) {
         for(var i = 0;i < infos.bagInit.count;i++) {
            var item = {};
            
            var initItem = infos.itemTypes[infos.bagInit.type];
            
            item.type = infos.bagInit.type;
            item.side = 0;
            item.offsetX = 0;
            item.offsetY = 0;
            item.nbStates = 1;
            item.zOrder = 0;
            for(var property in initItem) {
               item[property] = initItem[property];
            }
            
            context.bag.push(item);
         }
      }
      
      if(context.display) {
         this.delayFactory.destroyAll();
         this.raphaelFactory.destroyAll();
         if(paper !== undefined)
            paper.remove();
         paper = this.raphaelFactory.create("paperMain", "grid", infos.cellSide * context.nbCols * scale, infos.cellSide * context.nbRows * scale);
         resetBoard();
         resetItems();
         context.updateScale();
         $("#nbMoves").html(context.nbMoves);
      }
      else {
         resetItems();
      }
   };

   context.redrawDisplay = function() {
      if(context.display) {
         this.raphaelFactory.destroyAll();
         if(paper !== undefined)
            paper.remove();
         paper = this.raphaelFactory.create("paperMain", "grid", infos.cellSide * context.nbCols * scale, infos.cellSide * context.nbRows * scale);
         resetBoard();
         redisplayAllItems();
         context.updateScale();
         $("#nbMoves").html(context.nbMoves);
      }
   }

   context.getInnerState = function() {
      var removeItemElement = function (item) {
         var modifiedItem = Object.assign({}, item);
         delete modifiedItem.element;
         return modifiedItem;
      };
      innerState.items = context.items.map(removeItemElement);
      innerState.multicell_items = context.multicell_items.map(removeItemElement);
      innerState.last_connect = context.last_connect;
      innerState.wires = context.wires.map(removeItemElement);
      innerState.nbMoves = context.nbMoves;
      innerState.time = context.time;
      innerState.bag = context.bag.map(removeItemElement);

      return innerState;
   };

   context.implementsInnerState = function () {
      return true;
   }

   context.reloadInnerState = function(data) {
      innerState = data;
      context.items = data.items;
      context.multicell_items = data.multicell_items;
      context.last_connect = data.last_connect;
      context.wires = data.wires;
      context.nbMoves = data.nbMoves;
      context.time = data.time;
      context.bag = data.bag;
   };

   context.unload = function() {
      if(context.display && paper != null) {
         paper.remove();
      }
   };
   
   var itemAttributes = function(item) {
      var itemType = infos.itemTypes[item.type];
      var x = (infos.cellSide * item.col + item.offsetX + infos.leftMargin) * scale;
      var y = (infos.cellSide * item.row - (item.side - infos.cellSide) + item.offsetY + infos.topMargin) * scale;
      var xClip = x;
      if(item.dir != undefined) {
         var dirToState = [0, 2, 4, 6];
         x = x - (dirToState[item.dir] * item.side * scale);
      }
      var clipRect = "" + xClip + "," + y + "," + (item.side * scale) + "," + (item.side * scale);
      if((!itemType.img && !item.img) && (!itemType.color && !item.color)) {
         x += item.side * scale / 2;
         y += item.side * scale / 2;
      }
      
      var ret = {x: x, y: y, width: item.side * item.nbStates * scale, height: item.side * scale, "clip-rect": clipRect};
      return ret;
   }
   
   var resetBoard = function() {
      for(var iRow = 0;iRow < context.nbRows;iRow++) {
         cells[iRow] = [];
         for(var iCol = 0;iCol < context.nbCols;iCol++) {
            cells[iRow][iCol] = paper.rect(0, 0, 10, 10);
            if(context.tiles[iRow][iCol] == 0)
               cells[iRow][iCol].attr({'stroke-width': '0'});
            if(infos.backgroundColor && context.tiles[iRow][iCol] != 0)
               cells[iRow][iCol].attr({'fill': infos.backgroundColor});
            if(infos.noBorders) {
               if (context.tiles[iRow][iCol] != 0) {
                  cells[iRow][iCol].attr({'stroke': infos.backgroundColor});
               }
            } else {
               if (infos.borderColor) {
                  cells[iRow][iCol].attr({'stroke': infos.borderColor});
               }
            }
            
         }
      }
      if(infos.showLabels) {
         for(var iRow = 0;iRow < context.nbRows;iRow++) {
            rowsLabels[iRow] = paper.text(0, 0, (iRow + 1));
         }
         for(var iCol = 0;iCol < context.nbCols;iCol++) {
            colsLabels[iCol] = paper.text(0, 0, (iCol + 1));
         }
      }
      if (infos.showCardinals) {
         cardLabels = [
            paper.text(0, 0, strings.cardinals.north),
            paper.text(0, 0, strings.cardinals.south),
            paper.text(0, 0, strings.cardinals.west),
            paper.text(0, 0, strings.cardinals.east)
            ];
      }
   };
   
   var resetItem = function(initItem, redisplay) {
      if(redisplay === undefined)
         redisplay = true;
      var item = {};
      context.items.push(item);
      for(var property in initItem) {
         item[property] = initItem[property];
      }
      
      item.side = 0;
      item.offsetX = 0;
      item.offsetY = 0;
      item.nbStates = 1;
      item.zOrder = 0;
      for(var property in infos.itemTypes[item.type]) {
         item[property] = infos.itemTypes[item.type][property];
      }
      
      if(context.display && redisplay) {
         redisplayItem(item);
      }
   };
   
   var resetItems = function() {
      context.items = [];
      var itemTypeByNum = {};
      for(var type in infos.itemTypes) {
         var itemType = infos.itemTypes[type];
         if(itemType.num != undefined) {
            itemTypeByNum[itemType.num] = type;
         }
      }
      for(var iRow = 0;iRow < context.nbRows;iRow++) {
         for(var iCol = 0;iCol < context.nbCols;iCol++) {
            var itemTypeNum = context.tiles[iRow][iCol];
            if(itemTypeByNum[itemTypeNum] != undefined) {
               resetItem({
                  row: iRow,
                  col: iCol,
                  type: itemTypeByNum[itemTypeNum]
               }, false);
            }
         }
      }
      for(var iItem = context.initItems.length - 1;iItem >= 0;iItem--) {
         resetItem(context.initItems[iItem], false);
      }
      
      if(context.display)
         redisplayAllItems();
   };
   
   var resetItemsZOrder = function(row, col) {
      var cellItems = [];
      for(var iItem = context.items.length - 1;iItem >= 0;iItem--) {
         var item = context.items[iItem];
         if((item.row == row) && (item.col == col)) {
            cellItems.push(item);
         }
      }
      cellItems.sort(function(itemA, itemB) {
         if(itemA.zOrder < itemB.zOrder) {
            return -1;
         }
         if(itemA.zOrder > itemB.zOrder) {
            return 1;
         }
         return 0;
      });
      for(var iItem = 0;iItem < cellItems.length;iItem++) {
         if(cellItems[iItem].element)
            cellItems[iItem].element.toFront();
      }
   };

   var redisplayItem = function(item, resetZOrder) {
      if(context.display !== true)
         return;
      if(resetZOrder === undefined)
         resetZOrder = true;
      
      if(item.element !== undefined) {
         item.element.remove();
      }
      var x = (infos.cellSide * item.col + infos.leftMargin) * scale;
      var y = (infos.cellSide * item.row + infos.topMargin) * scale;
      var itemType = infos.itemTypes[item.type];
      
      if(item.customDisplay !== undefined) {
      	item.customDisplay(item);
      }
      
      if(item.img) {
         item.element = paper.image(getImgPath(item.img), x, y, item.side * item.nbStates * scale, item.side * scale);
      }
      else if(item.value !== undefined) {
         var fontColor = item.fontColor;
         if(fontColor === undefined) { fontColor = "black"; }
         
         var value;
         if(typeof(item.value) == "function")
            value = item.value(item);
         else
            value = item.value;
         
         item.element = paper.text(x + item.side * scale / 2, y + item.side * scale / 2, value).attr({
            "font-size": item.side * scale / 2,
            "fill": fontColor,
         });
         
         if(item.fontBold === true) {
            item.element.attr({
               "font-weight": "bold"
            });
         }
      }
      else if(item.color !== undefined) {
         item.element = paper.rect(0, 0, item.side, item.side).attr({"fill": item.color});
      }
      if(item.element !== undefined)
         item.element.attr(itemAttributes(item));
      if(resetZOrder)
         resetItemsZOrder(item.row, item.col);
   };
   
   context.updateScale = function() {
      if(!context.display) {
         return;
      }
      if(paper == null) {
         return;
      }
      
      if(window.quickAlgoResponsive) {
         var areaWidth = Math.max(200, $('#grid').width()-24);
         var areaHeight = Math.max(150, $('#grid').height()-24);
      } else {
         var areaWidth = 400;
         var areaHeight = 600;
      }
      var newCellSide = 0;
      if(context.nbCols && context.nbRows) {
         var marginAsCols = (infos.leftMargin + infos.rightMargin) / infos.cellSide;
         var marginAsRows = (infos.topMargin + infos.bottomMargin) / infos.cellSide;
         newCellSide = Math.min(infos.cellSide, Math.min(areaWidth / (context.nbCols + marginAsCols), areaHeight / (context.nbRows + marginAsRows)));
      }
      scale = newCellSide / infos.cellSide;
      var paperWidth = (infos.cellSide * context.nbCols + infos.leftMargin + infos.rightMargin) * scale;
      var paperHeight = (infos.cellSide * context.nbRows + infos.topMargin + infos.bottomMargin) * scale;
      paper.setSize(paperWidth, paperHeight);
      
      for(var iRow = 0;iRow < context.nbRows;iRow++) {
         for(var iCol = 0;iCol < context.nbCols;iCol++) {
            if(cells[iRow][iCol] === undefined)
               continue;
            var x = (infos.cellSide * iCol + infos.leftMargin) * scale;
            var y = (infos.cellSide * iRow + infos.topMargin) * scale;
            cells[iRow][iCol].attr({x: x, y: y, width: infos.cellSide * scale, height: infos.cellSide * scale});
         }
      }
      var textFontSize = {"font-size": infos.cellSide * scale / 2};
      if(infos.showLabels) {
         for(var iRow = 0;iRow < context.nbRows;iRow++) {
            var x = (infos.leftMargin - infos.cellSide / 2) * scale;
            var y = (infos.cellSide * (iRow + 0.5) + infos.topMargin) * scale;
            rowsLabels[iRow].attr({x: x, y: y}).attr(textFontSize);
         }
         for(var iCol = 0;iCol < context.nbCols;iCol++) {
            var x = (infos.cellSide * iCol + infos.leftMargin + infos.cellSide / 2) * scale;
            var y = (infos.topMargin - infos.cellSide / 2) * scale;
            colsLabels[iCol].attr({x: x, y: y}).attr(textFontSize);
         }
      }
      if (infos.showCardinals) {
         var middleX = (infos.leftMargin + infos.cellSide * context.nbCols / 2) * scale;
         var middleY = (infos.topMargin + infos.cellSide * context.nbRows / 2) * scale;
         cardLabels[0].attr({x: middleX, y: (infos.topMargin - (infos.showLabels ? infos.cellSide : 0) - infos.cellSide / 2) * scale}).attr(textFontSize);
         cardLabels[1].attr({x: middleX, y: paperHeight + (infos.cellSide / 2 - infos.bottomMargin) * scale}).attr(textFontSize);
         cardLabels[2].attr({x: (infos.leftMargin - (infos.showLabels ? infos.cellSide : 0) - infos.cellSide * 1.8 / 2) * scale, y: middleY}).attr(textFontSize);
         cardLabels[3].attr({x: paperWidth + (infos.cellSide / 2 - infos.rightMargin) * scale, y: middleY}).attr(textFontSize);
      }
      
      redisplayAllItems();      
   };
   
   var redisplayAllItems = function() {
      if(context.display !== true)
         return;
      for(var iItem = 0;iItem < context.items.length;iItem++) {
         var item = context.items[iItem];
         redisplayItem(item, false);
      }
      
      for(var iItem = 0;iItem < context.multicell_items.length;iItem++) {
         var item = context.multicell_items[iItem];
         item.redisplay();
      }
      
      var cellItems = [];
      
      for(var iItem = context.items.length - 1;iItem >= 0;iItem--) {
         var item = context.items[iItem];
         cellItems.push(item);
      }
      
      for(var iItem = 0;iItem < context.multicell_items.length;iItem++) {
         var item = context.multicell_items[iItem];
         cellItems.push(item);
      }
      
      cellItems.sort(function(itemA, itemB) {
         if(itemA.zOrder < itemB.zOrder) {
            return -1;
         }
         if(itemA.zOrder > itemB.zOrder) {
            return 1;
         }
         return 0;
      });
      for(var iItem = 0;iItem < cellItems.length;iItem++) {
         if(cellItems[iItem].element !== undefined)
            cellItems[iItem].element.toFront();
      }
   };
   
   context.advanceTime = function(epsilon) {
      var items = [];
      for(var id in context.items) {
         items.push(context.items[id]);
      }
      
      for(var iTime = 0;iTime < epsilon;iTime++) {
         context.time++;
         for(var id in items) {
            if(items[id] !== undefined && items[id].action !== undefined) {
               items[id].action.bind(context)(items[id], context.time);
            }
         }
         
         var robot = this.getRobot();
         if(this.hasOn(robot.row, robot.col, function(item) { return item.isProjectile === true; })) {
            throw(context.strings.messages.failureProjectile);
         }
      }
   };
   
   context.getRobotId = function() {
      for(var id in context.items) {
         if(context.items[id].isRobot != undefined) {
            return id;
         }
      }
      return undefined;
   };
   
   context.getRobot = function() {
      return context.items[context.getRobotId()];
   };
   
   context.getInfo = function(name) {
      return infos[name];
   };
   
   context.setInfo = function(name, value) {
      infos[name] = value;
   };
   
   context.hasOn = function(row, col, filter) {
      for(var id in context.items) {
         var item = context.items[id];
         if(item.row == row && item.col == col && filter(item)) {
            return true;
         }
      }
      return false;
   };
   
   context.setIndexes = function() {
      for(var id in context.items) {
         var item = context.items[id];
         item.index = id;
      }
   }
   
   context.getItemsOn = function(row, col, filter) {
      if(filter === undefined) {
         filter = function(obj) { return true; };
      }
      var selected = [];
      for(var id in context.items) {
         var item = context.items[id];
         if(item.row == row && item.col == col && filter(item)) {
            selected.push(item);
         }
      }
      return selected;
   };

   context.countFilter = function (filter) {
      var filterCount = 0;
      for(var row = 0;row < context.nbRows;row++) {
         for(var col = 0;col < context.nbCols;col++) {
            filterCount += context.getItemsOn(row, col, function(obj) { return filter(obj); }).length;
         }
      }

      for(var item in context.bag) {
         filterCount += filter(context.bag[item]) ? 1 : 0;
      }

      return filterCount;
   };

   context.isOn = function(filter) {
      var item = context.getRobot();
      return context.hasOn(item.row, item.col, filter);
   };
   
   context.isInFront = function(filter) {
      var coords = context.coordsInFront();
      return context.hasOn(coords.row, coords.col, filter);
   };
   
   context.isInGrid = function(row, col) {
      if(row < 0 || col < 0 || row >= context.nbRows || col >= context.nbCols) {
         return false;
      }
      if (context.tiles[row][col] == 0) {
         return false;
      }
      return true;
   };
   
   context.tryToBeOn = function(row, col) {
      // Returns whether the robot can move to row, col
      // true : yes, false : no but move ignored, string : no and throw error
      if(!context.isInGrid(row, col)) {
         if(infos.ignoreInvalidMoves)
            return false;
         return strings.messages.leavesGrid;
      }
      
      if(context.hasOn(row, col, function(item) { return item.isObstacle === true; })) {
         if(infos.ignoreInvalidMoves)
            return false;
         return strings.messages.obstacle;
      }
      
      if(context.hasOn(row, col, function(item) { return item.isProjectile === true; })) {
         if(infos.ignoreInvalidMoves)
            return false;
         return strings.messages.failureProjectile;
      }
      return true;
   };
   
   context.coordsInFront = function(dDir, mult) {
      if(dDir === undefined)
         dDir = 0;
      if(mult === undefined)
         mult = 1;
      var item = context.getRobot();
      var lookDir = (item.dir + dDir + 4) % 4;
      var delta = [[0,1],[1,0],[0,-1],[-1,0]];
      return {
         row: item.row + delta[lookDir][0] * mult,
         col: item.col + delta[lookDir][1] * mult
      };
   };
   
   context.isCrossing = function(wireA, wireB) {
      function crossProduct(pointA, pointB, pointC) {
         return (pointB[0] - pointA[0]) * (pointC[1] - pointA[1]) - (pointB[1] - pointA[1]) * (pointC[0] - pointA[0]);
      }
      
      function onLine(segment, point) {
         return (Math.min(segment[0][0], segment[1][0]) <= point[0] && point[0] <= Math.max(segment[0][0], segment[1][0]))
          && (Math.min(segment[0][1], segment[1][1]) <= point[1] && point[1] <= Math.max(segment[0][1], segment[1][1]));
      }
      
      if(crossProduct(wireA[0], wireA[1], wireB[0]) == 0 && crossProduct(wireA[0], wireA[1], wireB[1]) == 0) {
         return onLine(wireA, wireB[0]) || onLine(wireA, wireB[1]) || onLine(wireB, wireA[0]) || onLine(wireB, wireA[1]);
      }
      return (crossProduct(wireA[0], wireA[1], wireB[0])
      * crossProduct(wireA[0], wireA[1], wireB[1]) <= 0) &&
      (crossProduct(wireB[0], wireB[1], wireA[0])
      * crossProduct(wireB[0], wireB[1], wireA[1]) <= 0);
   }
   
   context.moveRobot = function(newRow, newCol, newDir, callback) {
      var iRobot = context.getRobotId();
      var item = context.items[iRobot];
      if (context.display) 
         item.element.toFront();
      var animate = (item.row != newRow) || (item.col != newCol) || (newDir == item.dir);
      
      if((item.dir != newDir) && ((item.row != newRow) || (item.col != newCol))) {
         if(item.dir !== undefined)
            item.dir = newDir;
         if(context.display) {
            var attr = itemAttributes(item);
            item.element.attr(attr);
         }
      }
      
      if(item.dir !== undefined)
         item.dir = newDir;
      
      item.row = newRow;
      item.col = newCol;
      
      context.withdraw(function(obj) { return obj.autoWithdraw === true; }, false);
      
      if(context.display) {
         attr = itemAttributes(item);
         if(infos.actionDelay > 0) {
            if(animate) {
               context.raphaelFactory.animate("animRobot" + iRobot + "_" + Math.random(), item.element, attr, infos.actionDelay);
            } else {
               context.delayFactory.createTimeout("moveRobot" + iRobot + "_" + Math.random(), function() {
                  item.element.attr(attr);
               }, infos.actionDelay / 2);
            }
         } else {
            item.element.attr(attr);
         }
         $("#nbMoves").html(context.nbMoves);
      }
      
      context.advanceTime(1);
      if(callback) {
         context.waitDelay(callback);
      }
   };
   
   context.moveItem = function(item, newRow, newCol) {
      var animate = (item.row != newRow) || (item.col != newCol);
      var robot = context.getRobot();
      if(context.display) {
         resetItemsZOrder(newRow, newCol);
         resetItemsZOrder(item.row, item.col);
         resetItemsZOrder(robot.row, robot.col);
      }
      item.row = newRow;
      item.col = newCol;
      
      if(context.display) {
         if(animate) {
            attr = itemAttributes(item);
            context.raphaelFactory.animate("animItem" + "_" + Math.random(), item.element, attr, infos.actionDelay);
         }
         else {
            attr = itemAttributes(item);
            if(infos.actionDelay > 0) {
               context.delayFactory.createTimeout("moveItem" + "_" + Math.random(), function() {
                  item.element.attr(attr);
               }, infos.actionDelay / 2);
            } else {
               item.element.attr(attr);
            }
         }
      }
   };
   
   context.moveProjectile = function(item) {
      if(!context.isInGrid(item.row + 1, item.col)) {
         context.destroy(item);
      }
      
      if(context.hasOn(item.row + 1, item.col, function(item) { return item.isObstacle === true; } )) {
         context.destroy(item);
         context.dropObject({type: "dispersion"}, {row: item.row + 1, col: item.col});
         return;
      }
      
      if(context.hasOn(item.row + 1, item.col, function(item) { return item.isRobot === true; } )) {
         context.destroy(item);
         context.dropObject({type: "dispersion_robot"}, {row: item.row + 1, col: item.col});
         return;
      }
      
      context.moveItem(item, item.row + 1, item.col);
      return;
   };
   
   context.destroy = function(item) {
      context.setIndexes();
      context.items.splice(item.index, 1);

      if(context.display) {
         item.element.remove();
      }
   };
   
   context.fall = function(item, row, col, callback) {
      var startRow = row;
      var platforms = context.getItemsOn(row + 1, col, function(obj) { return obj.isObstacle === true; });
      
      while(context.isInGrid(row + 1, col) && platforms.length == 0) {
         row++;
         platforms = context.getItemsOn(row + 1, col, function(obj) { return obj.isObstacle === true; });
      }
      
      if(!context.isInGrid(row + 1, col)) {
         throw(context.strings.messages.falls);
      }
      
      if(row - startRow > infos.maxFallAltitude) {
         throw(context.strings.messages.willFallAndCrash);
      }
      context.nbMoves++;
      context.moveRobot(row, col, item.dir, callback);
   };
   
   context.jump = function(callback) {
      if(!infos.hasGravity) {
         throw("Error: can't jump without gravity");
      }
      
      var item = context.getRobot();
      if(!context.isInGrid(item.row - 1, item.col)) {
         throw(context.strings.messages.jumpOutsideGrid);
      }
      var obstacle = context.getItemsOn(item.row - 2, item.col, function(obj) { return obj.isObstacle === true || obj.isProjectile === true; });
      if(obstacle.length > 0) {
         throw(context.strings.messages.jumpObstacleBlocking);
      }
      var platforms = context.getItemsOn(item.row - 1, item.col, function(obj) { return obj.isObstacle === true; });
      if(platforms.length == 0) {
         throw(context.strings.messages.jumpNoPlatform);
      }
      context.nbMoves++;
      context.moveRobot(item.row - 2, item.col, item.dir, callback);
   };
   
   context.withdraw = function(filter, errorWhenEmpty) {
      if(filter === undefined) {
         filter = function(obj) { return true; };
      }
      if(errorWhenEmpty === undefined) {
         errorWhenEmpty = true;
      }
      var item = context.getRobot();
      var withdrawables = context.getItemsOn(item.row, item.col, function(obj) { return obj.isWithdrawable === true && filter(obj); });
      if(withdrawables.length == 0) {
         if(errorWhenEmpty)
            throw(context.strings.messages.nothingToPickUp);
         return;
      }
      
      if(infos.bagSize != undefined && context.bag.length == infos.bagSize) {
         throw(context.strings.messages.tooManyObjects);
      }
      
      var withdrawable = withdrawables[0];
      context.setIndexes();
      context.items.splice(withdrawable.index, 1);
      context.bag.push(withdrawable);
      
      if(context.display) {
         function removeWithdrawable() {
            withdrawable.element.remove();
            var items = context.getItemsOn(item.row, item.col);
            for(var i = 0; i < items.length ; i++) {
               redisplayItem(items[i]);
            }
         }

         if (infos.actionDelay > 0) {
            context.delayFactory.createTimeout("takeItem_" + Math.random(), removeWithdrawable, infos.actionDelay);
         } else {
            removeWithdrawable();
         }
      }
   };
   
   context.checkContainer = function(coords) {
      var containers = context.getItemsOn(coords.row, coords.col, function(obj) { return (obj.isContainer === true) && (!obj.isFake) });
      if(containers.length != 0) {
         
         var container = containers[0];
         if(container.containerSize == undefined && container.containerFilter == undefined) {
            container.containerSize = 1;
         }
         var filter;
         if(container.containerFilter == undefined)
            filter = function(obj) { return obj.isWithdrawable === true; };
         else
            filter = function(obj) { return obj.isWithdrawable === true && container.containerFilter(obj) };
         
         if(container.containerSize != undefined && context.getItemsOn(coords.row, coords.col, filter).length > container.containerSize) {
            throw(window.languageStrings.messages.failureDropObject);
            return;
         }

         

         if(container.containerFilter != undefined) {
            if(context.hasOn(coords.row, coords.col, function(obj) { return obj.isWithdrawable === true && !container.containerFilter(obj) }) && (context.infos.blockingFilter !== false)) {

               throw(window.languageStrings.messages.failureDropObject);
               return;
            }
         }
      }
   };
   
   context.drop = function(count, coords, filter) {
      if(count === undefined) {
         count = 1;
      }
      if(filter === undefined) {
         filter = function(obj) { return true; };
      }
      if(coords == undefined) {
         var item = context.getRobot();
         coords = {row: item.row, col: item.col};
      }
      
      for(var i = 0;i < count;i++) {
         if(context.bag.length == 0) {
            throw(context.strings.messages.emptyBag);
         }
         
         var object = context.bag.pop();
         object.row = coords.row;
         object.col = coords.col;
         var itemsOn = context.getItemsOn(coords.row, coords.col);
         var maxi = object.zOrder;
         for(var item in itemsOn) {
            if(itemsOn[item].isWithdrawable === true && itemsOn[item].zOrder > maxi) {
               maxi = itemsOn[item].zOrder;
            }
            redisplayItem(item);
         }
         
         object.zOrder = maxi + 0.000001;
         resetItem(object, true);
         
         context.checkContainer(coords);
      }
      
      redisplayItem(this.getRobot());
   };
   
   context.dropObject = function(object, coords) {
      if(coords == undefined) {
         var item = context.getRobot();
         coords = {row: item.row, col: item.col};
      }
      
      if(!context.isInGrid(coords.row, coords.col)) {
         throw(window.languageStrings.messages.failureDropOutside);
         return;
      }
      
      object.row = coords.row;
      object.col = coords.col;
      
      var itemsOn = context.getItemsOn(coords.row, coords.col);
      var maxi = object.zOrder;
      if(maxi === undefined) {
         maxi = 0;
      }
      for(var item in itemsOn) {
         if(itemsOn[item].isWithdrawable === true && itemsOn[item].zOrder > maxi) {
            maxi = itemsOn[item].zOrder;
         }
         redisplayItem(item);
      }
      resetItem(object, true);
      context.checkContainer(coords);
      redisplayItem(this.getRobot());
   };
   
   context.turnLeft = function(callback) {
      var robot = context.getRobot();
      context.moveRobot(robot.row, robot.col, (robot.dir + 3) % 4, callback);
   };
   
   context.turnRight = function(callback) {
      var robot = context.getRobot();
      context.moveRobot(robot.row, robot.col, (robot.dir + 1) % 4, callback);
   };
   
   context.turnAround = function(callback) {
      var robot = context.getRobot();
      context.moveRobot(robot.row, robot.col, (robot.dir + 2) % 4, callback);
   };
   
   context.forward = function(callback) {
      var robot = context.getRobot();
      var coords = context.coordsInFront();
      var ttbo = context.tryToBeOn(coords.row, coords.col);
      if(ttbo === true) {
         if(infos.hasGravity) {
            context.fall(robot, coords.row, coords.col, callback);
         } else {
            context.nbMoves++;
            context.moveRobot(coords.row, coords.col, robot.dir, callback);
         }
      } else if(ttbo === false) {
         context.waitDelay(callback);
      } else {
         context.moveRobot(robot.row + (coords.row - robot.row) / 4, robot.col + (coords.col - robot.col) / 4, robot.dir);
         throw ttbo;
      }
   };
   
   context.backwards = function(callback) {
      var robot = context.getRobot();
      var coords = context.coordsInFront(2);
      var ttbo = context.tryToBeOn(coords.row, coords.col);
      if(ttbo === true) {
         if(infos.hasGravity) {
            context.fall(robot, coords.row, coords.col, callback);
         } else {
            context.nbMoves++;
            context.moveRobot(coords.row, coords.col, robot.dir, callback);
         }
      } else if(ttbo === false) {
         context.waitDelay(callback);
      } else {
         context.moveRobot(robot.row + (coords.row - robot.row) / 4, robot.col + (coords.col - robot.col) / 4, robot.dir);
         throw ttbo;
      }
   };
   
   context.north = function(callback) {
      var item = context.getRobot();
      var ttbo = context.tryToBeOn(item.row - 1, item.col);
      if(ttbo === true) {
         context.nbMoves++;
         context.moveRobot(item.row - 1, item.col, 3, callback);
      } else if(ttbo === false) {
         context.waitDelay(callback);
      } else {
         context.moveRobot(item.row - 1/4, item.col, 3);
         throw ttbo;
      }
   };
   
   context.south = function(callback) {
      var item = context.getRobot();
      var ttbo = context.tryToBeOn(item.row + 1, item.col);
      if(ttbo === true) {
         context.nbMoves++;
         context.moveRobot(item.row + 1, item.col, 1, callback);
      } else if(ttbo === false) {
         context.waitDelay(callback);
      } else {
         context.moveRobot(item.row + 1/4, item.col, 1);
         throw ttbo;
      }
   };
   
   context.east = function(callback) {
      var item = context.getRobot();
      var ttbo = context.tryToBeOn(item.row, item.col + 1);
      if(ttbo === true) {
         context.nbMoves++;
         context.moveRobot(item.row, item.col + 1, 0, callback);
      } else if(ttbo === false) {
         context.waitDelay(callback);
      } else {
         context.moveRobot(item.row, item.col + 1/4, 0);
         throw ttbo;
      }
   };
   
   context.west = function(callback) {
      var item = context.getRobot();
      var ttbo = context.tryToBeOn(item.row, item.col - 1);
      if(ttbo === true) {
         context.nbMoves++;
         context.moveRobot(item.row, item.col - 1, 2, callback);
      } else if(ttbo === false) {
         context.waitDelay(callback);
      } else {
         context.moveRobot(item.row, item.col - 1/4, 2);
         throw ttbo;
      }
   };
   
   context.obstacleInFront = function() {
      return context.isInFront(function(obj) { return obj.isObstacle === true; });
   };
   
   context.platformInFront = function() {
      var coords = context.coordsInFront();
      return context.hasOn(coords.row + 1, coords.col, function(obj) { return obj.isObstacle === true; });
   };
   
   context.platformAbove = function() {
      var robot = context.getRobot();
      return context.hasOn(robot.row - 1, robot.col, function(obj) { return obj.isObstacle === true; });
   };
   
   context.writeNumber = function(row, col, value) {
      var numbers = context.getItemsOn(row, col, function(obj) { return obj.isWritable === true; });
      
      if(numbers.length == 0) {
         throw(strings.messages.failureWriteHere);
      }
      
      var number = numbers[0];
      number.value = value;
      if(context.display) {
         redisplayItem(number);
      }
   };
   
   context.readNumber = function(row, col) {
      var numbers = context.getItemsOn(row, col, function(obj) { return obj.value !== undefined; });
      
      if(numbers.length == 0) {
         throw(strings.messages.failureReadHere);
      }
      
      return parseInt(numbers[0].value);
   };
   
   context.pushObject = function(callback) {
      var robot = context.getRobot();
      var coords = context.coordsInFront();
      
      var items = context.getItemsOn(coords.row, coords.col, function(obj) { return obj.isPushable === true ; });
      
      if(items.length == 0) {
         throw(strings.messages.failureNothingToPush);
      }
      
      var coordsAfter = context.coordsInFront(0, 2);
      
      if(!context.isInGrid(coordsAfter.row, coordsAfter.col))
         throw(strings.messages.failureWhilePushing);
      if(context.hasOn(coordsAfter.row, coordsAfter.col, function(obj) { return obj.isObstacle === true; } ))
         throw(strings.messages.failureWhilePushing);
      if(context.tiles[coordsAfter.row][coordsAfter.col] == 0)
         throw(strings.messages.failureWhilePushing);
      
      context.moveItem(items[0], coordsAfter.row, coordsAfter.col);
      
      context.forward(callback);
   };
   
   context.shoot = function(lig, col, dir) {
      dir = dir % 8;
      var dirs = [
         [-1, 0],
         [-1, 1],
         [0, 1],
         [1, 1],
         [1, 0],
         [1, -1],
         [0, -1],
         [-1, -1]
      ];
      
      var lights = context.getItemsOn(lig, col, function(obj) {
         return obj.isLight === true;
      });
      
      for(var light in lights) {
         lights[light].state = 1;
         lights[light].img = lights[light].states[lights[light].state];
         if(context.display)
            redisplayItem(lights[light]);
      }
      
      var x = (infos.cellSide * (col + 0.5) + infos.leftMargin) * scale;
      var y = (infos.cellSide * (lig + 0.5) + infos.topMargin) * scale;
      
      var taille = infos.cellSide;
      
      var findRobot = false;
      
      var plig = lig + dirs[dir][0];
      var pcol = col + dirs[dir][1];
      if(!context.isInGrid(plig, pcol) || context.hasOn(plig, pcol, function(obj) { return obj.isOpaque === true; })) {
         taille /= 2;
         
         findRobot = context.hasOn(plig, pcol, function(obj) { return obj.isRobot === true; });
      }
      else {
         var pdir = dir;
         var mirrors = context.getItemsOn(plig, pcol, function(obj) { return obj.isMirror === true; });
         if(mirrors.length != 0) {
            pdir = mirrors[0].mirrorFunction(dir);
         }
         
         findRobot = context.hasOn(plig, pcol, function(obj) { return obj.isRobot === true; });
         
         if(context.shoot(plig, pcol, pdir)) {
            findRobot = true;
         }
      }
      
      var dx = (taille * dirs[dir][1]) * scale;
      var dy = (taille * dirs[dir][0]) * scale;
      
      if(context.display && paper != undefined) {
         var segment = paper.path("M " + x + " " + y + " l " + dx + " " + dy);
         
         segment.attr({'stroke-width': 5, 'stroke': '#ffff93'});
         
         context.delayFactory.createTimeout("deleteSegement_" + Math.random(), function() {
            segment.remove();
         }, infos.actionDelay * 2);
      }
      
      return findRobot;
   };
   
   context.connect = function() {
      var robot = context.getRobot();
      
      var plugs = context.getItemsOn(robot.row, robot.col, function(obj) { return obj.plugType !== undefined ; });
      
      if(plugs.length == 0) {
         throw(strings.messages.failureNoPlug);
      }
      
      var wires = context.getItemsOn(robot.row, robot.col, function(obj) { return obj.isWire === true; });
      
      if(wires.length != 0) {
         throw(strings.messages.failureAlreadyWired);
      }
      
      this.dropObject({type: "wire", zOrder: 1});
      
      if(this.last_connect !== undefined) {
         if(this.last_connect.plugType + plugs[0].plugType != 0)
            throw(strings.messages.failureWrongPlugType);
            
         function segmentLength(segment) {
            return Math.sqrt((segment[0][0] - segment[1][0]) * (segment[0][0] - segment[1][0]) + (segment[0][1] - segment[1][1]) * (segment[0][1] - segment[1][1]));
         }
         
         var wire = [[this.last_connect.row, this.last_connect.col],[plugs[0].row, plugs[0].col]];
         
         if(segmentLength(wire) > infos.maxWireLength) {
            throw(strings.messages.failureWireTooLong);
         }
         
         var totalLength = segmentLength(wire);
         for(var iWire = 0;iWire < this.wires.length;iWire++) {
            if(this.isCrossing(wire, this.wires[iWire])) {
               throw(strings.messages.failureWireCrossing);
            }
            totalLength += segmentLength(this.wires[iWire]);
         }
         
         if(totalLength > infos.maxTotalLength) {
            throw(strings.messages.failureTotalLengthExceeded);
         }
         
         this.wires.push(wire);
         
         var x = (this.last_connect.col + 0.5) * infos.cellSide + infos.leftMargin;
         var y = (this.last_connect.row + 0.5) * infos.cellSide + infos.topMargin;
         var dx = (plugs[0].col - this.last_connect.col) * infos.cellSide;
         var dy = (plugs[0].row - this.last_connect.row) * infos.cellSide;
         
         var wire_item = {zOrder: 2};
         wire_item.redisplay = function() {
            wire_item.element = paper.path("M " + (x * scale) + " " + (y * scale) + " l " + (dx * scale) + " " + (dy * scale));
            wire_item.element.attr({'stroke-width': 5, 'stroke': '#dd0000'});
         };
         
         this.multicell_items.push(wire_item);
         redisplayAllItems();
         
         this.last_connect = undefined;
      }
      else {
         this.last_connect = plugs[0];
      }
   };

   window.taskGetResourcesPost = function (res, callback) {
      var typeData = contextParams[infos.contextType];
      if (typeData.itemTypes) {
         for (var key in typeData.itemTypes) {
            var params = typeData.itemTypes[key];
            if (params.img) {
               var imgPath = params.img;
               if (window.modulesPath) {
                  imgPath = window.modulesPath + "img/algorea/" + imgPath;
               }
               res.task.push({ type: 'image', url: imgPath });
            }
         }
      }
      callback(res);
   }

   return context;
};

// Is this really used? How? It seems only window.taskGetResourcesPost is used
var getResources = function(subTask) {
   var res = [];
   var type = subTask.gridInfos.contextType;
   var typeData = contextParams[type];
   if(typeData.itemTypes){
      for(var key in typeData.itemTypes){
         var params = typeData.itemTypes[key];
         if(params.img){
            res.task.push({ type: 'image', url: params.img });
         }
      }
   }
   return res
};

var robotEndConditions = {
   checkReachExit: function(context, lastTurn) {
      var robot = context.getRobot();
      if(context.isOn(function(obj) { return obj.isExit === true; })) {
         context.success = true;
         throw(window.languageStrings.messages.successReachExit);
      }
      if(lastTurn) {
         context.success = false;
         throw(window.languageStrings.messages.failureReachExit);
      }
   },
   checkPickedAllWithdrawables: function(context, lastTurn) {
      var solved = true;
      for(var row = 0;row < context.nbRows;row++) {
         for(var col = 0;col < context.nbCols;col++) {
            if(context.hasOn(row, col, function(obj) { return obj.isWithdrawable === true; })) {
               solved = false;
            }
         }
      }

      var filterCount = context.countFilter(function(obj) { return obj.isWithdrawable === true; });

      if(solved) {
         context.success = true;
         throw(filterCount <= 1 && window.languageStrings.messages.successPickedAllWithdrawablesSingular
             ? window.languageStrings.messages.successPickedAllWithdrawablesSingular
             : window.languageStrings.messages.successPickedAllWithdrawables);
      }
      if(lastTurn) {
         context.success = false;
         throw(filterCount <= 1 && window.languageStrings.messages.failurePickedAllWithdrawablesSingular
             ? window.languageStrings.messages.failurePickedAllWithdrawablesSingular
             : window.languageStrings.messages.failurePickedAllWithdrawables);
      }
   },
   checkPlugsWired: function(context, lastTurn) {
      var solved = true;
      for(var row = 0;row < context.nbRows;row++) {
         for(var col = 0;col < context.nbCols;col++) {
            if(context.hasOn(row, col, function(obj) { return obj.plugType !== undefined; }) && !context.hasOn(row, col, function(obj) { return obj.isWire === true; })) {
               solved = false;
            }
         }
      }
      
      if(solved) {
         context.success = true;
         throw(window.languageStrings.messages.successPlugsWired);
      }
      if(lastTurn) {
         context.success = false;
         throw(window.languageStrings.messages.failurePlugsWired);
      }
   },
   checkContainersFilled: function(context, lastTurn) {
      var solved = true;
      
      var messages = [
         'failureContainersFilled',
         'failureContainersFilledLess',
         'failureContainersFilledBag'
      ];
      var message = 2;
      if (context.infos.maxMoves != undefined) {
         if (context.nbMoves > context.infos.maxMoves) {
            context.success = false;
            throw(window.languageStrings.messages.failureTooManyMoves + " : " + context.nbMoves);
         }
      }
      var containersCount = 0;
      var notFilledCount = 0;
      var filledLessCount = 0;
      for(var row = 0;row < context.nbRows;row++) {
         for(var col = 0;col < context.nbCols;col++) {
            var containers = context.getItemsOn(row, col, function(obj) { return (obj.isContainer === true) && (!obj.isFake) });
            if(containers.length != 0) {
               var container = containers[0];
               containersCount++;
               if(container.containerSize == undefined && container.containerFilter == undefined) {
                  container.containerSize = 1;
               }
               var filter;
               if(container.containerFilter == undefined)
                  filter = function(obj) { return obj.isWithdrawable === true; };
               else
                  filter = function(obj) { return obj.isWithdrawable === true && container.containerFilter(obj) };
               
               if(container.containerSize != undefined && context.getItemsOn(row, col, filter).length != container.containerSize) {
                  solved = false;
                  message = Math.min(message, 1);
                  filledLessCount++;
               }
               else if(context.getItemsOn(row, col, filter).length == 0) {
                  solved = false;
                  message = Math.min(message, 0);
               }
               
               if(container.containerFilter != undefined) {
                  if(context.hasOn(row, col, function(obj) { return obj.isWithdrawable === true && !container.containerFilter(obj) })) {
                     solved = false;
                     message = Math.min(message, 0);
                  }
                  for(var item in context.bag) {
                     if(filter(context.bag[item]) && context.infos.ignoreBag === undefined) {
                        solved = false;
                        message = Math.min(message, 2);
                     }
                  }
               }
            }
            else {
               if(context.getItemsOn(row, col, function(obj) { return obj.isWithdrawable === true && obj.canBeOutside !== true; }).length > 0) {
                  solved = false;
                  message = Math.min(message, 0);
                  notFilledCount++;
               }
            }
         }
      }

      if(solved) {
         context.success = true;
         throw(window.languageStrings.messages.successContainersFilledSingular && containersCount <= 1
             ? window.languageStrings.messages.successContainersFilledSingular
             : window.languageStrings.messages.successContainersFilled);
      }
      if(lastTurn) {
         context.success = false;

         var singular = (0 === message && notFilledCount <= 1) || (1 === message && filledLessCount <= 1);

         throw(window.languageStrings.messages[messages[message] + 'Singular'] && singular
             ? window.languageStrings.messages[messages[message] + 'Singular']
             : window.languageStrings.messages[messages[message]]);
      }
   },
   checkSpecificCollection: function(context, lastTurn) {
      // Look for a custom filter in the task infos, otherwise accept everything (legacy behavior)
      var filter = context.infos.checkFilter || function(obj) { return true; };
      var filterCount = context.countFilter(filter);

      var solved = true;

      // 1. Check if there are valid items left on the grid (Failure if yes)
      for(var row = 0;row < context.nbRows;row++) {
         for(var col = 0;col < context.nbCols;col++) {
            // We look for items that are withdrawable AND match the criteria
            var remainingTargets = context.getItemsOn(row, col, function(obj) { 
               return obj.isWithdrawable === true && filter(obj); 
            });
            if(remainingTargets.length > 0) {
               solved = false;
            }
         }
      }
      
      // 2. Check if the robot picked up something it shouldn't have
      for(var i = 0; i < context.bag.length; i++) {
         var item = context.bag[i];
         if(!filter(item)) {
            context.success = false;
            throw(window.languageStrings.messages.failureUnfilteredObject); 
         }
      }

      if(solved) {
         context.success = true;
         throw(filterCount <= 1 && window.languageStrings.messages.successPickedAllWithdrawablesSingular
             ? window.languageStrings.messages.successPickedAllWithdrawablesSingular
             : window.languageStrings.messages.successPickedAllWithdrawables);
      }
      if(lastTurn) {
         context.success = false;
         throw(filterCount <= 1 && window.languageStrings.messages.failurePickedAllWithdrawablesSingular
             ? window.languageStrings.messages.failurePickedAllWithdrawablesSingular
             : window.languageStrings.messages.failurePickedAllWithdrawables);
      }
   },
   checkBothReachAndCollect: function(context, lastTurn) {
      var filterCount = context.countFilter(function(obj) { return obj.isWithdrawable === true; });
      var filterExitCount = context.countFilter(function(obj) { return obj.isExit === true; });
      if(0 === filterExitCount || context.isOn(function(obj) { return obj.isExit === true; })) {
         var solved = true;
         for(var row = 0;row < context.nbRows;row++) {
            for(var col = 0;col < context.nbCols;col++) {
               if(context.hasOn(row, col, function(obj) { return obj.isWithdrawable === true; })) {
                  solved = false;
                  throw(filterCount <= 1 && window.languageStrings.messages.failurePickedAllWithdrawablesSingular
                     ? window.languageStrings.messages.failurePickedAllWithdrawablesSingular
                     : window.languageStrings.messages.failurePickedAllWithdrawables);
               }
            }
         }
         
         if(solved) {
            context.success = true;
            throw(filterCount <= 1 && window.languageStrings.messages.successPickedAllWithdrawablesSingular
                ? window.languageStrings.messages.successPickedAllWithdrawablesSingular
                : window.languageStrings.messages.successPickedAllWithdrawables);
         }
      }
      if(lastTurn) {
         context.success = false;
         throw(window.languageStrings.messages.failureReachExit);
      }
   },
   checkLights: function(context, lastTurn) {
      var solved = true;
      for(var row = 0;row < context.nbRows;row++) {
         for(var col = 0;col < context.nbCols;col++) {
            if(context.hasOn(row, col, function(obj) { return obj.isLight === true && obj.state === 0; })) {
               solved = false;
            }
         }
      }
      
      if(solved) {
         context.success = true;
         throw(window.languageStrings.messages.successLights);
      }
      if(lastTurn) {
         context.success = false;
         throw(window.languageStrings.messages.failureLights);
      }
   }
};


var robotEndFunctionGenerator = {
   allFilteredPicked: function(filter) {
      return function(context, lastTurn) {
         var solved = true;
         for(var row = 0;row < context.nbRows;row++) {
            for(var col = 0;col < context.nbCols;col++) {
               var filtered = context.getItemsOn(row, col, function(obj) { return obj.isWithdrawable && filter(obj); })
               if(filtered.length != 0) {
                  solved = false;
               }
            }
         }

         for(var item in context.bag) {
            if(!filter(context.bag[item])) {
               context.success = false;
               throw(window.languageStrings.messages.failureUnfilteredObject);
            }
         }

         var filterCount = context.countFilter(filter);

         if(solved) {
            context.success = true;
            throw(filterCount <= 1 && window.languageStrings.messages.successPickedAllWithdrawablesSingular
                ? window.languageStrings.messages.successPickedAllWithdrawablesSingular
                : window.languageStrings.messages.successPickedAllWithdrawables);
         }
         if(lastTurn) {
            context.success = false;
            throw(filterCount <= 1 && window.languageStrings.messages.failurePickedAllWithdrawablesSingular
               ? window.languageStrings.messages.failurePickedAllWithdrawablesSingular
                : window.languageStrings.messages.failurePickedAllWithdrawables);
         }
      };
   },
   allNumbersWritten: function(numbers) {
      return function(context, lastTurn) {
         var solved = true;
         for(var iNumber in numbers) {
            var number = numbers[iNumber];
            var items = context.getItemsOn(number.row, number.col, function(obj) { return obj.value !== undefined; });
            if(items.length == 0)
               throw("Error: no number here");
            
            var expected;
            if(typeof number.value === "number") {
               expected = number.value;
            } else {
               expected = number.value.bind(context)();
            }
            
            if(expected != items[0].value) {
               solved = false;
            }
         }
         
         if(solved) {
            context.success = true;
            throw(window.languageStrings.messages.successNumbersWritten);
         }
         
         if(lastTurn) {
            context.success = false;
            throw(window.languageStrings.messages.failureNumbersWritten);
         }
      };
   }
};

if(window.quickAlgoLibraries) {
   quickAlgoLibraries.register('robot', getContext);
} else {
   if(!window.quickAlgoLibrariesList) { window.quickAlgoLibrariesList = []; }
   window.quickAlgoLibrariesList.push(['robot', getContext]);
}
