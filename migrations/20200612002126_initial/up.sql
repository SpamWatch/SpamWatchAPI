DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'permission') THEN
        CREATE TYPE permission AS ENUM ('User', 'Admin', 'Root');
    END IF;

END$$;

CREATE TABLE IF NOT EXISTS tokens
(
    id         SERIAL PRIMARY KEY,
    token      Text       NOT NULL,
    permission permission NOT NULL,
    userid     bigint     NOT NULL,
    retired    bool       NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS banlist
(
    id          bigint                         NOT NULL PRIMARY KEY,
    reason      Text                           NOT NULL,
    date        timestamp                      NOT NULL,
    admin_token integer references tokens (id) NOT NULL
);
