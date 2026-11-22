-- Minimal *local tracer* table. PRD 02 adds ownership, permissions, RLS and tenancy policy.
CREATE TABLE businesses (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
