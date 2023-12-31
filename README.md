# Community

Simple application to keep track of your local community

## Development

### Tools Used

* `sqlx-cli`

  ```
  cargo install sqlx-cli
  ```

* `cargo-watch`

  ```
  cargo install cargo-watch
  ```

* `trunk`

  ```
  cargo install --locked trunk
  ```

* Rust WASM build support

  ```
  rustup target add wasm32-unknown-unknown
  ```

### Building the Project

1. Create the database

  ```
  sqlx db create
  ```
  ```
  sqlx migrate run --source core_app/migrations/
  ```

2. Build the frontend

  ```
  trunk build --release web_frontend/index.html
  ```

3. Build the rest

  ```
  cargo build --release
  ```

### Live Rebuilding in Dev

1. Rebuild the FE

  ```
  trunk watch web_frontend/index.html
  ```

2. Rebuild and run the BE

  ```
  cargo watch -c -q -w core_app/ -w web_backend/ -x 'run --bin web_backend'
  ```
