watch:
  export DATABASE_URL="sqlite:todos.db"
  cargo watch -x run

debug:
  export DATABASE_URL="sqlite:todos.db"
  sqlx migrate run
  cargo run

run:
  export DATABASE_URL="sqlite:todos.db"
  sqlx migrate run
  cargo run --release
