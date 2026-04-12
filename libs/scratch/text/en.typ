// text/en.typ — English language wrapper for shared parser engine

#import "parser.typ": parse-scratch-text as parse-generic, render-scratch-text as render-generic, default-statement-defs-raw, default-expression-defs-raw
#import "profiles.typ": get-language-profile

#let _LANG_CODE = "en"
#let _PROFILE = get-language-profile(_LANG_CODE)
#let _END_MARKER = _PROFILE.at("end-marker")
#let _ELSE_MARKER = _PROFILE.at("else-marker")
#let _COMMENT_PREFIX = _PROFILE.at("line-comment-prefix")

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
