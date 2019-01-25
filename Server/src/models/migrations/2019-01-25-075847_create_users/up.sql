CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    phone TEXT NOT NULL,
    validation_code INTEGER NOT NULL,
    room_id INTEGER REFERENCES rooms(id)
)