// text/de.typ — German language wrapper for shared parser engine

#import "parser.typ": parse-scratch-text as parse-generic, render-scratch-text as render-generic, default-statement-defs-raw, default-expression-defs-raw

#let _LANG_CODE = "de"
#let _END_MARKER = "ende"
#let _ELSE_MARKER = "sonst"
#let _COMMENT_PREFIX = "//"

// Exposed for language-specific overrides if needed.
#let statement-defs-raw = default-statement-defs-raw
#let expression-defs-raw = default-expression-defs-raw

#let parse-scratch-text(text) = parse-generic(
  text,
  lang-code: _LANG_CODE,
  end-marker: _END_MARKER,
  else-marker: _ELSE_MARKER,
  line-comment-prefix: _COMMENT_PREFIX,
  statement-defs-raw: statement-defs-raw,
  expression-defs-raw: expression-defs-raw,
)

#let render-scratch-text(text) = render-generic(
  text,
  lang-code: _LANG_CODE,
  end-marker: _END_MARKER,
  else-marker: _ELSE_MARKER,
  line-comment-prefix: _COMMENT_PREFIX,
  statement-defs-raw: statement-defs-raw,
  expression-defs-raw: expression-defs-raw,
)
