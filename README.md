![Build Status](https://github.com/cgorski/glow-effects/actions/workflows/rust.yml/badge.svg?branch=main)

[![Crates.io](https://img.shields.io/crates/v/glow-effects.svg)](https://crates.io/crates/glow-effects)
`Library`

# Glow Effects for Programmable LEDs

*Be sure to check out the [book](https://glowcontrol.rs/book/index.html) at the
project [website](https://glowcontrol.rs).*

The `glow-effects` crate is a Rust library designed to create effects
with generic programmable LEDs. It contains routines for various effects, as well as constructs to
create new effects. Notably, this crate is used by the [glow-control](http://github.com/cgorski/glow-control) project
to interface with Twinkly LED devices, but this crate is general enough to use
with any generic devices, or even virtual devices in software (e.g., for graphics effects).

## Features

- Various pre-built effects for programmable LEDs
- Constructs for creating new effects
- Generalizable to any LED device, real or virtual