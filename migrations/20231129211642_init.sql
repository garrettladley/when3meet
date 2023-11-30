CREATE TYPE day_of_week AS ENUM ('Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday');

CREATE TABLE IF NOT EXISTS event (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    earliest_time TIMESTAMPTZ NOT NULL,
    latest_time TIMESTAMPTZ NOT NULL,
    days_of_week day_of_week[] NOT NULL
);
