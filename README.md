# Platform monorepo — Wave A / TB-01.1

A *development-only* scaffold for the first tracer in PRD 01: browser → Rust Axum API → PostgreSQL. No merchant data, payment processing, production login, or enforced tenancy is included yet.

## Prerequisites
- Docker Desktop or Docker Engine with Compose v2
- Git

## Run
```bash
docker compose up --build
```
Open http://localhost:5173 and click **Create and load demo business**. The API is at http://localhost:8080/healthz.

## Verify
```bash
curl -fsS http://localhost:8080/healthz
curl -fsS -X POST http://localhost:8080/v1/dev/seed-business
```
The button issues POST followed by GET through the Vite proxy; the resulting name comes from PostgreSQL. Docker Compose provisions only local test services. To reset the local database: `docker compose down -v` (destructive to local demo data).

## What this does *not* establish
- No real authentication, Firebase verification, role enforcement, PostgreSQL RLS, auditability, production deployment or rollback.
- Local dev endpoints are explicitly disabled outside `APP_ENV=local`. Never publish these endpoints to the internet.
- CI/CD, automated integration/browser testing, deployed environments and independent storefronts belong to later TB-01 tracers.

See [docs/architecture](docs/architecture) and [AGENTS.md](AGENTS.md).
