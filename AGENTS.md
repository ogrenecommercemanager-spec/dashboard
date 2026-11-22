# Coding-agent operating rules

1. Read `README.md`, `docs/architecture/`, relevant PRDs, and ADRs before editing.
2. Work on exactly one tracer bullet at a time; do not build all backend layers before an end-to-end check.
3. For each change list touched files, commands run, results, tests, risks, and rollback/disable plan.
4. Never commit secrets, grant production privileges, access real merchant records, or deploy production without explicit operator review.
5. Backend routing stays thin; business rules belong to services and SQL to repositories. See `docs/architecture/backend.md`.
6. Later multi-tenant API endpoints must verify membership AND enforce PostgreSQL RLS; never rely on a client-supplied business ID for authorization.
7. Keep OpenAPI contracts and generated TypeScript/Swift clients in sync when introduced.
8. Preserve backward-compatible database migrations; no automated destructive migrations.
9. Do not represent this scaffold as proof of auth, RLS, production isolation or CI/CD correctness.
10. Stop and fix any failing tracer acceptance test before starting the next tracer.
