# fricgan - A simple (byte based) crate

fricgan is a byte manipulation crate, which targets common operations used in both std and core-only work.

It is being released mostly because I've written this code dozens of times for different little projects, but never previous put it all together before.  Hopefully this addresses people asking how/why things work in certain ways.

Currently set to version 0.0.1 to perform some final testing, but releasing into the wild to give people a chance to play with it (i.e. those who've explicitly asked for it).

## Features

All `IO` types are feature protected (so `io-{i,u}{8,16,32,64}`), and so is the `VLQ` trait (by `vlq`).  Only 32-bit and 64-bit unsigned values are implemented for `VLQ`, and they are feature protected behind `vlq-{32,64}`.

String support (for those who are fine with `std`) is included behind either `io-string` or `vlq-string`.

`io-string` and `vlq-string` bring in `num`, which is compiled with `std` support.

## Licence

Per the Cargo TOML: Apache 2.0 or MIT, at your option.