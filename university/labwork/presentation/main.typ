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
    title: [*Combinator Explorer*],
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
  (year: 1958, name: "LISP", desc: "első osztályú függvények", dir: 1, color: blue),
  (year: 1973, name: "ML", desc: "Hindley-Milner típusok\nés mintaillesztés", dir: -1, color: blue),
  (year: 1983, name: "C++", desc: "objektumorientált\nparadigma elterjedése", dir: 1, color: orange),
  (year: 1996, name: "Java", desc: "OOP-ra alapozott\n(3 billion devices)", dir: -1, color: orange),
  (year: 2011, name: "C++11", desc: "lambdák és constexpr", dir: 1, color: green),
  (year: 2020, name: "\"modern\" OOP", desc: "rekordok, mintaillesztés\n(Java, C#, Kotlin)", dir: -1, color: green),
  (year: 2025, name: "Rust", desc: "multiparadigmális\nnyelv a Linux kernelben", dir: 1, color: green),
)

#slide(
  repeat: milestones.len(),
  title: [paradigmaváltozás],
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

#slide(title: [== paradigmaváltozás])[
  - az újabb nyelvek már *multiparadigmálisak* (Rust, Kotlin)
  #pause
  - a régebbi nyelvek is efelé mozognak
  #pause
  - az eszközök (lambdák, mintaillesztés) mindennaposak lettek
  #pause
  - a mögöttes *matematikai alapok* idegenek a mérnöki közönség számára
]

== feladat

#set quote(block: true)

#quote(attribution: [Témakiírás])[
  [...] A hallgató feladata egy
  - *webes eszköz* (_Combinator Explorer_) elkészítése, amely
  - SKI-kifejezések esetén megmutatja a *redukciós lépéseket*, vizualizálja a redexeket, és felrajzolja a *kiértékelési fát*.
  - A redukció során többféle *kiértékelési stratégiát* támogat, valamint az SKI-kifejezéseket *lefordítja* λ-kalkulusba. [...]
]

== kezdeti állapot

#pause
- Funkcionális programozás C++-ban szabvál $arrow$ *$lambda$-kalkulus*
#pause
- Stephen *Wolfram* írásai $arrow$ *SKI-kalkulus*

== $lambda$-kalkulus és SKI-kalkulus

#slide(
  title: [$lambda$-kalkulus és SKI-kalkulus],
  repeat: 9,
  self => [
    #let (uncover, only, alternatives) = utils.methods(self)

    #grid(
      columns: (1fr, 1fr),
      gutter: 2em,
      [
        #uncover("1-")[*$lambda$-kalkulus:*]
        #uncover("2-")[- általános célú, Turing-teljes nyelv]
        #uncover("3-")[- változók, absztrakciók, applikációk]
        #uncover("4-")[
          #align(center + bottom)[
            #rect(width: 100%, height: 120pt, fill: rgb("#3c3836"))[
              #align(center + horizon)[
                $
                  lambda x. lambda y.(x+y)
                $
              ]
            ]
          ]
        ]
      ],
      [
        #uncover("5-")[*SKI-kalkulus:*]
        #uncover("6-")[- Turing-teljes]
        #uncover("7-")[- három kombinátor ($S$, $K$, $I$)]
        #uncover("8-")[- változómentes]
        #uncover("9-")[
          #align(center + bottom)[
            #rect(width: 100%, height: 120pt, fill: rgb("#3c3836"))[
              #align(center + horizon)[
                #align(center)[
                  $
                    S & = lambda x.(lambda y.(lambda z.x z(y z))) \
                    K & = lambda x.(lambda y.x) \
                    I & = lambda x.x
                  $
                ]
              ]
            ]
          ]
        ]
      ],
    )
  ],
)

== architektúra

