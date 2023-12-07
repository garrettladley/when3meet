CREATE TABLE IF NOT EXISTS meetings (
  id      UUID UNIQUE PRIMARY KEY,
  name    TEXT NOT NULL,
  range   TSTZRANGE NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id            UUID UNIQUE PRIMARY KEY,
  name          TEXT NOT NULL,
  availability  TSTZRANGE[] NOT NULL,
  meeting_id    UUID NOT NULL,
  FOREIGN KEY (meeting_id) REFERENCES meetings(id) ON DELETE CASCADE
);
