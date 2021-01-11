CREATE TABLE polls (
    id INTEGER AUTOINCREMENT PRIMARY KEY
    name TEXT NOT NULL
    start TIMESTAMP NOT NULL
    end TIMESTAMP NOT NULL
    moderators
)

CREATE TABLE poll-votes (
    user_id INTEGER
    vote_id INTEGER
    vote INTEGER NOT NULL
    FOREIGN KEY(vote_id) REFERENCES polls(id)
)