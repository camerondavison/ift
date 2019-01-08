[![Build Status](https://travis-ci.org/camerondavison/ift.svg?branch=master)](https://travis-ci.org/camerondavison/ift)
[![Ift Version](https://img.shields.io/crates/v/ift.svg)](https://crates.io/crates/ift)

## IFT (interface templates)

Template strings to extract the correct interface and IpAddr to bind your to

Heavily inspired by https://github.com/hashicorp/go-sockaddr

Most of the time you are going to want to use the [eval](https://camerondavison.github.io/ift/ift/fn.eval.html#evaluate-a-interface-template)
function to evaluate a template.

The template is a string that starts with a [producer](https://camerondavison.github.io/ift/ift/fn.eval.html#producers)
and is followed by [filters](https://camerondavison.github.io/ift/ift/fn.eval.html#filters) and [sorts](https://camerondavison.github.io/ift/ift/fn.eval.html#sorts)
each of which is pipe `|` delimited. `eval` returns a vector of IpAddr objects
that can then be used to bind to.

Sometimes it makes more sense to bind to a single address. The [evals](https://camerondavison.github.io/ift/ift/fn.evals.html)
function helps with this, by expecting everything will go smoothly and returning
and `Optional<IpAddr>`


## To regenerate the rfc code
```bash
make gen
```

## To Update README
```bash
make update
```

## To Release

You can use the [cargo release](https://github.com/sunng87/cargo-release) command.

```bash
cargo release patch
```

License: MIT
