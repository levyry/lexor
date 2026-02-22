# Lexor

## TODO

- lexor-reducer
  - add a gc
  - implement and use `View` struct instead of stringifying everything
  - better error handling (more results with own error types)
  - overhaul the parser
    - allow "variables" that don't reduce
    - later on allow numbers and strict operators like addition

- lexor-convert
  - finish conversion

- lexor-core
  - refactor to be only about lambda calc
  - revisit reduction implementation

- meta
  - write a pretty todo for main project
  - write pretty todos for individual crates
  - implement UI crate
  - add itertools crate if considerable optimizations can be made

Need to think about best way to hook up the two backends to UI. The UI would probably need to provide a `Backend` trait for possible backends to implement.