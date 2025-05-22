CREATE TABLE wisdom.symbols(
    id uuid PRIMARY KEY,
    title text not null,
    formula text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

CREATE UNIQUE INDEX ON wisdom.symbols(title);
