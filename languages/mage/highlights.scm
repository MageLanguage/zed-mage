(identifier) @variable

(single_quoted
  (raw) @string
)

(double_quoted
  (raw) @string
)

(single_quoted
  (escape) @string.escape
)

(double_quoted
  (escape) @string.escape
)

[
  (binary)
  (octal)
  (decimal)
  (hex)
] @number

[
  (constant)
  (variable)
  (pipe)
  (and)
  (or)
  (equal)
  (not_equal)
  (less_than)
  (greater_than)
  (less_equal)
  (greater_equal)
  (add)
  (subtract)
  (multiply)
  (divide)
  (modulo)
  (extract)
] @operator

[
  "("
  ")"
  "{"
  "}"
  "\""
] @punctuation.bracket

(call
  (_)
  (pipe)
  (identifier) @function
)

(call
  (_)
  (pipe)
  (member
    (_)
    (extract)
    (identifier) @function)
)
