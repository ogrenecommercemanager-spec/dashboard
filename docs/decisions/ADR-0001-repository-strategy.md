# ADR 0001 — Hybrid repository strategy
Status: Accepted for initial Wave A scaffold | Date: 2026-11-22

## Context
Shared infrastructure/API/dashboard requires coordinated code changes and consistent AI coding agent instructions. Each merchant needs independently controlled website source and deployment.

## Decision
Use one private `platform` monorepo for shared backend, dashboard, workers, contracts, infrastructure and docs. Maintain separate private `storefront-starter` and merchant-specific storefront repos.

## Consequences
Shared API and contracts can change in one PR; CI must be path-aware as the repo grows. Storefront deployments do not require a central platform rebuild. API versions and published contracts become cross-repo compatibility boundaries. No merchant owns shared backend code.
