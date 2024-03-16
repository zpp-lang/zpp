<div align="center">
<br>
<img src="https://avatars.githubusercontent.com/u/154477754?s=400&u=92d0487c2d358c4dc96a56a12c87384774110c76&v=4" alt="Z++ Logo" style="width: 35%;">
<h1>Z++</h1>
An experimental, statically typed, fast, drop-in replacement for ZSharp
</div>

<p align="center">
<a href="#goals">Goals</a> |
<a href="#planned-features">Planned Features</a> |
<a href="#getting-started">Getting Started</a> |
<a href="#contributing">Contributing</a> |
<a href="#acknowledgements">Acknowledgements</a> 
</p>

## Goals
I am not intending to _replace_ ZSharp, I want to improve upon it. In my
eyes, there could've been so many things done a better way that weren't. ZSharp
has been abandoned since **2022** and hasn't seen much love since.

This project, while still aiming to be simple, tries to reimplement ZSharp and
make it much more usable, performant, and feature rich. Right now, I am still
working on implementing the basics of the language. **Z++ is not ready for production,
or any kind of usage at all.**

At the very least, I want to provide;
- The compiler / interpreter
- Feature rich standard library
- A package manager
- An IDE or an LSP server for VSCode

Bear in mind, this is simply a side project and may not even go anywhere. It might suddenly
get archived without a word. Don't expect too much to happen, especially anything big.

## Planned features
While I would love to maintain full compatibility with ZSharp, there are some things that I
decided we will not be including in the standard library, such as graphics, and the `ZS` package
to name a few.

With all that aside, here's what I want to have for the language before getting to some form of
release.
- More concise syntax, with helpful error reporting
- JIT Compilation and execution with LLVM
- A low level C API to language internals
- Compile time code analysis

## Getting started
You will need to build the compiler from its source. Ensure you have `rustup`, the Rust Toolchain,
and LLVM installed on your system. If you are on Windows, I highly recommend using [WSL](https://learn.microsoft.com/en-us/windows/wsl/about),
it will make your life so much easier.

Once you have the compiled binary, you can simply run the following command in your Terminal to
compile and execute a file.
`zxx file_name.zpp [optional arguments]`

## Contributing
Thank you for considering making a contribution to Z++! Contributions are welcome and strongly
encouraged, whether it's through code, documentation, bug reports, or any other form of help. If you have
a new feature to contribute, you should instead make a pull request to our [RFCs](rfcs) subproject.

### How to contribute
1. Ensure you have `rustup`, the Rust Toolchain, and LLVM installed on your system.
2. Fork the Z++ repository and clone it to your local machine.
3. Create a new branch to work on, and pick a descriptive name for your branch, such as `feature/ast`.
4. Implement and thoroughly test your changes. Having a buggy PR will not get it approved.
5. Submit a pull request to the Z++ repository.

### Code guidelines
- Follow the existing code style and conventions used in the project
- Write clear and concise code that is well documented, as if you're going to present it to someone
who has never worked on the codebase before.

### Reporting issues
If you encounter any bugs, or issues please create an issue on the GitHub repository. Provide 
detailed information about the problem, including the steps to reproduce.

## Acknowledgements
This would not of been possible without some really neat open source libraries and their
communities:

- [Inkwell](https://crates.io/crates/inkwell): Inkwell provides a rustic wrapper around the LLVM Rust bindings.
- [LLVM](https://llvm.org/): The LLVM Project provides a collection of modular and reusable compiler technologies.
- [clap](https://github.com/clap-rs/clap): Clap is an amazing command line argument parser that's blazing fast.
- [codespan](https://github.com/brendanzab/codespan): Codespan makes error reporting a breeze, while also making said
errors look stylish.


<br>
<sup>Copyright (c) 2024 Liam Dvorscak, LGPL 3.0-only</sup>