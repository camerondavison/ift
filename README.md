[![Build Status](https://travis-ci.org/camerondavison/ift.svg?branch=master)](https://travis-ci.org/camerondavison/ift)
[![Ift Version](https://img.shields.io/crates/v/ift.svg)](https://crates.io/crates/ift)

## IFT (interface templates)

Template strings to extract the correct interface and IpAddr to bind your to

Heavily inspired by https://github.com/hashicorp/go-sockaddr


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
