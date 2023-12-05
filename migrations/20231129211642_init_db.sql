DROP TABLE IF EXISITS meetings;

CREATE TABLE IF NOT EXISTS meetings
(
  id                    INTEGER NOT NULL PRIMARY KEY,
  name                  TEXT NOT NULL,
  start_date            TEXT NOT NULL,
  end_date              TEXT NOT NULL,
  no_earlier_than_hr    INTEGER NOT NULL,
  no_earlier_than_min   INTEGER NOT NULL,
  no_later_than_hr      INTEGER NOT NULL,
  no_later_than_min     INTEGER NOT NULL,
);

DROP TABLE IF EXISITS users;

CREATE TABLE IF NOT EXISTS users
(
  id            INTEGER NOT NULL PRIMARY KEY,
  name          TEXT NOT NULL,
  availability  TEXT NOT NULL,
  meeting_id    INTEGER NOT NULL,
  FOREIGN KEY (meeting_id) REFERENCES meetings(id)
);