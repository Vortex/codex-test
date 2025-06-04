# codex-test

This repository includes a starter Astro project for a music website aimed at metal fans. See `metal-fans-website/README.md` for instructions.

## Rust Backend

The `axum-sqlite-backend` directory contains a small example API server built with **Axum**, **SQLx**, and **SQLite**. To run it:

```bash
cd axum-sqlite-backend
cargo run
```

It provides a simple `/` endpoint as well as `/users` for listing and creating users stored in a local SQLite database.
