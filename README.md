# Do
A basic todo app with Rust and SQLx

## Requirements
- [Rust](https://rust-lang.org)

## Usage
```bash
git clone https://github.com/BenMcAvoy/Do && cd Do
cargo install sqlx-cli
export DATABASE_URL="sqlite:todo.db"
sqlx db create
sqlx migrate run
cargo run --release
```
