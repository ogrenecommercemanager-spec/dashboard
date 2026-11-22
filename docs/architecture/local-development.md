# Local development / TB-01.1

## Startup
`docker compose up --build`, then visit http://localhost:5173. Services: db (private Compose network), api (localhost:8080), dashboard (localhost:5173). `db` must pass `pg_isready` before API start. SQLx runs embedded migrations at API startup.

## Manual acceptance checklist
1. Fresh checkout starts with Compose on a machine with Docker.
2. `curl -f http://localhost:8080/healthz` succeeds.
3. Clicking **Create and load demo business** shows `Demo Candle Shop` and a UUID.
4. `docker compose logs api` shows no migration or SQL errors.
5. Stopping/restarting containers retains demo records; `docker compose down -v` resets LOCAL demo DB only.
6. Verify API doesn't expose `/v1/dev` when APP_ENV is not `local` before any public staging use.

**Status:** Scaffold created; commands/browser checks must be executed on the developer machine. No claim tests have passed.

## Next tracer
TB-01.2: GitHub Actions CI running Rust formatting, clippy, tests; TypeScript typecheck/build; dependency/secret scanning and a deliberate failing-test demonstration. Add automated SQL/API/browser integration checks before calling TB-01.1 completed.
