# EvalEx

EvalEx is a powerful expression evaluation library for Elixir, based on [evalexpr]() using [rustler]().

## Installation

Add `:evalex` to the list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:evalex, "~> 0.1.0"}
  ]
end
```

## Type conversion table

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
