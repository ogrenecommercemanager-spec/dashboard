# PostgreSQL schema and migration standards

## Storage strategy
Shared PostgreSQL per platform, tenant-owned records keyed by `business_id`, enforced with PostgreSQL RLS from PRD 02/03. **Current TB-01.1 database is NOT tenancy-safe**; `businesses` is a bootstrap demo table only.

## Schema conventions
- Use `snake_case`, plural table names, singular column names, UUID primary keys.
- `created_at` and `updated_at` use `TIMESTAMPTZ` where appropriate.
- Prefer explicit foreign keys, check constraints, unique indexes and tenant-consistent composite keys.
- Index tenant-owned hot paths beginning with `business_id` when query patterns justify it.
- Currency + signed integer minor units; quantity invariants via constraints/transactions.
- Audited writes and immutable order-line snapshots when commerce is introduced.

## Tenancy / safety checklist (PRD 02–03)
- Separate non-owner runtime DB role (not table owner, not `BYPASSRLS`).
- Enable/force RLS on tenant tables; test using runtime role, not admin credentials.
- Set trusted tenant context with `SET LOCAL` (or equivalent transaction-local setting) *inside every transaction*; fail closed when absent.
- Confirm pooled connection reuse cannot carry stale tenant context.
- Validate membership/capability in API from server-side identity, never trust a client-provided business ID.
- Cross-business foreign key consistency, negative RLS tests A↔B, invitation/revocation tests.

## Migrations
Versioned append-only SQL files in `api/migrations/`; reviewed with API change. Adopt expand/migrate/contract rollout for breaking changes. Never automatically delete tenant records or run destructive prod rollbacks. Test migration against fresh and upgraded DBs.

## Planned domain tables
`businesses`, `users`, `memberships`, `invitations`, `products`, `product_variants`, `inventory_items`, `carts`, `orders`, `order_items`, `payments`, `refunds`, `discounts`, `audit_events`, `idempotency_keys`, `outbox_events`, `delegated_agent_grants`, plus deployment/job tables described by other PRDs.