#slide(
  title: [architektúra],
  repeat: 11,
  self => [
    #let (only,) = utils.methods(self)

    #only("1-10")[implementálási nyelv: *Rust*]

    #only("2-10")[*frontend (elm-architektúra)*]
    #only("3-6")[- UI $arrow$ `update` $arrow$ `view` körforgás]
    #only("4-6")[- message-queue alapú állapotkezelés]
    #only("5-6")[- tisztán kliensoldali futás (*WASM*)]
    #only("6-6")[- cpu-heavy redukciós számítások kiszervezése *Web Worker*-ekbe]

    #only("7-10")[*backend (compiler felépítés)*]
    #only("8-9")[- compiler frontend: lexer és parser]
    #only("9-9")[- compiler backend: G-machine alapú redukciós gépek]

    #only("10")[*API réteg:* a frontend és backend között *facade* mintával]

    #only("11")[
      #block(width: 100%)[
        #set text(size: 0.7em, fill: rgb("#d4be98"))

        #align(center)[
          #fletcher.diagram(
            node-stroke: 1.2pt,
            edge-stroke: 1pt + rgb("#d4be98"),
            spacing: (1em, 2.5em),
            {
              let fg = rgb("#d4be98")
              let bg = rgb("#1d2021")
              let gray_bg = rgb("#3c3836")
              let blue = rgb("#7daea3")
              let green = rgb("#a9b665")
              let orange = rgb("#e78a4e")

              let fe_style = (fill: blue.darken(70%), stroke: blue, corner-radius: 3pt, inset: 0.5em)
              let api_style = (fill: green.darken(70%), stroke: green, corner-radius: 3pt, inset: 0.5em)
              let be_style = (fill: orange.darken(70%), stroke: orange, corner-radius: 3pt, inset: 0.5em)

              let group_style = (
                fill: gray_bg,
                stroke: (paint: fg.darken(30%), dash: "solid"),
                corner-radius: 5pt,
                extrude: 0.5em,
              )

              node(enclose: ((0, 0), (-1, 1), (1, 1)), ..group_style)
              node((0, -0.7), [*frontend*], stroke: none)

              node(enclose: ((-1, 3), (1, 4)), ..group_style)
              node((0, 4.7), [*backend*], stroke: none)

              // --- FRONTEND ---
              node((0, 0), `UI`, ..fe_style, name: <ui>)
              node((-1, 1), `view`, ..fe_style, name: <view>)
              node((1, 1), `update`, ..fe_style, name: <update>)

              edge(<view>, <ui>, "->", label: [megjelenés], label-side: left, label-pos: 0.5, label-sep: 7pt)
              edge(<ui>, <update>, "->", label: [üzenetek], label-side: left, label-pos: 0.5, label-sep: 7pt)
              edge(<update>, <view>, "->", label: [új modell], label-side: left, label-pos: 0.5, label-sep: 7pt)

              // --- API ---
              node((0, 2), [`lexor-api`], ..api_style, width: 14em, name: <api>)

              edge(
                <api>,
                <update>,
                "<->",
                label: [kérések / válaszok],
                label-side: right,
                label-fill: bg,
                label-sep: 6pt,
                label-anchor: "north-west",
              )

              // --- BACKEND ---
              node((-1, 3), [$lambda$ lexer + parser], ..be_style, name: <lam-p>)
              node((-1, 4), [$lambda$ redukciós gép], ..be_style, name: <lam-e>)

              node((1, 3), [SKI lexer + parser], ..be_style, name: <ski-p>)
              node((1, 4), [SKI redukciós gép], ..be_style, name: <ski-e>)

              edge(<api>, <lam-p>, "<->")
              edge(<api>, <ski-p>, "<->")
              edge(<lam-p>, <lam-e>, "<->")
              edge(<ski-p>, <ski-e>, "<->")
            },
          )
        ]
      ]
    ]
  ],
)

== redukciós gépek

*graph reduction machine* (G-machine)

#only("2-4")[tankönyvi algoritmus:]
#only("2-4")[- csúcsok $arrow$ kombinátorok és applikáció]
#only("3-4")[- magát az absztrakt szintaxisfát értékeljük ki]
#only("4")[- közben módosítunk rajta]
#only("5-7")[*optimalizált* algoritmus:]
#only("6-7")[- új féle csúcs: *indirekció*]
#only("7")[- #text(fill: rgb("#98971a"))[részfa megosztás]]
#only("8-")[*optimalizált* algoritmus]

