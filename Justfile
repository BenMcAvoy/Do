watch:
  export DATABASE_URL="mysql://root:root@localhost/todos"
  cargo watch -x run

debug:
  export DATABASE_URL="mysql://root:root@localhost/todos"
  sqlx migrate run
  cargo run

run:
  export DATABASE_URL="mysql://root:root@localhost/todos"
  sqlx migrate run
  cargo run --release
