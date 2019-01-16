[![Build Status](https://travis-ci.org/camerondavison/ift.svg?branch=master)](https://travis-ci.org/camerondavison/ift)
[![Ift Version](https://img.shields.io/crates/v/ift.svg)](https://crates.io/crates/ift)

## IFT (interface templates)

Template strings to extract the correct interface and IpAddr to bind your to.
Heavily inspired by https://github.com/hashicorp/go-sockaddr

### What is it?
`eval`([docs](https://camerondavison.github.io/ift/ift/fn.eval.html#evaluate-a-interface-template))
takes an interface template string. The template is a string that starts with a
[producer](https://camerondavison.github.io/ift/ift/fn.eval.html#producers)
and is followed by [filters](https://camerondavison.github.io/ift/ift/fn.eval.html#filters)
and [sorts](https://camerondavison.github.io/ift/ift/fn.eval.html#sorts)
each of which is pipe `|` delimited. `eval` returns a vector of [IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html) objects
that can then be used as bindings

### Usage

#### general
```rust
use ift::eval;
print!("{:?}", eval(r#"GetInterface "en0""#).unwrap());
```

#### actix
```rust
use actix_web::{
   server,
   App,
};
let mut s = server::new(|| { App::new() });
for ip in ift::eval("GetPrivateInterfaces").unwrap().into_iter() {
  s = s.bind((ip, 8080)).unwrap();
}
```

#### Example Templates
- get private interfaces
  `GetAllInterfaces | FilterFlags "up" | FilterForwardable | SortBy "default"`
- get private interfaces short
  `GetPrivateInterfaces`
- get specific interface by name
  `GetInterface "en0"`
- get only interfaces with ipv6 addresses
  `GetAllInterfaces | FilterIPv6`

### Examples
There are examples in the [examples](https://github.com/camerondavison/ift/tree/master/examples)
folder.
* [actix](https://github.com/camerondavison/ift/blob/master/examples/actix.rs) - bind multiple private interfaces


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
