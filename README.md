<h1 align="center">when3meet</h1>

<br />

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/garrettladley/when3meet/actions/workflows/backend.yml">
    <img src="https://github.com/garrettladley/when3meet/actions/workflows/backend.yml/badge.svg"
      alt="Backend Workflow Status" />
  </a>
  <a href="https://github.com/garrettladley/when3meet/actions/workflows/audit.yml">
    <img src="https://github.com/garrettladley/when3meet/actions/workflows/audit.yml/badge.svg"
    alt="Security Audit Workflow Status" />
  </a>
</div>

## Quick Start

> [!NOTE]
> The following are prerequisites for running the application. Please refer to their respective documentation to install the dependencies for your operating system.
>
> - Rust
> - Docker
> - Node

### Backend

***Ensure the Docker daemon is running***

```bash
cd backend
./scripts/init_db.sh
cargo run
```

### Frontend

```bash
cd frontend
npm i
npm run dev
```

## External Libraries Used

- [actix-web](https://actix.rs/): a web framework for Rust.
- [sqlx](https://docs.rs/sqlx/latest/sqlx/): an asynchronous, pure Rust SQL crate featuring compile-time checked queries without a DSL.
- [Solid.js](https://www.solidjs.com/): a declarative JavaScript library for building user interfaces.
