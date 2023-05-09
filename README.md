<h1>
<p align="center">
<img src="https://calyxir.org/img/logo-text.svg" width="300">
</p>
<p align="center">
<a href="https://calyxir.org">A Compiler Infrastructure for Accelerator Generators</a>
</p>
</h1>

Calyx is an intermediate language and infrastructure for building compilers that generate custom hardware accelerators.

See the [Calyx website][site], [language documentation][docs] and the [documentation for the source code][source-docs] for more information.
Calyx's design is based on [our paper][paper].

## Installation

### Quick
If you want to try out the compiler, install it using `cargo`:
```
cargo install calyx
```

This will install the `calyx` binary can optimize and compile Calyx programs to Verilog or [CIRCT][].

### Recommended

Follow the [getting started][docs] instructions.

## Organization

This repository contains the source code for the following:
* [`calyx-utils`][]: Utilities for the Calyx compiler
* [`calyx-frontend`][]: Parser and frontend AST for the Calyx language.
* [`calyx-ir`][]: The Calyx intermediate language.
* [`calyx-opt`][]: Optimizations for the Calyx intermediate language.
* [`calyx`][]: The Calyx compiler driver.

You can also use the Calyx compiler as a library and implement your own optimizations. To do this, following the [source documentation][source-docs].

[site]: https://calyxir.org
[docs]: https://docs.calyxir.org
[source-docs]: https://docs.calyxir.org/source/calyx
[paper]: https://rachitnigam.com/files/pubs/calyx.pdf

[`calyx-utils`]: https://crates.io/crates/calyx-utils
[`calyx-frontend`]: https://crates.io/crates/calyx-frontend
[`calyx-ir`]: https://crates.io/crates/calyx-ir
[`calyx-opt`]: https://crates.io/crates/calyx-opt
[`calyx`]: https://crates.io/crates/calyx
[circt]: https://docs.calyxir.org/fud/circt.html