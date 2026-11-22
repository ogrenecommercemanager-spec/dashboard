# Monorepo architecture

**Decision:** One private `platform` repository contains shared API, dashboard, workers, contracts, infrastructure, and docs. `storefront-starter` and each customized merchant storefront have separate private repositories and deploy independently.

## Boundaries
- `api/`: one deployable Rust Axum service (modular monolith); domain ownership of commerce records.
- `apps/dashboard/`: TypeScript React merchant administration interface; uses API only.
- `workers/`: durable job processors (future PRD 10); no business logic duplication.
- `packages/contracts/`: authoritative OpenAPI schemas and generated client artifacts (future PRD 02).
- `infra/`: Compose/GCP/Firebase provisioning and deployment pipelines.
- `docs/`: architecture, API/database conventions, operational runbooks, ADRs.
- `.github/workflows/`: CI/CD (TB-01.2 onward).

## Dependencies
Storefronts and dashboard call published API contracts; no frontend direct PostgreSQL access. API and worker may share versioned Rust domain crates later; do not create many crates until a real dependency boundary emerges. Merchant repos do **not** import internal platform source.

## Deployment independence
One repository does not mean one deployment. Each service has its own build artifact, gates, release history, and rollback. Production shared backend/auth/payment/schema changes require operator-reviewed PR.

## Build order
TB-01.1 local end-to-end; TB-01.2 CI; TB-01.3 storefront preview; TB-01.4 isolated deploy/rollback; TB-01.5 production permissions and secrets.
