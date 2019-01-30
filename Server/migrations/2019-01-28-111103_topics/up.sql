CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    theme INTEGER REFERENCES themes(id),
    created_at TIMESTAMP NOT NULL
)