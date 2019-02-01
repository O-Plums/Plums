CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    theme_id INTEGER REFERENCES themes(id),
    created_at TIMESTAMP NOT NULL
)