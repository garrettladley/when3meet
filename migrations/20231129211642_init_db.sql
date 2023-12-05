DROP TABLE IF EXISITS meetings;

CREATE TABLE IF NOT EXISTS meetings
(
  id                INTEGER NOT NULL PRIMARY KEY,
  event_name        TEXT,
  no_earlier_than   TEXT,
  no_later_than     TEXT,
);

DROP TABLE IF EXISITS users;

CREATE TABLE IF NOT EXISTS users
(
  id            INTEGER NOT NULL PRIMARY KEY,
  name          TEXT,
  availability  TEXT,
  meeting_id    INTEGER,
  FOREIGN KEY (meeting_id) REFERENCES meetings(id)
);