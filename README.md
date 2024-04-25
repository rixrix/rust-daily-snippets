<p align="center">
    <img src="./assets/logo.png" height="100">
    <h3 align="center">Rust Daily Snippets</h3>
</p>

My daily Rust journey in 'Rust Daily Snippets' a personal repository documenting my progress and projects in Rust programming. From web apps to command-line tools, discover my daily learnings and creations.

## Features

- webapp
  - Actix
  - Axum
- Generics
  - Basic with Ord trait
- Enums
  - Basic with Option trait

## Requirements

- Rust

## How to add new crates

1. Update Cargo.toml members array ie.

```
members = [
  ...
  "crates/your-crates-name"
]
```

2. Run cargo, and add your new crates

```
cargo new crates/your-crates-name
```

## How to run a crates package

Specify the package you want to try. Remove the "crates/" folder

```
cargo run --package your-crates-name
```

## Tech Stack

> Rust only!

## License

[The Unlicense](https://choosealicense.com/licenses/unlicense/)
