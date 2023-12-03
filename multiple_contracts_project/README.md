# Multiple Contracts Project

- PSP34 with market feature
- PSP22 token

Create workspace `Cargo.toml`

## PSP34 Market

Create contract

```sh
cargo contract new psp34_market
```

Update `Cargo.toml`

```

```

Update contract `lib.rs` with openbrush PSP34

Build contract should be successful

```sh
cd psp34_market
cargo contract build
```

## Logic crate

`logic_traits` is a common module contains logic slices of all contracts in the project. For example it contains

- LogicSliceA (used by contract_one)
- LogicSliceB (used by contract_one)
- LogicSlideC (used by contract_two)
- LogicSlideD (used by both contracts, the `upgradable` trait in this case)

The reason all the logic defined (and implemented as well for convenient) in a common module is for cross-contract call betweens contracts in the project.

Each logic slice basically contains

- `types.rs`: define data, error.
- `traits.rs`: define trait functions that can be used by other contracts by cross-contract call.
- `impls.rs`: implement all message functions, include but not limited to functions defined in `traits.rs`.

In this tutorial we are going to implement PSP34 very basic market feature.



