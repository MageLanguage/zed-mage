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

(assign
  (identifier) @keyword
  (constant)
)

(assign
  (member
    (extract)
    (identifier) @keyword
  )
  (constant)
)

(assign
  (identifier) @property
  (variable)
)

(assign
  (member
    (extract)
    (identifier) @property
  )
  (variable)
)

(call
  (pipe)
  (identifier) @function
)

(call
  (pipe)
  (member
    (extract)
    (identifier) @function
  )
)
