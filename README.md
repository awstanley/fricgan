# fricgan - A simple (byte based) crate

[![Build Status](https://travis-ci.org/awstanley/fricgan.svg?branch=master)](https://travis-ci.org/awstanley/fricgan)
[![Build status](https://ci.appveyor.com/api/projects/status/5kqunyhlyoe4mvv6/branch/master?svg=true)](https://ci.appveyor.com/project/awstanley/fricgan/branch/master)

`fricgan` is a trivial byte manipulation crate, which targets common operations used in both `std` and `no_std` work.  I am releasing it as it is part of the code that I keep re-using, and I've had more than one request to access it.

Internally there is a lot of direct pointer manipulation, which is entirely safe if the buffers passed into the system are the right size.  By default the `safety-checks` feature is turned on, which installs an assert prior to copying.  Default copying is safe (using iteration over slices), but a copy using `copy_nonoverlapping` (also optionally protected by `safety-checks`) is available guarded by the feature `unsafe`.

## Features

All `IO` types are feature protected (so `io-{i,u}{8,16,32,64}`), and so is the `VLQ` trait (by `vlq`).  Only 32-bit and 64-bit unsigned values are implemented for `VLQ`, and they are feature protected behind `vlq-{32,64}`.  These can be easily tuned to what is required by a given application or library.

String support (for those who are fine with `std`) is included behind either `io-string` or `vlq-string`.  `FricganString` (`io-string`) requires an `IO` implemented type to be implemented an integer, or more precisely:

```rust
where V: ToPrimitive + FromPrimitive + Unsigned + IO
```

`VLQString` (`vlq-string`) requires VLQ:

```rust
where V: ToPrimitive + FromPrimitive + Unsigned + VLQ
```

`io-string` and `vlq-string` require `num`, which is compiled with `std` support.  (There are ways to do this without `num`, but there is no need to reinvent the wheel in this package when it provides sensible guards.)

The default configuration disables everything except `IO` and the default `[u8]` implementation.  (These cannot be disabled by features.)

All builds implement `IO` on `[u8]`, as it is used internally (it is where every safety check occurs, save for the single byte variants which use slice indexing).

## Changes

Changes are kept in `CHANGELOG.md` (in the root of the repository), and begin at version 0.1.0.  (Prior to 0.1 there was a brief public release, some formatting fixes, and the addition of floating point types.  Tests were also added prior to 0.1.)

## Licence

Apache 2.0 or MIT, at your option.