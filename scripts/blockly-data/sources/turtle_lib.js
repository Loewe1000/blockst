

var makeTurtle = function(coords) {
   this.reset = function(stepsize, newcoords) {
      this.x = this.drawingContext && this.drawingContext.canvas.width ? this.drawingContext.canvas.width / 2 : 150;
      this.y = this.drawingContext && this.drawingContext.canvas.height ? this.drawingContext.canvas.height / 2 : 150;

      this.directionDeg = 0;
      this.direction = 0;

      var initcoords = newcoords || coords;
      if(initcoords) {
         this.x = initcoords.x;
         this.y = initcoords.y;

         if (initcoords.dir) {
           this.directionDeg = initcoords.dir;
           this.direction = this.directionDeg*Math.PI/180;
        }
      }

      this.paint = true;
      this.stepsize = 5;

      if (this.drawingContext) {
         this.drawingContext.clearRect(0, 0, this.drawingContext.canvas.width, this.drawingContext.canvas.height);
      }
      if (this.turtle) {
         this.turtle.src = this.turtle.getAttribute("pendown");
         this.turtle.style.transform = "rotate(" + (-this.direction) + "rad)";
         this.placeTurtle();
      }
      if (stepsize) {
         this.stepsize = stepsize;
      }
   }
   this.reset(5);

   this.turn = function(angle) {
      angle = parseInt(angle);
      this.directionDeg = (this.directionDeg + angle) % 360;

      // Make sure we have a positive direction
      this.directionDeg = (this.directionDeg + 360) % 360;

      this.direction = this.directionDeg*Math.PI/180;
      if (this.turtle) {
         this.turtle.style.transform = "rotate(" + (-this.direction) + "rad)";
      }
   }
   this.trig = function() {
      // Fix rounding issues
      if(this.directionDeg == 0) {
         return {sin: 0, cos: 1};
      } else if(this.directionDeg == 90) {
         return {sin: 1, cos: 0};
      } else if(this.directionDeg == 180) {
         return {sin: 0, cos: -1};
      } else if(this.directionDeg == 270) {
         return {sin: -1, cos: 0};
      } else {
         return {sin: Math.sin(this.direction), cos: Math.cos(this.direction)};
      }
   }
   this.move = function(amount) {
      if (this.paint) {
         this.drawingContext.beginPath();
         this.drawingContext.moveTo(this.x, this.y);
      }

      var trig = this.trig();
      this.x -= amount * this.stepsize * 10 * trig.sin;
      this.y -= amount * this.stepsize * 10 * trig.cos;

      if (this.paint) {
         this.drawingContext.lineTo(this.x, this.y);
         this.drawingContext.stroke();
      }

      this.placeTurtle();
   }
   this.jump = function(x, y) {
	   this.x = x;
	   this.y = y;
      this.placeTurtle();
   }
   this.start_painting = function() {
      this.paint = true;
      if(this.turtle) {
         this.turtle.src = this.turtle.getAttribute("pendown");
      }
   }
   this.stop_painting = function() {
      this.paint = false;
      if(this.turtle) {
         this.turtle.src = this.turtle.src = this.turtle.getAttribute("penup");
      }
   }

   this.set_colour = function(colour) {
      this.drawingContext.strokeStyle = colour;
   }
   this.set_stepsize = function(stepsize) {
      this.stepsize = stepsize;
   }
   this.setDrawingContext = function(drawingContext) {
      this.drawingContext = drawingContext;
      this.drawingContext.lineWidth = 3;
      this.drawingContext.lineCap = 'round'
   }
   this.setTurtle = function(turtle) {
      this.turtle = turtle;
      this.placeTurtle();
   }
   this.placeTurtle = function() {
      if(!this.turtle) { return; }
      this.turtle.style.left= this.x - 12 + "px";
      this.turtle.style.top= this.y - 15 + "px";
   }
   this.fixTurtle = function() {
      // Add padding so the turtle styas centered
      this.turtle.style.paddingRight = '2px';
      this.turtle.style.paddingBottom = '3px';
   }
   this.getCoords = function() {
      return {x: this.x, y: this.y};
   }
};


