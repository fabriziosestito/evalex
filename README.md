# EvalEx

[![CI](https://github.com/fabriziosestito/evalex/actions/workflows/main.yaml/badge.svg)](https://github.com/fabriziosestito/evalex/actions/workflows/main.yaml)
[![Rust CI](https://github.com/fabriziosestito/evalex/actions/workflows/rust-ci.yaml/badge.svg)](https://github.com/fabriziosestito/evalex/actions/workflows/rust-ci.yaml)
[![NIFs precompilation](https://github.com/fabriziosestito/evalex/actions/workflows/release.yaml/badge.svg)](https://github.com/fabriziosestito/evalex/actions/workflows/release.yaml)
--
EvalEx is a powerful expression evaluation library for Elixir, based on [evalexpr]() using [rustler]().

## About

EvalEx evaluates expressions in Elixir, leveraging the [evalexpr]() crate tiny scripting language.
Please refer to the [evalexpr documentation]() for extended information about the language.

## Installation

Add `:evalex` to the list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:evalex, "~> 0.1.0"}
  ]
end
```

## License

This library is licensed under Apache 2.0 License. See [LICENSE](LICENSE) for details.

## Usage

## Type conversion table

Elixir Types are converted to EvalEx types (and back) as follows:

| Elixir     | EvalEx                |
| ---------- | --------------------- |
| integer()  | Int                   |
| float()    | Float                 |
| bool()     | Boolean               |
| String.t() | String                |
| list()     | Tuple                 |
| tuple()    | Tuple                 |
| nil()      | Empty                 |
| pid()      | Empty (not supported) |
| ref()      | Empty (not supported) |
| fun()      | Empty (not supported) |
| map()      | Empty (not supported) |

## Rustler precompiled

EvalEx uses [RustlerPrecompiled](https://github.com/philss/rustler_precompiled) and ships with the following precompiled architectures:

so there is no need to install the Rust toolchain to use it in your project as a dependency
In case

## Links

- [evalexpr](https://github.com/ISibboI/evalexpr) The Rust crate doing most of the work.
- [RustlerPrecompiled](https://github.com/philss/rustler_precompiled) Use precompiled NIFs from trusted sources in your Elixir code.
- [NimbleLZ4](https://github.com/whatyouhide/nimble_lz4) Major inspiration for the RustlerPrecompiled GitHub actions workflow and general setup.
