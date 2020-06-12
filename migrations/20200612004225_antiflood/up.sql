CREATE TABLE IF NOT EXISTS antiflood
(
    token       integer references tokens (id) NOT NULL PRIMARY KEY,
    banlist_all timestamp                      NOT NULL
);
