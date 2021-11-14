CREATE TABLE IF NOT EXISTS dash (
    id BIGINT NOT NULL PRIMARY KEY,
    bind BOOLEAN NOT NULL,
    levels ENUM('Administrator', 'Owner', 'Common')
)