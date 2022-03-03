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

[this paper]: https://doi.org/10.1016/j.scico.2009.02.005
[McCabe]: https://en.wikipedia.org/wiki/Cyclomatic_complexity

## Installation

### MacOS with Homebrew

```sh
brew tap thoughtbot/formulae
brew install complexity
```

## Configuration

`complexity` has configuration options to ignore certain file extensions or
substrings with paths.

To install this default configuration, run:

```sh
complexity install-configuration
```

This creates/overwrites a yaml configuration at
`$HOME/.config/complexity/complexity.yml`.

By default, extensions including `lock`, `toml`, `json`, and `md` are ignored.

`complexity` will automatically honor `.gitignore` settings.

## Usage

### Basic

Let's grab the 20 most complex files:

```sh
complexity | sort -n --reverse | head -n 20
```

Within the [Discourse codebase], for example, here's what the output might look like:

```
  487.96 ./spec/components/guardian_spec.rb
  465.19 ./spec/requests/users_controller_spec.rb
  363.09 ./spec/requests/topics_controller_spec.rb
  311.10 ./spec/models/topic_spec.rb
  273.57 ./lib/javascripts/messageformat.js
  266.61 ./spec/models/user_spec.rb
  248.28 ./app/assets/javascripts/discourse/app/controllers/topic.js
  238.81 ./app/assets/javascripts/discourse/tests/fixtures/discovery-fixtures.js
  219.06 ./script/import_scripts/socialcast/test/test_data.rb
  207.01 ./app/controllers/users_controller.rb
  205.86 ./app/assets/javascripts/discourse/tests/unit/lib/pretty-text-test.js
  202.18 ./app/assets/javascripts/discourse/tests/fixtures/topic.js
  200.17 ./lib/search.rb
  193.05 ./app/assets/javascripts/discourse/app/controllers/composer.js
  191.63 ./app/models/user.rb
  187.97 ./app/models/topic.rb
  186.10 ./spec/components/pretty_text_spec.rb
  179.25 ./spec/requests/session_controller_spec.rb
  174.89 ./spec/requests/groups_controller_spec.rb
  173.44 ./app/assets/javascripts/discourse/tests/integration/widgets/post-test.js
```

[Discourse codebase]: https://github.com/discourse/discourse

### Advanced

`complexity` supports alternative formatting options, like JSON and CSV.
Additionally, you can limit results by substring with the `--only` flag, or
modify paths to ignore with `--ignore`.

You can view the full suite of options by running `complexity help`.

## License

Copyright 2020 Josh Clayton and thoughtbot, inc. See the [LICENSE](LICENSE).
