# Merchant dashboard architecture

React + TypeScript with Vite. This is the **merchant admin dashboard**, not merchant storefronts (which remain separate repositories).

## Planned directories
```
apps/dashboard/src/
  app/          # app entry, routing, providers
  features/     # catalog, orders, employees, settings by domain
  components/   # genuinely reusable UI, not business-specific components
  lib/api/      # generated OpenAPI client + auth token transport
  hooks/        # shared hooks only
  styles/       # design tokens/styles
```

Keep fetching/serialization in an API layer, not scattered inside view components. Organize features by workflow, each with its own UI and tests. The API owns authorization decisions; hiding UI controls is not an access-control boundary. Avoid storing provider tokens in logs. Use generated clients when OpenAPI ships.

TB-01.1 is an intentionally single-file proof, to be refactored as domain workflows are added. For local Docker, Vite forwards `/v1` calls to `http://api:8080` to avoid browser CORS complexity.
