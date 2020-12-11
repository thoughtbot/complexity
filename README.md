# `complexity`

Calculate an approximation of code complexity per file in a language-agnostic way.

## Motivation

If you're new to a codebase, it's helpful to understand at a glance what files
may be particularly complex. With that guidance, developers can more quickly
read through the code to understand hotspots.

At thoughtbot, we work in codebases of all shapes and languages, including
Ruby, Elixir, Python, Scala, TypeScript/JavaScript, Go, Elm, Swift, and Java.
This CLI tool aims to highlight complexity across any of these codebases by
assigning simple heuristics to increases in indentation.

This concept has been discussed in [this paper]; `complexity` does not intend
to mimic approaches in this paper directly, although the motivations discussed
in the paper – especially avoiding calculating cyclomatic complexity ([McCabe])
given requirements of AST parsing and analysis due to time and language
requirements – are of considerable overlap.

[this paper]: https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.68.3558&rep=rep1&type=pdf
[McCabe]: https://en.wikipedia.org/wiki/Cyclomatic_complexity

## Installation

### MacOS with Homebrew

```sh
brew tap thoughtbot/formulae
brew install complexity
```

## Usage

Let's grab the 20 most complex files:

```sh
complexity | sort -n --reverse | head -n 20
```

## License

Copyright 2020 Josh Clayton and thoughtbot, inc. See the [LICENSE](LICENSE).
