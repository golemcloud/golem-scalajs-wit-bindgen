# golem-scalajs-wit-bindgen

Command line interface to generate Scala.js bindings for [Golem Cloud](https://golem.cloud).


## Installation

To install `golem-scalajs-wit-bindgen` you currently need to use `cargo`, Rust's build tool. 

To get `cargo` on your system, we recommend to use [rustup](https://rustup.rs/):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
rustup default stable
```

Then you can install `golem-scalajs-wit-bindgen` with the following command:

```shell
cargo install golem-scalajs-wit-bindgen
```

Or, you can clone this repository and install with the following command:

```shell
cargo install --path .
```

## Running

To run `golem-scalajs-wit-bindgen` and get more help, you can execute the following command:

```shell
golem-scalajs-wit-bindgen --help 
```