CREATE TABLE IF NOT EXISTS races (
  id INTEGER PRIMARY KEY NOT NULL,
  runner_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  distance_km REAL NOT NULL,
  event_date DATE NOT NULL,
  goal_duration REAL,
  pacing_strategy REAL,
  default_aid_station_duration REAL,
  is_deleted BOOLEAN DEFAULT FALSE NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,

  FOREIGN KEY (runner_id) REFERENCES users (id)
    ON DELETE CASCADE
);
