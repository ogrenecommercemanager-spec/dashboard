# Rust API architecture

**Approach:** Axum modular monolith initially, one binary/deployment. SQLx handles PostgreSQL, migrations, transactions and typed queries. REST `/v1` and OpenAPI contracts as PRD 02 matures.

## Target module shape (PRD 02)
```
api/src/
  main.rs              # config, startup, shutdown
  app.rs               # router composition and app state
  config.rs            # validated configuration
  error.rs             # typed API error envelope
  auth/                # Firebase identity, tenant resolver, authorization
  businesses/
    mod.rs
    routes.rs          # endpoint registration
    handlers.rs        # HTTP input/output only
    service.rs         # domain decisions and transaction boundaries
    repository.rs      # SQLx queries only
    models.rs          # domain and DTO types
  catalog/ inventory/ orders/ payments/ audit/ jobs/
```

**Important:** TB-01.1 keeps its tiny routes in `main.rs` strictly for simplicity; refactor in TB-02.1 instead of prematurely generating empty modules.

## Request flow
Request → correlation/request ID → verified Firebase identity (PRD 03) → membership & capability check → service → RLS-scoped database transaction → response. All tenant-owned SQL must execute under verified per-transaction context; revoke or missing context fails closed. No direct client-to-database or agent direct SQL.

## Conventions
- Handler parses/validates; service handles business invariants; repository owns SQL.
- Explicit transactions for multi-step writes; use DB constraints as backstop.
- Money is integer minor units plus currency; never use float money arithmetic.
- API errors eventually use `{code,message,request_id,details?}`; no SQL internals or tenant details leaked.
- Distinguish readiness (DB reachable) from liveness (process healthy) before cloud rollout.
- Use outbox and idempotency keys for later durable side effects.
- Keep generated OpenAPI + TypeScript/Swift clients versioned and drift-tested.

## Security boundary
`/v1/dev/*` endpoints are **local tracer only**, disabled unless `APP_ENV=local`. They have no authentication and must never be exposed publicly. Remove/replace before any public environment.
