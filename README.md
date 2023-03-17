# Low Level Journey

This repository contains all my experiments and projects of embedded programming
using the `Rust` Programing Language and the `Arduino UNO` board.

> Note: All of this work is meant to support my learning path of the programming language
> using it in every possible area like the one mentioned.

## Dependencies
All of the projects uses the [avr-hal](https://github.com/Rahix/avr-hal)
crate that exposes the *hardware abstraction layer* for AVR microcontrollers
and common boards like the `Arduino UNO`.

The other common dependencies are the listed in the
[template](https://github.com/Rahix/avr-hal-template.git)
provided by the the dependency mentioned above, same that is the base of all
the crates listed in this workspace.

## Tools
The main and the most important tool that I use to have a fluid workflow developing
embedded code is [ravedude](https://github.com/Rahix/avr-hal/tree/main/ravedude):
a cargo runner that wraps the capabilities of `avrdude`, these like the direct
access to the target board's serial console.

### TODOS
* Find a debugging tool.
* Learn more about `rustc`'s and `cargo`'s options and configurations.
* **Keep learning**\.
