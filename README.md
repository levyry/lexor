# Lexor

## TODO

### Docs

- talk about de bruijn in docs

### Frontend

- toggle reduction chain coloring
- themes

### Backend

#### kiselyov

- deepen understanding of algorithm
- add eta-optimization to kiselyov
- make kiselyov non recursive to avoid stack overflows
- name ideas:
  - `kiselyov`: clean
  - `brabs`: portmanteau of "bracket abstraction", which is what this algorithm does
  - `lexor-convert`: boooring!!

### Meta

- write a pretty todo for main project
- finish documentation everywhere (uncomment lint in Cargo.toml)
- add [itertools](https://crates.io/crates/itertools) if considerable optimizations can be made
- add [rayon](https://crates.io/crates/rayon) if parallelization opportunities are found
- look into if [proptest](https://crates.io/crates/proptest) is applicable
- think more about benchmarking
  - look for examples from other projects to see what is popular
- think about how to break apart/publish project
  - if i end up creating 2-3 distinct crates, putting them in their own repos would be best (especially if they are reusable like stg and kiselyov) but that just makes it harder to work on the _whole_ project at once, so it might only make sense to do this after the thesis is done
- try out and benchmark cranelift backend
- read through [The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html) and implement things like:
  - `-C target-cpu=native` (does this work for WASM?)
  - PGO ([`cargo-pgo`](https://github.com/Kobzol/cargo-pgo))
  - [`cargo-wizard`](https://github.com/Kobzol/cargo-wizard)
  - alternative allocator (like [`mimalloc`](https://github.com/microsoft/mimalloc))
- search whole project for "TODO" strings and clean them up

## Resources

- [$\\lambda$ to SKI, Semantically (Oleg Kiselyov)](https://okmij.org/ftp/tagless-final/ski.pdf)
- [Optimizing bracket abstraction for Combinator Reduction](https://thma.github.io/posts/2023-10-08-Optimizing-bracket-abstraction-for-combinator-reduction.html)
- [Implementing a Functional Language with Graph Reduction](https://thma.github.io/posts/2021-12-27-Implementing-a-functional-language-with-Graph-Reduction.html)
- [Lambda the Penultimate](https://benlynn.blogspot.com/2018/11/lambda-penultimate_16.html)
- [STG paper (Peyton, 1992)](https://www.microsoft.com/en-us/research/wp-content/uploads/1992/04/spineless-tagless-gmachine.pdf)
