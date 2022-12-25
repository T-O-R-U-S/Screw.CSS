# Screw.CSS
## Procedural utility classes for CSS

### Screw is not production-ready (yet ;), and many features are in the concept stage

### Why Screw?

### Simpler:

Tailwind's `md:w-5 md:bg-blue` becomes `md:(w-5 bg-blue)`

### More versatile

Tailwind doesn't support using it's own utility classes in `supports` queries; Screw does!

- Gradient text is a popular effect on websites; except some browsers may not support it, 
resulting in invisible and illegible text if `background-clip: text` is unsupported

    `supports(bg-clip:text):(bg-clip:text text-colour:transparent)`

- In Tailwind:

  `supports-[background-clip:text]:bg-clip-text supports-[background-clip-text]:text-transparent`

- Tailwind has child selector by default; Screw does!

    `child:(w:20 h:20)`

The lack of selectors like `child` or `supports` in Tailwind means you will have to leave your HTML more often
in order to extract these properties into their own utility class; with Screw, you don't have to leave as often!

### Procedural

Tailwind is not procedural; screw is!

- Screw: `grid-cols:14`
- Tailwind (JIT): `grid-cols-[repeat(14, minmax(0, 1fr))]`

### Features

#### Selectors as functions

`nth:3:(bg:blue-300)`

#### Easy custom values

`bg:rgba(0, 255, 0, 0)`

`w:400px`

`h:20vh`

#### Procedural variable widths/heights, and colours from the Tailwind palette

- Tailwind lacks a `h-17` property; Screw procedurally generates numeric variables for things like 
border radii and widths and heights. Screw already has a `h:17` property
- Colours from the tried-and-true Tailwind palette that you're accommodated to

### Syntax

For functions with one parameter, the colon `:` may be used to pass a parameter.

For functions with multiple, you can use `{function name}:{parameters}` to pass one parameter,
or you can use `{function name}({parameters})`, or `{function name}:({parameters})` depending on the context
and code-styling preference.

#### Standard rules

- For rules and selectors with one parameter, you should use `{function name}:{parameter}`
- For selectors that need rules nested in them (i.e `supports()`), you should use the `{function name}({parameters})`
syntax
- For selectors, you should use `{function name}({paremeters})`
- For rules with several parameters or media queries with parameters, you should use `{function name}:({parameters})`

Examples:

- `nth:3:(bg:blue-300 text:yellow-400)`

- `md` with a max of `lg`: `md:lg:(bg:yellow-300 text:blue-400)`

- `max-lg:(bg:red-200 text:yellow-400)`