#only("8-11")[adatszerkezet:]
#only("9-10")[- hagyományosan: rekurzív, pointer alapú AST]
#only("10")[- #text(fill: rgb("#cc241d"))[pointer chasing! folyamatos allokációk!]]
#only("11")[- #strike[rekurzív, pointer alapú AST]]
#only("11")[- aréna allokátor alapú Flat-AST]
#only("12-")[adatszerkezet]

#only("12-")[stratégiák:]
#only("13-")[- normál sorrend (leftmost-outmost)]
#only("14-")[- applikatív sorrend (leftmost-innermost)]
#only("15-")[- call-by-name/call-by-value (argumentumok kiértékelése)]

== stratégiák

#align(center)[
  #rect(width: 72.6%, height: 100%, fill: rgb("#3c3836"), stroke: rgb("#e78a4e") + 2pt)[
    #image("./strategy_base.png")
  ]
]

== stratégiák (normal order)

#align(center)[
  #rect(width: 72.6%, height: 100%, fill: rgb("#3c3836"), stroke: rgb("#e78a4e") + 2pt)[
    #image("./strategy_first.png")
  ]
]

== stratégiák (applicative order)

#align(center)[
  #rect(width: 72.6%, height: 100%, fill: rgb("#3c3836"), stroke: rgb("#e78a4e") + 2pt)[
    #image("./strategy_second.png")
  ]
]

== $lambda$-kalkulus specifikus kihívások

#only("1-2")[a $lambda$-kalkulus kiértékelése bonyolultabb]
#only("2")[- változónevek ($alpha$-ekvivalencia)]
#only("3-6")[*de Bruijn-indexek:*]
#only("4-6")[- nevek $arrow$ indexek (számok)]
#only("5-6")[- hány szinttel feljebb található az őt kötő $lambda$-absztrakció]
#only("6")[
  $
    lambda x.lambda y. x arrow.double.r.l.long lambda lambda 2
  $
]
#only("7-8")[*de Bruijn-indexek*]

#only("7-8")[oda-vissza konverzió a tradicionális alak és a de Bruijn-forma között]

#only("8")[(ez segít a SKI $arrow lambda$ konverzióban is)]

== tesztelés

egységtesztek mellett *property-based testing*
#only("2")[- hasonló a fuzzinghoz]

#only("3-")[*orákulum:*]
#only("4-")[- külső, helyes interpreter futtatása konténerben]
#only("5-")[- eredmények összehasonlítása]

== visuals

- a backend JSON-ként továbbítja a redukciós fát
#pause
- a frontend *szemantikailag* színezve jeleníti meg a kifejezéseket

== visuals

#align(center)[
  #rect(width: 34%, height: 100%, fill: rgb("#3c3836"), stroke: rgb("#e78a4e") + 2pt)[
    #image("./visuals.png")
  ]
]

== visuals

- a backend JSON-ként továbbítja a redukciós fát
- a frontend *szemantikailag* színezve jeleníti meg a kifejezéseket
#pause
- gráf konverzió minden egyes állapothoz
#pause
- agresszív cachelés

== ui

- egui keretrendszer (Dear ImGui feel)
#pause
- *immediate-mode* GUI
#pause
- docking support (compiler explorer feel)

== ui

#align(center)[
  #rect(width: 83%, height: 100%, fill: rgb("#3c3836"), stroke: rgb("#e78a4e") + 2pt)[
    #image("./ui.png")
  ]
]

== hova tovább?

#pause
- indirekció ki/be kapcsolása
#pause
- *guided learning* mód
#pause
- nyelvtan bővítése új elemekkel (számok, műveletek)
#pause
- $lambda arrow$ SKI konverzió
#pause
- frontend személyre szabhatóságának növelése

== összegzés

#pause
- egy *interaktív* felület a SKI-kalkulussal való ismerkedésre
#pause
- gyors redukció #pause (mind lépésszám, mind cpu cycles terén)
#pause
- intuitív vizuális segítő elemek

==

#align(center + horizon)[
  #text(size: 2em, weight: "bold")[Köszönöm a figyelmet!]
]
