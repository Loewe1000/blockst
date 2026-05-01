#let scratch-block-options = state("scratch-block-options", (
  theme: "normal",
  stroke-width: auto,
  scale: 100%,
  font: "Helvetica Neue",
  line-numbers: false,
  line-number-start: 1,
  line-number-gutter: 24,
  inset-scale: 1.0,
))

#let get-options() = scratch-block-options.get()

#let get-theme(options) = options.at("theme", default: "normal")

#let get-scale(options) = options.at("scale", default: 100%)

#let get-font(options) = options.at("font", default: "Helvetica Neue")

#let get-line-numbers(options) = options.at("line-numbers", default: false)

#let get-line-number-start(options) = options.at("line-number-start", default: 1)

#let get-line-number-gutter(options) = options.at("line-number-gutter", default: 24)

#let get-inset-scale(options) = options.at("inset-scale", default: 1.0)
