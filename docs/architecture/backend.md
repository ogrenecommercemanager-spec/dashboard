## Current module structure (TB-01.1)

The Rust API uses a modular monolith architecture. All API modules are compiled into one application and deployed together.

```text
api/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── state.rs
│   ├── error.rs
│   └── businesses/
│       ├── mod.rs
│       ├── handlers.rs
│       ├── models.rs
│       ├── service.rs
│       └── repository.rs
├── migrations/
└── tests/
    ├── business_db.rs
    └── business_http.rs
```

### Responsibilities

- **main.rs:** Application initialization, environment configuration, database connections, migrations, and HTTP server startup.
- **lib.rs:** Exposes application modules for reuse and integration testing.
- **app.rs:** Constructs the Axum router and registers HTTP endpoints.
- **state.rs:** Defines shared application resources, including the PostgreSQL connection pool.
- **error.rs:** Defines reusable API errors and HTTP error responses.
- **handlers.rs:** Extracts HTTP inputs, calls services, and constructs HTTP responses.
- **service.rs:** Implements business rules and coordinates operations.
- **repository.rs:** Owns SQLx database queries and persistence operations.
- **models.rs:** Defines Rust structures representing business records and API data.

### Adding a new API feature

When implementing a new domain capability:

1. Define the data structures and request/response models.
2. Implement database operations in the repository.
3. Implement validation, business rules, and transaction coordination in the service.
4. Implement HTTP handlers that call the service.
5. Register endpoints through the router.
6. Add focused integration tests for successful and rejected operations.
7. Update API contracts and architecture documentation when necessary.

Do not place arbitrary SQL queries or substantial business logic directly in HTTP handlers.

### Current limitations and planned improvements

TB-01.1 establishes the basic module architecture and verifies local database and HTTP behavior.

The current development endpoints are unauthenticated and must never be exposed publicly. Authentication, tenant isolation, capability-based authorization, structured API contracts, and more comprehensive observability will be introduced in subsequent PRDs.

### Architectural decision

The platform begins as a modular monolith rather than independently deployed microservices.

Modules should maintain clear domain boundaries so that individual components can be extracted into separate services if future scaling or operational requirements justify it. Deployment independence is not currently required between backend modules.
