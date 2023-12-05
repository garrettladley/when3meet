CREATE TABLE IF NOT EXISTS meetings (
  id                    UUID UNIQUE PRIMARY KEY,
  name                  TEXT NOT NULL,
  start_date            TIMESTAMPTZ NOT NULL,
  end_date              TIMESTAMPTZ NOT NULL,
  no_earlier_than_hr    INTEGER NOT NULL,
  no_earlier_than_min   INTEGER NOT NULL,
  no_later_than_hr      INTEGER NOT NULL,
  no_later_than_min     INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id            UUID UNIQUE PRIMARY KEY,
  name          TEXT NOT NULL,
  availability  TEXT NOT NULL,
  meeting_id    UUID NOT NULL,
  FOREIGN KEY (meeting_id) REFERENCES meetings(id)
);
