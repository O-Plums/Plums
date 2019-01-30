CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    phone_code VARCHAR(5) NOT NULL,
    phone VARCHAR(16) NOT NULL,
    username VARCHAR(24),
    room_id INTEGER REFERENCES rooms(id),
    gender VARCHAR(10),
    validation_code INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)