#import "@preview/touying:0.7.3": *
#import "gruvbox-dark-theme.typ": *
#import "@preview/cetz:0.5.0"
#import "@preview/fletcher:0.5.8" as fletcher: edge, node
#import "@preview/numbly:0.1.0": numbly
#import "@preview/theorion:0.6.0": *
#import cosmos.clouds: *
#show: show-theorion

#let cetz-canvas = touying-reducer.with(reduce: cetz.canvas, cover: cetz.draw.hide.with(bounds: true))

#set text(font: "CommitMono-levy", lang: "hun")

#show: gruvbox-dark-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: [Combinator Explorer],
    author: [Orbán Levente],
    date: datetime.today(),
    institution: [Konzulens: Németh Gábor],
  ),
)

#title-slide()

#let fg = rgb("#d4be98")
#let bg = rgb("#1d2021")
#let blue = rgb("#7daea3")
#let orange = rgb("#e78a4e")
#let green = rgb("#a9b665")

#let milestones = (
  (year: 1958, name: "LISP", desc: "Első osztályú függvények", dir: 1, color: blue),
  (year: 1973, name: "ML", desc: "Hindley-Milner típusok\nés mintaillesztés", dir: -1, color: blue),
  (year: 1983, name: "C++", desc: "Objektumorientált\nparadigma térnyerése", dir: 1, color: orange),
  (year: 1995, name: "Java", desc: "Nagyvállalati OOP\nés virtuális gépek", dir: -1, color: orange),
  (year: 2011, name: "C++11", desc: "Lambdák és constexpr\nrendszerszinten", dir: 1, color: green),
  (year: 2020, name: "\"Modern\" OOP", desc: "Rekordok, mintaillesztés\n(Java, C#, Kotlin)", dir: -1, color: green),
  (year: 2025, name: "Rust", desc: "Multiparadigmális\nnyelv a Linux kernelben", dir: 1, color: green),
)

#slide(
  repeat: milestones.len(),
  title: [Paradigmaváltozás],
  self => {
    let step = self.subslide

    align(center + horizon)[
      #cetz-canvas({
        import cetz.draw: *
        let min_year = 1955.0
        let max_year = 2030.0
        let x_start = -2
        let x_end = 16.5

        let transparent = rgb(0, 0, 0, 0)
        line((-6, -8), (22, 8), stroke: (paint: transparent, thickness: 0.1pt))
        line((-6, 8), (22, -8), stroke: (paint: transparent, thickness: 0.1pt))

        // Main timeline arrow
        line((-3, 0), (19, 0), stroke: (paint: fg, thickness: 3pt), mark: (end: ">", fill: fg, scale: 1.5))

        for (i, m) in milestones.enumerate() {
          if i + 1 > step { continue }

          let x = x_start + ((m.year - min_year) / (max_year - min_year)) * (x_end - x_start)

          let is_active = (i + 1 == step)

          line((x, -0.3), (x, 0.3), stroke: (paint: fg, thickness: 2pt))

          if is_active {
            let end_y = m.dir * 4.5
            let year_y = m.dir * -0.7

            line((x, 0), (x, end_y), stroke: (paint: m.color, thickness: 1.5pt, dash: "dashed"))
            content((x, year_y), text(fill: fg, weight: "bold", size: 1.2em)[#m.year])

            content(
              (x, end_y),
              box(
                fill: m.color.darken(60%),
                stroke: (paint: m.color, thickness: 2pt),
                radius: 0.3em,
                inset: 0.6em,
                align(center)[
                  #text(fill: fg, weight: "bold", size: 1.1em)[#m.name] \
                  #v(0.1em)
                  #text(fill: fg, size: 0.75em)[#m.desc]
                ],
              ),
            )
          } else {
            let end_y = m.dir * 1.5
            let year_y = m.dir * -0.6

            line((x, 0), (x, end_y), stroke: (paint: m.color.darken(30%), thickness: 1pt))
            content((x, year_y), text(fill: fg.darken(30%), size: 0.8em)[#m.year])

            content(
              (x, end_y),
              box(
                fill: bg,
                stroke: (paint: m.color.darken(30%), thickness: 1pt),
                radius: 0.2em,
                inset: 0.3em,
                text(fill: fg.darken(20%), size: 0.7em)[#m.name],
              ),
            )
          }
        }
      })
    ]
  },
)

== Paradigmaváltozás

- az újabb nyelvek már multiparadigmálisak (Rust, Kotlin)
#pause
- a régebbi nyelvek is efelé mozognak


#speaker-note(
  "
- 90es évek: full OO
- elmúlt 10-20 év: egyre több funkcionális elem => multi-paradigm
  - C++11 constexpr
  - Java/C\# records
  - lambdas everywhere!
  - [insert graph plot here]
- we still mainly teach OO concepts, even though writing \"modern\" code requires using these new, shiny features
  - how do they work?
",
)

== Feladat

#set quote(block: true)

#quote(attribution: [Témakiírás])[
  [...] A hallgató feladata egy
  - *webes eszköz* (Combinator Explorer) elkészítése, amely
  - SKI-kifejezések esetén *megmutatja a redukciós lépéseket, vizualizálja a redexeket, és felrajzolja a kiértékelési fát*.
  - A redukció során *többféle kiértékelési stratégiát támogat*, valamint az *SKI-kifejezéseket lefordítja λ-kalkulusba*. [...]
]

== Kezdeti állapot

==

== Test

We can use `#pause` to #pause display something later.

#pause

Just like this.

#meanwhile

Meanwhile, #pause we can also use `#meanwhile` to #pause display other content synchronously.

== Complex Animation

At subslide #touying-fn-wrapper((self: none) => str(self.subslide)), we can

use #uncover("2-")[`#uncover` function] for reserving space,

use #only("2-")[`#only` function] for not reserving space,

#alternatives[call `#only` multiple times \u{2717}][use `#alternatives` function #sym.checkmark] for choosing one of the alternatives.


== Callback Style Animation

#slide(
  repeat: 3,
  self => [
    #let (uncover, only, alternatives) = utils.methods(self)

    At subslide #self.subslide, we can

    use #uncover("2-")[`#uncover` function] for reserving space,

    use #only("2-")[`#only` function] for not reserving space,

    #alternatives[call `#only` multiple times \u{2717}][use `#alternatives` function #sym.checkmark] for choosing one of the alternatives.
  ],
)


== Math Equation Animation

Equation with `pause`:

$
  f(x) & = pause x^2 + 2x + 1 \
       & = pause (x + 1)^2 \
$

#meanwhile

Here, #pause we have the expression of $f(x)$.

#pause

By factorizing, we can obtain this result.

= Theorems

= Others

== Multiple columns

#cols[
  First column.
][
  Second column.
]

== Multiple columns with equal height blocks

#cols(columns: (1fr, 1fr), gutter: 1em)[
  #emph-block[
    First column with equal height: #lorem(10)
    #lazy-v(1fr)
  ]
][
  #emph-block[
    Second column with equal height: : #lorem(15)
    #lazy-v(1fr)
  ]
]


== Multiple Pages

#show: appendix

= Appendix

== Appendix

Please pay attention to the current slide number.

