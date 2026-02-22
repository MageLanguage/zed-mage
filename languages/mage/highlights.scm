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
  (hexadecimal)
  (integer)
] @number

[
  (constant)
  (variable)
  (equal_sign)
  (not_equal_sign)
  (less_than_sign)
  (greater_than_sign)
  (less_than_or_equal_sign)
  (greater_than_or_equal_sign)
  (addition)
  (subtraction)
  (multiplication)
  (division)
  (modulus)
  (extract)
] @operator

[
  "("
  ")"
  "{"
  "}"
  "\""
  ","
] @punctuation.bracket

(constant_assignment
  name: (identifier) @keyword
)

(constant_assignment
  name: (member
    property: (identifier) @keyword
  )
)

(variable_assignment
  name: (identifier) @property
)

(variable_assignment
  name: (member
    property: (identifier) @property
  )
)

(multiple_variable_assignment
  names: (name_list
    (identifier) @property
  )
)

(call
  name: (identifier) @function
)
