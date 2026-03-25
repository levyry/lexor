# Lexor

## TODO

### Frontend

- probably a single crate like "lexor-wasm" or "combinator-explorer"
- [egui](https://github.com/emilk/egui) with [egui dock](https://github.com/anhosh/egui_dock) to mimic [GoldenLayout](https://golden-layout.com/)
- for drawing the AST, use [egui_graphs](https://github.com/blitzarx1/egui_graphs?tab=readme-ov-file). look into maybe implementing a Reingold-Tilford layout algorithm for a "neater" AST look
- if i want progressive streaming on longer reductions, [egui_infinite_scroll](https://docs.rs/crate/egui_infinite_scroll/0.9.0/source/examples/infinite_scroll_async.rs) seems very useful for that
  - probably send over a `reporting_frequency` in the request, and count the reductions in the closure. after `reporting_frequency` amount of reductions, return the current batch _somehow_...
- look and feel similar to [regex101](https://regex101.com/) and [compiler explorer](https://godbolt.org/)
- WASM target
- generate recursive diagrams showing applied combinator and it's operands at each reduction step
- (look into generating some lambda visualizations)
- think about best way to hook up the two backends to UI. the UI would probably need to provide a `Backend` trait for possible backends to implement

### Backend

#### lexor-core

- base crate for common utilities and definitions
- have to look at other projects to see what the community deems acceptable to put into a "core" crate vs. putting it into it's own crate
- revisit lambda reduction implementation
- use arena allocators for lambda as well

#### parsing

- either write own recursive decent parser or benchmark nom and chumsky
- search for reference implementations, read first couple of chapters of "Engineering a Compiler" for technical details
- (potentially look into just parsing into a flat AST in the first place, and not parsing into a regular AST which then gets flattened)
- allow "variables" that don't reduce
- later on allow numbers and strict operators like addition

#### graph-reducer

- add a gc
- convert to STG later
- name ideas:
  - `stg`: Spineless Tagless G-machine
  - `ventral`: opposite of "dorsal", refernce to lack of spine (spineless creatures usually have ventral nerve cords instead of a spine)
  - `coleoid`: members of the cephalopoda class, commonly thought of as "soft-bodied" (i.e. spineless) like octopi, squids and cuttlefish (potential logo idea)
  - `lexor-reduce`: boooring
  - `lexor-stg`: i mean i guess...

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
- write pretty todos for individual crates
- finish documentation everywhere (uncomment lint in Cargo.toml)
- add [itertools](https://crates.io/crates/itertools) if considerable optimizations can be made
- add [rayon](https://crates.io/crates/rayon) if parallelization opportunities are found
- look into if [proptest](https://crates.io/crates/proptest) is applicable
- think more about benchmarking
  - look for examples from other projects to see what is popular
  - could use [hyperfine](https://github.com/sharkdp/hyperfine) if i create a CLI
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
