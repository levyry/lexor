#import "@preview/touying:0.7.3": *

#let slide(title: auto, ..args) = touying-slide-wrapper(self => {
  if title != auto {
    self.store.title = title
  }
  
  let header(self) = {
    set align(top)
    show: components.cell.with(fill: self.colors.neutral-lightest, inset: 1em)
    set align(center + horizon)
    set text(weight: "bold", size: 1.5em)
    if self.store.title != none {
      utils.call-or-display(self, self.store.title)
    } else {
      utils.display-current-heading(level: 2)
    }
  }
  
  let footer(self) = {
    set align(bottom)
    show: pad.with(.4em)
    set text(fill: self.colors.neutral-darkest, size: .8em)
    utils.call-or-display(self, self.store.footer)
    h(1fr)
    context utils.slide-counter.display() + " / " + utils.last-slide-number
  }
  
  self = utils.merge-dicts(
    self,
    config-page(
      header: header,
      footer: footer,
      fill: self.colors.neutral-lightest,
    ),
  )
  touying-slide(self: self, ..args)
})

#let title-slide(..args) = touying-slide-wrapper(self => {
  let info = self.info + args.named()
  let body = {
    set align(horizon + center)
    block(
      width: 100%,
      inset: (y: 1em),
      text(size: 2.5em, weight: "bold", info.title),
    )
    set text(fill: self.colors.neutral-darkest)
    if info.author != none { 
      block(
        info.author
      ) 
    }
    if info.institution != none and info.date != none {
      align(bottom + center, block[
          #text(size: 0.7em, info.institution)
          #h(1fr)
          #text(size: 0.7em, utils.display-info-date(self))
      ])
      
    }

  }
  self = utils.merge-dicts(self, config-page(fill: self.colors.neutral-lightest))
  touying-slide(self: self, body)
})

#let new-section-slide(self: none, body) = touying-slide-wrapper(self => {
  let main-body = {
    set align(center + horizon)
    set text(size: 2em, fill: self.colors.secondary, weight: "bold", style: "italic")
    utils.display-current-heading(level: 1)
  }
  self = utils.merge-dicts(self, config-page(fill: self.colors.neutral-lightest))
  touying-slide(self: self, main-body)
})

#let gruvbox-dark-theme(
  aspect-ratio: "16-9",
  footer: none,
  ..args,
  body,
) = {
  set text(size: 21pt, fill: rgb("#d4be98"))

  show: touying-slides.with(
    config-page(
      paper: "presentation-" + aspect-ratio,
      margin: (top: 6em, bottom: 1.5em, x: 2em),
    ),
    config-common(
      slide-fn: slide,
      new-section-slide-fn: new-section-slide,
    ),
    config-methods(alert: utils.alert-with-primary-color),
    
    config-colors(
      primary: rgb("#e78a4e"),
      secondary: rgb("#a9b665"),
      tertiary: rgb("#7daea3"),
      neutral-lightest: rgb("#1d2021"),
      neutral-darkest: rgb("#d4be98"),
    ),
    config-store(
      title: none,
      footer: footer,
    ),
    ..args,
  )

  body
}