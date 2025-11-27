CREATE TABLE habbits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL,
    archived_at DATETIME
);

CREATE TABLE complitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    habit_id INTEGER NOT NULL,
    done_at DATETIME NOT NULL,
    note TEXT,
    FOREIGN KEY (habit_id) REFERENCES habbits(id) ON DELETE CASCADE
);
