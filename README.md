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