var getContext = function(display, infos) {
   var constants = {
      DEFAULT_CANVAS_SIZE: {
         width: 300, //px
         height: 300 //px
      },
   };

   var localLanguageStrings = {
      fr: {
         turnleft: "droite ↺",
         turnright: "gauche ↻",
         penup: "lever le pinceau",
         pendown: "baisser le pinceau",
         categories: {
            turtle: "Tortue",
            turtleInput: "Entrée"
         },
         label: {
            move: "avancer",
            moveamount: "avancer de %1 pas",
            movebackamount: "reculer de %1 pas",
            moveamountvalue: "avancer de %1 pas",
            movebackamountvalue: "reculer de %1 pas",
            turnleft: "pivoter vers la droite ↺",
            turnright: "pivoter vers la gauche ↻",
            turnleftamount: "pivoter de %1° vers la gauche ↺",
            turnrightamount: "pivoter de %1° vers la droite ↻",
            turnleftamountvalue: "pivoter de %1 vers la gauche ↺",
            turnrightamountvalue: "pivoter de %1 vers la droite ↻",
            turnleftamountvalue_noround: "drehe um %1 Grad nach links ↺",
            turnrightamountvalue_noround: "drehe um %1 Grad nach rechts ↻",
            turnleftamountvalue_options: "drehe um %1 nach links ↺",
            turnrightamountvalue_options: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_moreoptions: "drehe um %1 nach links ↺",
            turnrightamountvalue_moreoptions: "drehe um %1 nach rechts ↻",
            turneitheramount: "pivoter de %1° vers la %2",
            turneitheramountvalue: "pivoter de %1 vers la %2",
            row: "ligne de la tortue",
            col: "colonne de la tortue",
            penup: "lever le pinceau",
            pendown: "baisser le pinceau",
            peneither: "%1",
            colour2: "setze Farbe",
            colourRGB: "mettre la couleur RGB ( %1, %2, %3 )",
            colourRGBAmount: "mettre la couleur RGB ( %1, %2, %3 )",
            colourvalue: "mettre la couleur %1",
            turn: "drehe (Grad) ",
            alert: "messagebox",
            log: "logge",
            inputvalue: "lire un nombre sur l'entrée",
            jump: "aller à la position (%1,%2)",
            jumpAmount: "aller à la position (%1,%2)"
         },
         code: {
            move: "avancer",
            moveamount: "avancer",
            movebackamount: "reculer",
            moveamountvalue: "avancer",
            movebackamountvalue: "reculer",
            turnleft: "pivoterGauche",
            turnright: "pivoterDroite",
            turnleftamount: "gauche",
            turnrightamount: "droite",
            turnleftamountvalue: "gauche",
            turnrightamountvalue: "droite",
            turnleftamountvalue_noround: "dreheLinksGrad",
            turnrightamountvalue_noround: "dreheRechtsGrad",
            turnleftamountvalue_options: "dreheLinksGrad",
            turnrightamountvalue_options: "dreheRechtsGrad",
            turnleftamountvalue_moreoptions: "dreheLinksGrad",
            turnrightamountvalue_moreoptions: "dreheRechtsGrad",
            turneitheramount: "pivoter",
            turneitheramountvalue: "pivoter",
            row: "ligneTortue",
            col: "colonneTortue",
            penup: "leverPinceau",
            pendown: "baisserPinceau",
            peneither: "stift",
            colour2: "setzeFarbe",
            colourRGB: "couleurRGB",
            colourRGBAmount: "couleurRGB",
            colourvalue: "couleur",
            turn: "drehe",
            alert: "alert",
            log: "log",
            inputvalue: "eingabewert",
			   jump: "jump",
            jumpAmount: "jump",
         },
         description: {
            moveamount: '@(steps) la tortue avance du nombre de pas indiqué en paramètre. Exemple : @(50)',
            moveamountvalue: '@(steps) la tortue avance du nombre de pas indiqué en paramètre. Exemple : @(50)',
            movebackamount: '@(steps) la tortue recule du nombre de pas indiqué en paramètre. Exemple : @(50)',
            movebackamountvalue: '@(steps) la tortue recule du nombre de pas indiqué en paramètre. Exemple : @(50)',
            turnleftamount: '@(angle) la tortue pivote vers la gauche du nombre de degrés indiqué en paramètre. Exemple : @(90)',
            turnleftamountvalue: '@(angle) la tortue pivote vers la gauche du nombre de degrés indiqué en paramètre. Exemple : @(90)',
            turnrightamount: '@(angle) la tortue pivote vers la droite du nombre de degrés indiqué en paramètre. Exemple : @(90)',
            turnrightamountvalue: '@(angle) la tortue pivote vers la droite du nombre de degrés indiqué en paramètre. Exemple : @(90)',
            row: '@() capteur qui renvoie la ligne sur laquelle se trouve la tortue',
            col: '@() capteur qui renvoie la colonne sur laquelle se trouve la tortue',
            penup: '@() la tortue lève son pinceau. Dans cette position, le pinceau ne laisse pas de trace.',
            pendown: '@() la tortue place son pinceau dans la position où il laisse une trace.',
            colourvalue: '@(colorName) la trace du pinceau aura la couleur indiquée en paramètre. Exemple : @(\'red\')',
            colourRGB: '@(red, green, blue) la trace du pinceau aura la couleur avec les composantes rouge, verte et bleue indiquées en paramètres. Exemple : @(255, 255, 0)',
            colourRGBAmount: '@(red, green, blue) la trace du pinceau aura la couleur avec les composantes rouge, verte et bleue indiquées en paramètres. Exemple : @(255, 255, 0)',
            inputvalue: '@() lire un nombre en entrée.',
            jump: '@(x, y) la tortue est placée aux coordonnées données.',
            jumpAmount: '@(x, y) la tortue est placée aux coordonnées données.'
         },
         startingBlockName: "Programme de la tortue",
         exportAsSvg: "Exporter en SVG",
         exportAsPng: "Exporter en PNG",
         messages: {
            paintingWrong: "La tortue n'a pas tout dessiné correctement.",
            paintingCorrect: "Bravo! La tortue a tout dessiné correctement.",
            paintingFree: "La tortue a tracé le dessin que vous avez programmé. Si vous voulez le garder, cliquez sur le bouton \"Exporter\"."
         }
      },
      de: {
         left: "links ↺",
         right: "rechts ↻",
         penup: "hebe Stift ab",
         pendown: "setze Stift auf",
         categories: {
            turtle: "Schildkröte"
         },
         label: {
            move: "gehe",
            moveamount: "gehe %1 Schritte",
            movebackamount: "gehe %1 Schritte zurück",
            moveamountvalue: "gehe %1 Schritte",
            movebackamountvalue: "gehe %1 Schritte zurück",
            turnleft: "drehe nach links ↺",
            turnright: "drehe nach rechts ↻",
            turnleftamount: "drehe um %1° nach links ↺",
            turnrightamount: "drehe um %1° nach rechts ↻",
            turnleftamountvalue: "drehe um %1 nach links ↺",
            turnrightamountvalue: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_noround: "drehe um %1 Grad nach links ↺",
            turnrightamountvalue_noround: "drehe um %1 Grad nach rechts ↻",
            turnleftamountvalue_options: "drehe um %1 nach links ↺",
            turnrightamountvalue_options: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_moreoptions: "drehe um %1 nach links ↺",
            turnrightamountvalue_moreoptions: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_europe: "drehe um %1 nach links ↺",
            turnrightamountvalue_europe: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_nikolaus: "drehe um %1 nach links ↺",
            turnrightamountvalue_nikolaus: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_penta: "drehe um %1 nach links ↺",
            turnrightamountvalue_penta: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_pentasimple: "drehe um %1 nach links ↺",
            turnrightamountvalue_pentasimple: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_Ntimes30: "drehe um %1 nach links ↺",
            turnrightamountvalue_Ntimes30: "drehe um %1 nach rechts ↻",
            turneitheramount: "drehe um %1° nach %2",
            turneitheramountvalue: "drehe um %1 nach %2",
            row: "turtle's row", // TODO :: translate
            col: "turtle's column",
            penup: "hebe Stift ab",
            pendown: "setze Stift auf",
            peneither: "%1",
            colour2: "setze Farbe",
            colourRGB: "setze Farbe RGB ( %1, %2, %3 )",
            colourRGBAmount: "setze Farbe RGB ( %1, %2, %3 )",
            colourvalue: "setze Farbe %1",
            turn: "drehe (Grad) ",
            alert: "messagebox",
            log: "logge",
            inputvalue: "Eingabewert"
         },
         code: {
            move: "gehe",
            moveamount: "geheSchritte",
            movebackamount: "geheZurueckSchritte",
            moveamountvalue: "geheSchritte",
            movebackamountvalue: "geheZurueckSchritte",
            turnleft: "dreheLinks",
            turnright: "dreheRechts",
            turnleftamount: "dreheLinksGrad",
            turnrightamount: "dreheRechtsGrad",
            turnleftamountvalue: "dreheLinksGrad",
            turnrightamountvalue: "dreheRechtsGrad",
            turnleftamountvalue_noround: "dreheLinksGrad",
            turnrightamountvalue_noround: "dreheRechtsGrad",
            turnleftamountvalue_options: "dreheLinksGrad",
            turnrightamountvalue_options: "dreheRechtsGrad",
            turnleftamountvalue_moreoptions: "dreheLinksGrad",
            turnrightamountvalue_moreoptions: "dreheRechtsGrad",
            turnleftamountvalue_europe: "dreheLinksGrad",
            turnrightamountvalue_europe: "dreheRechtsGrad",
            turnleftamountvalue_nikolaus: "dreheLinksGrad",
            turnrightamountvalue_nikolaus: "dreheRechtsGrad",
            turnleftamountvalue_penta: "dreheLinksGrad",
            turnrightamountvalue_penta: "dreheRechtsGrad",
            turnleftamountvalue_pentasimple: "dreheLinksGrad",
            turnrightamountvalue_pentasimple: "dreheRechtsGrad",
            turnleftamountvalue_Ntimes30: "dreheLinksGrad",
            turnrightamountvalue_Ntimes30: "dreheRechtsGrad",
            turneitheramount: "dreheGrade",
            turneitheramountvalue: "dreheGrad",
            row: "turtleRow", // TODO :: translate
            col: "turtleColumn",
            penup: "stiftHoch",
            pendown: "stiftRunter",
            peneither: "stift",
            colour2: "setzeFarbe",
            colourRGB: "setzeFarbeRGB",
            colourRGBAmount: "setzeFarbeRGB",
            colourvalue: "setzeFarbe",
            turn: "drehe",
            alert: "alert",
            log: "log",
            inputvalue: "eingabewert"
         },
         description: {
         },
         startingBlockName: "Schildkröten-Programm",
         messages: {
            paintingWrong: "Die Schildkröte hat nicht alles richtig gezeichnet.",
            paintingCorrect: "Bravo! Die Schildkröte hat alles richtig gezeichnet.",
            paintingFree: "La tortue a tracé le dessin que vous avez programmé. Si vous voulez le garder, vous pouvez faire une capture d'écran."
         }
      },
      en: {
         turnleft: "right ↺",
         turnright: "left ↻",
         penup: "lift the paintbrush",
         pendown: "lower the paintbrush",
         categories: {
            turtle: "Tortue"
         },
         label: {
            move: "move forward",
            moveamount: "move forward by %1 step(s)",
            movebackamount: "move back by %1 step(s)",
            moveamountvalue: "move forward by %1 step(s)",
            movebackamountvalue: "move back by %1 step(s)",
            turnleft: "turn right ↺",
            turnright: "turn left ↻",
            turnleftamount: "turn to the left by %1° ↺",
            turnrightamount: "turn to the right by %1° ↻",
            turnleftamountvalue: "turn to the left by %1 ↺",
            turnrightamountvalue: "turn to the right by %1 ↻",
            turnleftamountvalue_noround: "drehe um %1 Grad nach links ↺",
            turnrightamountvalue_noround: "drehe um %1 Grad nach rechts ↻",
            turnleftamountvalue_options: "drehe um %1 nach links ↺",
            turnrightamountvalue_options: "drehe um %1 nach rechts ↻",
            turnleftamountvalue_moreoptions: "drehe um %1 nach links ↺",
            turnrightamountvalue_moreoptions: "drehe um %1 nach rechts ↻",
            turneitheramount: "turn by %1° to the %2",
            turneitheramountvalue: "turn by %1 to the %2",
            row: "turtle's row",
            col: "turtle's column",
            penup: "lift the paintbrush",
            pendown: "lower the paintbrush",
            peneither: "%1",
            colour2: "setze Farbe",
            colourRGB: "use color RGB ( %1, %2, %3 )",
            colourRGBAmount: "use color RGB ( %1, %2, %3 )",
            colourvalue: "use color %1",
            turn: "drehe (Grad) ",
            alert: "messagebox",
            log: "logge",
            inputvalue: "Eingabewert"
         },
         code: {
            move: "forward",
            moveamount: "forward",
            movebackamount: "backward",
            moveamountvalue: "forward",
            movebackamountvalue: "backward",
            turnleft: "turnLeft",
            turnright: "turnRight",
            turnleftamount: "left",
            turnrightamount: "right",
            turnleftamountvalue: "left",
            turnrightamountvalue: "right",
            turnleftamountvalue_noround: "dreheLinksGrad",
            turnrightamountvalue_noround: "dreheRechtsGrad",
            turnleftamountvalue_options: "dreheLinksGrad",
            turnrightamountvalue_options: "dreheRechtsGrad",
            turnleftamountvalue_moreoptions: "dreheLinksGrad",
            turnrightamountvalue_moreoptions: "dreheRechtsGrad",
            turneitheramount: "turn",
            turneitheramountvalue: "turn",
            row: "row",
            col: "col",
            penup: "liftBrush",
            pendown: "lowerBrush",
            peneither: "stift",
            colour2: "setzeFarbe",
            colourRGB: "colorRGB",
            colourRGBAmount: "colorRGB",
            colourvalue: "color",
            turn: "drehe",
            alert: "alert",
            log: "log",
            inputvalue: "inputvalue"
         },
         description: {

         },
         startingBlockName: "Program of the turtle",
         exportAsSvg: "Export to SVG",
         exportAsPng: "Export to PNG",
         messages: {
            paintingWrong: "The turtle didn't draw everything correctly.",
            paintingCorrect: "Congratulations, the turtle has drawn everything correctly.",
            paintingFree: "The turtle painted according to your program. If you want to keep it, do a screenshot."
         }
      },
      none: {
         comment: {
         }
      }
   };

   var context = quickAlgoContext(display, infos);
   var strings = context.setLocalLanguageStrings(localLanguageStrings);

   if(infos.turtleInputValueLabel) {
      strings.label.inputvalue = infos.turtleInputValueLabel;
   }
   if(infos.turtleInputValueDescription) {
      strings.description.inputvalue = infos.turtleInputValueDescription;
   }

   var cells = [];
   var texts = [];
   var scale = 1;
   var paper;

   context.turtle = {
      displayTurtle: new makeTurtle(infos.coords),
      displaySolutionTurtle: new makeTurtle(infos.coords),
      invisibleTurtle: new makeTurtle(infos.coords),
      invisibleSolutionTurtle: new makeTurtle(infos.coords),
      svgTurtle: null
   };

   if (context.infos.allowInfiniteLoop)
      context.allowInfiniteLoop = true;

   switch (infos.blocklyColourTheme) {
      case "bwinf":
         context.provideBlocklyColours = function() {
            return {
               categories: {
                  logic: 100,
                  loops: 180,
                  math: 230,
                  texts: 60,
                  lists: 40,
                  colour: 20,
                  variables: 330,
                  functions: 290,
                  turtle: 260,
                  turtleInput: 200,
                  _default: 0
               },
               blocks: {}
            };
         }
         break;
      default:
         // we could set turtle specific default colours here, if we wanted to …
   }

   context.debug_log = function(callback, message) {
      message = message ? message.toString() : '';
      if (context.display) {
         console.log("vvvvv");
      }
      console.log(message);
      if (context.display) {
         console.log("^^^^^");
      }
      context.callCallback(callback);
   };

   context.reset = function(gridInfos) {
      if(gridInfos === undefined) {
         gridInfos = context.defaultGridInfos;
      }
      else {
         context.defaultGridInfos = gridInfos;
      }

      context.turtle.canvasSize = gridInfos.options && gridInfos.options['canvas_size'] ?
        gridInfos.options['canvas_size'] :
        (infos.options && infos.options['canvas_size'] ? infos.options['canvas_size'] : constants.DEFAULT_CANVAS_SIZE);

      if (context.display && gridInfos) {
         context.resetDisplay();

         context.turtle.displayTurtle.setDrawingContext(document.getElementById('displayfield').getContext('2d'));
         context.turtle.displaySolutionTurtle.setDrawingContext(document.getElementById('solutionfield').getContext('2d'));

         context.turtle.displayTurtle.reset(context.infos.turtleStepSize, gridInfos.coords || infos.coords);
         context.turtle.displaySolutionTurtle.reset(context.infos.turtleStepSize, gridInfos.coords || infos.coords);
      }

      function createMeACanvas() {
         var canvas = document.createElement('canvas');
         canvas.width = context.turtle.canvasSize.width;
         canvas.height = context.turtle.canvasSize.height;
         canvas.style.width = context.turtle.canvasSize.width + "px";
         canvas.style.height = context.turtle.canvasSize.height + "px";
         canvas.style.border = "1px solid black";
         canvas.style.display = "none";

         //document.body.appendChild(canvas); // for debug
         return canvas.getContext('2d');
      }

      if (gridInfos) {
         context.turtle.invisibleTurtle.setDrawingContext(createMeACanvas());
         context.turtle.invisibleSolutionTurtle.setDrawingContext(createMeACanvas());

         context.turtle.invisibleTurtle.reset(context.infos.turtleStepSize, gridInfos.coords);
         context.turtle.invisibleSolutionTurtle.reset(context.infos.turtleStepSize, gridInfos.coords);

         context.drawSolution = gridInfos.drawSolution;
         context.inputValue   = gridInfos.inputValue;

         context.drawSolution(context.turtle.invisibleSolutionTurtle);
         if (context.display) {
            context.drawSolution(context.turtle.displaySolutionTurtle);
         }
      }

      if (window.C2S) {
         // Canvas2SVG library is loaded, we create the SVG at the same time
         context.turtle.svgTurtle = new makeTurtle(infos.coords);
         context.turtle.svgTurtle.setDrawingContext(new window.C2S(context.turtle.canvasSize.width, context.turtle.canvasSize.height));
         context.turtle.svgTurtle.reset(context.infos.turtleStepSize, gridInfos.coords);
      }
   };

   context.resetDisplay = function() {
      var turtleFileName = "turtle.svg";

      if ($("#turtleImg").length > 0) {
         turtleFileName = $("#turtleImg").attr("src");
      }
      var turtleUpFileName = "turtle.svg";
      if ($("#turtleUpImg").length > 0) {
         turtleUpFileName = $("#turtleUpImg").attr("src");
      }
      $("#grid").html("<div id='output'  style='height: " + (context.turtle.canvasSize.height + 4) + "px;width: " + (context.turtle.canvasSize.width + 4) + "px;border: solid 2px;margin: 12px auto;position:relative;background-color:white;'> <img id='drawinggrid' width='" + context.turtle.canvasSize.width + "' height='" + context.turtle.canvasSize.height + "' style='width:" + context.turtle.canvasSize.width + "px;height:" + context.turtle.canvasSize.height + "px;position:absolute;top:0;left:0;opacity: 0.4;filter: alpha(opacity=10);' src='" + context.infos.overlayFileName + "'><canvas id='solutionfield' width='" + context.turtle.canvasSize.width + "' height='" + context.turtle.canvasSize.height + "' style='width:" + context.turtle.canvasSize.width + "px;height:" + context.turtle.canvasSize.height + "px;position:absolute;top:0;left:0;opacity: 0.4;filter: alpha(opacity=20);'></canvas><canvas id='displayfield' width='" + context.turtle.canvasSize.width + "' height='" + context.turtle.canvasSize.height + "' style='width:" + context.turtle.canvasSize.width + "px;height:" + context.turtle.canvasSize.height + "px;position:absolute;top:0;left:0;'></canvas><canvas id='invisibledisplayfield' width='" + context.turtle.canvasSize.width + "' height='" + context.turtle.canvasSize.height + "' style='width:" + context.turtle.canvasSize.width + "px;height:" + context.turtle.canvasSize.height + "px;position:absolute;top:0;left:0;visibility:hidden;'></canvas><img id='turtle' pendown='" + turtleFileName + "' penup='" + turtleUpFileName + "' src='" + turtleFileName + "' style='width: 22px; height: 27px; position:absolute; left: 139px; top: 136px;'></img></div>")

      if (infos.buttonExportAsSvg) {
         var exportButton = $('<div><button id="exportAsSvg" style="margin-top: 10px">' + strings.exportAsSvg + '</button></div>');
         $('#grid').append(exportButton);
         $('#exportAsSvg').click(function(e) {
           context.exportAsSvg();
         });
      }
      if (infos.buttonExportAsPng) {
         var exportButton = $('<div><button id="exportAsPng" style="margin-top: 10px">' + strings.exportAsPng + '</button></div>');
         $('#grid').append(exportButton);
         $('#exportAsPng').click(function(e) {
            context.exportAsPng();
         });
      }

      context.blocklyHelper.updateSize();
      context.turtle.displayTurtle.setTurtle(document.getElementById('turtle'));
      context.turtle.displayTurtle.fixTurtle(); // TODO :: find a way to define whether the turtle needs fixing or not
      context.turtle.displayTurtle.reset();

      context.updateScale(); // does nothing for now
   };

   context.unload = function() {
      if (context.display) {
         // ... clean up necessary?
      }
   };

   context.updateScale = function() {
   };

   context.exportAsSvg = function () {
     var svg = context.exportGridAsSvg();
     if (!svg) {
       return;
     }

     var svgData = svg.outerHTML;
     var svgBlob = new Blob([svgData], {type:"image/svg+xml;charset=utf-8"});
     var svgUrl = URL.createObjectURL(svgBlob);
     var downloadLink = document.createElement("a");
     downloadLink.href = svgUrl;
     downloadLink.download = "turtle.svg";
     document.body.appendChild(downloadLink);
     downloadLink.click();
     document.body.removeChild(downloadLink);
   };

   context.exportAsPng = function () {
      var downloadLink = document.createElement('a');
      var canvas = context.turtle.displayTurtle.drawingContext.canvas;
      var dataURL = canvas.toDataURL('image/png');
      var pngUrl = dataURL.replace(/^data:image\/png/,'data:application/octet-stream');
      downloadLink.href = pngUrl;
      downloadLink.download = "turtle.png";
      document.body.appendChild(downloadLink);
      downloadLink.click();
      document.body.removeChild(downloadLink);
   };

   context.exportGridAsSvg = function (option) {
      if (!window.C2S) {
         console.error("Unable to export as SVG, canvas2svg library is not loaded.");
         return;
      }

      var svgTurtle = context.turtle.svgTurtle;
      var svg = svgTurtle.drawingContext.getSvg();
      // Remove leftover images from previous export
      var svgImages = svg.getElementsByTagName('image');
      for (var i = 0; i < svgImages.length; i++) {
         svgImages[i].remove();
      }

      if (option == 'full') {
         // Make an export with the background and the turtle
         var bgimg = document.createElementNS('http://www.w3.org/2000/svg', 'image');
         bgimg.setAttribute('x', 0);
         bgimg.setAttribute('y', 0);
         bgimg.setAttribute('width', context.turtle.canvasSize.width);
         bgimg.setAttribute('height', context.turtle.canvasSize.height);
         bgimg.setAttribute('style', 'opacity: 0.4; filter: alpha(opacity=10);');
         bgimg.setAttribute('xlink:href', context.infos.overlayFileName);
         svg.prepend(bgimg);

         var turtleFileName = "turtle.svg";
         if ($("#turtleImg").length > 0) {
            turtleFileName = $("#turtleImg").attr("src");
         }

         var turtleimg = document.createElementNS('http://www.w3.org/2000/svg', 'image');
         turtleimg.setAttribute('x', svgTurtle.x - 12);
         turtleimg.setAttribute('y', svgTurtle.y - 15);
         turtleimg.setAttribute('width', 22);
         turtleimg.setAttribute('height', 27);
         turtleimg.setAttribute('style', 'padding-right: 2px; padding-bottom: 3px;');
         turtleimg.setAttribute('xlink:href', turtleFileName);
         if (svgTurtle.direction) {
            turtleimg.setAttribute('transform', "rotate(" + (-svgTurtle.directionDeg) + ", " + svgTurtle.x + ", " + svgTurtle.y + ")");
         }
         svg.append(turtleimg);
      }

      return svg;
   }


   function callOnAllTurtles(fn) {
      fn(context.turtle.invisibleTurtle);
      if (context.display) {
         fn(context.turtle.displayTurtle);
      }
      if (context.turtle.svgTurtle) {
         fn(context.turtle.svgTurtle);
      }
   }

   context.turtle.moveamount = function(param, callback) {
      if (typeof callback == "undefined") {
         callback = param;
         param = 0;
      }

      callOnAllTurtles(function(turtle) {
         turtle.move(param);
      })

      context.waitDelay(callback);
   }

   context.turtle.movebackamount = function(param, callback) {
      if (typeof callback == "undefined") {
         callback = param;
         param = 0;
      }

      callOnAllTurtles(function(turtle) {
         turtle.move(-param);
      });

      context.waitDelay(callback);
   }

   // DEPRECATED
   context.turtle.turn = function(param, callback) {
      callOnAllTurtles(function(turtle) {
         turtle.turn(param);
      })

      context.waitDelay(callback);
   }

   context.turtle.turneitheramount = function(degree, direction, callback) {
      if (typeof callback == "undefined") {
         callback = direction;
         direction = "l";
         if (typeof callback == "undefined") {
            callback = degree;
            degree = 0;
         }
      }

      callOnAllTurtles(function(turtle) {
         if (direction.search('l') != -1) {
            turtle.turn(degree);
         }
         else {
            turtle.turn(-degree);
         }
      });

      context.waitDelay(callback);
   }

   context.turtle.jump = function(x, y, callback) {
      callOnAllTurtles(function(turtle) {
		  turtle.jump(x, y);
      });
      context.waitDelay(callback);
   }

   context.turtle.peneither = function(status, callback) {
      callOnAllTurtles(function(turtle) {
         if (status == "up") {
            turtle.stop_painting();
         }
         else {
            turtle.start_painting();
         }
      })

      context.waitDelay(callback);
   }

   context.turtle.row = function(callback) {
      context.runner.noDelay(callback, context.turtle.invisibleTurtle.getCoords().y);
   }
   context.turtle.col = function(callback) {
      context.runner.noDelay(callback, context.turtle.invisibleTurtle.getCoords().x);
   }
   context.turtle.move = function(callback) {
      context.turtle.moveamount(1, callback);
   }
   context.turtle.turnleftamount = function(param, callback) {
      context.turtle.turneitheramount(param, "l", callback);
   }
   context.turtle.turnrightamount = function(param, callback) {
      context.turtle.turneitheramount(param, "r", callback);
   }
   context.turtle.turnleft = function(callback) {
      context.turtle.turnleftamount(90, callback);
   }
   context.turtle.turnright = function(callback) {
      context.turtle.turnrightamount(90, callback);
   }
   context.turtle.penup = function(callback) {
      context.turtle.peneither("up", callback);
   }
   context.turtle.pendown = function(callback) {
      context.turtle.peneither("down", callback);
   }
   context.turtle.inputvalue = function(callback) {
      context.callCallback(callback, context.inputValue);
   };

   context.turtle.moveamountvalue = context.turtle.moveamount;
   context.turtle.movebackamountvalue = context.turtle.movebackamount;
   context.turtle.turnleftamountvalue = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue = context.turtle.turnrightamount;
   context.turtle.turneitheramountvalue = context.turtle.turneitheramount;
   context.turtle.turnleftamountvalue_noround = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_noround = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_options = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_options = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_moreoptions = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_moreoptions = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_europe = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_europe = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_nikolaus = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_nikolaus = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_penta = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_penta = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_pentasimple = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_pentasimple = context.turtle.turnrightamount;
   context.turtle.turnleftamountvalue_Ntimes30 = context.turtle.turnleftamount;
   context.turtle.turnrightamountvalue_Ntimes30 = context.turtle.turnrightamount;
   context.turtle.jumpAmount = context.turtle.jump;

   context.turtle.colour2 = function(colour, callback) {
      if (typeof callback == "undefined") {
         callback = colour;
         colour = "#000000";
      }

      callOnAllTurtles(function(turtle) {
         turtle.set_colour(colour);
      })

      context.waitDelay(callback);
   }
   context.turtle.colourvalue = context.turtle.colour2;

   context.turtle.colourRGB = function(r, g, b, callback) {
      colour = "rgb("+r+", "+g+", "+b+")";
      if (typeof callback == "undefined") {
         callback = colour;
         colour = "#000000";
      }

      callOnAllTurtles(function(turtle) {
         turtle.set_colour(colour);
      })

      context.waitDelay(callback);
   }

   context.turtle.colourRGBAmount = context.turtle.colourRGB;

   var defaultMoveAmount = 1;
   if(context.infos.defaultMoveAmount != undefined)
      defaultMoveAmount = context.infos.defaultMoveAmount;

   context.customBlocks = {
      turtle: {
         turtle: [
            { name: "move" },
            { name: "moveamount", params: [null]},
            { name: "movebackamount", params: [null]},
            { name: "jump", params: [null, null], blocklyJson: {"args0": [{"type": "field_number", "name": "PARAM_0", "value": 0},{"type": "field_number", "name": "PARAM_1", "value": 0}]}},
            { name: "moveamountvalue", params: [null], blocklyJson: {"args0": [{"type": "field_number", "name": "PARAM_0", "value": defaultMoveAmount}]}},
            { name: "movebackamountvalue", params: [null], blocklyJson: {"args0": [{"type": "field_number", "name": "PARAM_0", "value": defaultMoveAmount}]}},
            { name: "turnleft" },
            { name: "turnright" },
            { name: "turn",      params: [null]},
            { name: "turnleftamount", params: [null]},
            { name: "turnrightamount", params: [null]},
            { name: "turnleftamountvalue", params: [null], blocklyJson: {"args0": [{"type": "field_angle", "name": "PARAM_0", "angle": 90}]}},
            { name: "turnrightamountvalue", params: [null], blocklyJson: {"args0": [{"type": "field_angle", "name": "PARAM_0", "angle": 90}]}},
            { name: "turnleftamountvalue_noround", blocklyJson: {"args0": [{"type": "field_number", "name": "PARAM_0", "value": 90}]}},
            { name: "turnrightamountvalue_noround", blocklyJson: {"args0": [{"type": "field_number", "name": "PARAM_0", "value": 90}]}},
            { name: "turnleftamountvalue_options", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
                ["36 °","36"],["45 °","45"],["60 °","60"],["72 °","72"],["90 °","90"],["108 °","108"],["120 °","120"],["135 °","135"],["144 °","144"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_options", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
               ["36 °","36"],["45 °","45"],["60 °","60"],["72 °","72"],["90 °","90"],["108 °","108"],["120 °","120"],["135 °","135"],["144 °","144"],["180 °","180"]]}]}},
            { name: "turnleftamountvalue_moreoptions", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
               ["15 °","15"],["18 °","18"],["30 °","30"],["36 °","36"],["45 °","45"],["60 °","60"],["72 °","72"],["90 °","90"],["108 °","108"],["120 °","120"],["135 °","135"],["144 °","144"],["150 °","150"],["162 °","162"],["165 °","165"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_moreoptions", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
               ["15 °","15"],["18 °","18"],["30 °","30"],["36 °","36"],["45 °","45"],["60 °","60"],["72 °","72"],["90 °","90"],["108 °","108"],["120 °","120"],["135 °","135"],["144 °","144"],["150 °","150"],["162 °","162"],["165 °","165"],["180 °","180"]]}]}},
            { name: "turnleftamountvalue_europe", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["15 °","15"],["30 °","30"],["75 °","75"],["90 °","90"],["105 °","105"],["144 °","144"],["162 °","162"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_europe", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["15 °","15"],["30 °","30"],["75 °","75"],["90 °","90"],["105 °","105"],["144 °","144"],["162 °","162"],["180 °","180"]]}]}},
            { name: "turnleftamountvalue_nikolaus", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["36.9 °","36.86989"],["53.1 °","53.13010"],["73.7 °","73.73979"],["90 °","90"],["106.3 °","106.26020"],["126.9 °","126.86989"],["143.1 °","143.13010"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_nikolaus", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["36.9 °","36.86989"],["53.1 °","53.13010"],["73.7 °","73.73979"],["90 °","90"],["106.3 °","106.26020"],["126.9 °","126.86989"],["143.1 °","143.13010"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_penta", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["18 °","18"],["36 °","36"],["54 °","54"],["72 °","72"],["90 °","90"],["108 °","108"],["126 °","126"],["144 °","144"],["162 °","162"],["180 °","180"]]}]}},
            { name: "turnleftamountvalue_penta", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["18 °","18"],["36 °","36"],["54 °","54"],["72 °","72"],["90 °","90"],["108 °","108"],["126 °","126"],["144 °","144"],["162 °","162"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_pentasimple", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["18 °","18"],["72 °","72"],["90 °","90"],["108 °","108"],["144 °","144"],["162 °","162"],["180 °","180"]]}]}},
            { name: "turnleftamountvalue_pentasimple", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["18 °","18"],["72 °","72"],["90 °","90"],["108 °","108"],["144 °","144"],["162 °","162"],["180 °","180"]]}]}},
            { name: "turnleftamountvalue_Ntimes30", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["30 °","30"],["60 °","60"],["90 °","90"],["120 °","120"],["180 °","180"]]}]}},
            { name: "turnrightamountvalue_Ntimes30", blocklyJson: {"args0": [{"type": "field_dropdown", "name": "PARAM_0", "options": [
              ["30 °","30"],["60 °","60"],["90 °","90"],["120 °","120"],["180 °","180"]]}]}},
            { name: "turneitheramount", blocklyJson: {"args0": [
               {"type": "input_value", "name": "PARAM_0"},
               {"type": "field_dropdown", "name": "PARAM_1", "options":
                 [[localLanguageStrings[window.stringsLanguage]["left"],"l"],[localLanguageStrings[window.stringsLanguage]["right"],"r"]]}]}},
            { name: "turneitheramountvalue", blocklyJson: {"args0": [
               {"type": "field_angle", "name": "PARAM_0", "angle": 90},
               {"type": "field_dropdown", "name": "PARAM_1", "options":
                 [[localLanguageStrings[window.stringsLanguage]["left"],"l"],[localLanguageStrings[window.stringsLanguage]["right"],"r"]]}]}},
            { name: "row", yieldsValue: true },
            { name: "col", yieldsValue: true },
            { name: "penup" },
            { name: "pendown" },
            { name: "peneither", blocklyJson: {"args0": [
               {"type": "field_dropdown", "name": "PARAM_0", "options":
                 [[localLanguageStrings[window.stringsLanguage]["penup"],"up"],[localLanguageStrings[window.stringsLanguage]["pendown"],"down"]]}]}},
            { name: "colour2", params: [null]},
            { name: "colourvalue", params: [null], blocklyJson: {"args0": [{"type": "field_colour", "name": "PARAM_0", "colour": "#ff0000"}]}},
            { name: "colourRGB", params: [null, null, null], blocklyJson: {"args0": [{"type": "field_number", "name": "PARAM_0", "value": 0},{"type": "field_number", "name": "PARAM_1", "value": 0},{"type": "field_number", "name": "PARAM_2", "value": 0}]}},
            {
               name: "colourRGBAmount",
               params: [null, null, null],
               blocklyJson: {
                  "args0": [
                    {"type": "input_value", "name": "PARAM_0", "value": 0},
                    {"type": "input_value", "name": "PARAM_1", "value": 0},
                    {"type": "input_value", "name": "PARAM_2", "value": 0}
                 ]
               },
               blocklyXml: "<block type='colourRGBAmount'>" +
                 "<value name='PARAM_0'><shadow type='math_number'></shadow></value>" +
                 "<value name='PARAM_1'><shadow type='math_number'></shadow></value>" +
                 "<value name='PARAM_2'><shadow type='math_number'></shadow></value>" +
                 "</block>"
            },
            { 
               name: "jumpAmount", 
               params: [null, null], 
               blocklyJson: {
                  "args0": [
                     {"type": "input_value", "name": "PARAM_0", "value": 0},
                     {"type": "input_value", "name": "PARAM_1", "value": 0}
                  ]
               },
               blocklyXml: "<block type='jumpAmount'>" +
                 "<value name='PARAM_0'><shadow type='math_number'></shadow></value>" +
                 "<value name='PARAM_1'><shadow type='math_number'></shadow></value>" +
                 "</block>"
            },
         ],
         turtleInput: [
            { name: "inputvalue", yieldsValue: true }
          ]
      },
      debug: {
         debug: [
            { name: "alert", params: [null], handler: context.debug_alert,
              blocklyXml: "<block type='alert'><value name='PARAM_0'><block type='text'><field name='TEXT'></field></block></value></block>"},
            { name: "log",   params: [null], handler: context.debug_log,
              blocklyXml: "<block type='log'><value name='PARAM_0'><block type='text'><field name='TEXT'></field></block></value></block>"}
         ]
      }
   };

   return context;
}

if(window.quickAlgoLibraries) {
   quickAlgoLibraries.register('turtle', getContext);
} else {
   if(!window.quickAlgoLibrariesList) { window.quickAlgoLibrariesList = []; }
   window.quickAlgoLibrariesList.push(['turtle', getContext]);
}
