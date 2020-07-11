CREATE TABLE system_metrics(
    id INTEGER NOT NULL ,
    last_updated INTEGER NOT NULL,
    total_memory INTEGER NOT NULL,
    used_memory INTEGER NOT NULL,
    total_swap INTEGER NOT NULL,
    used_swap INTEGER NOT NULL,
    cpu_usage REAL NOT NULL,
    PRIMARY KEY(id)
);


CREATE TABLE proc_metrics(
    id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    last_updated INTEGER NOT NULL,
    used_memory INTEGER NOT NULL,
    used_virtual INTEGER NOT NULL,
    cpu_usage REAL NOT NULL,
    read_bytes INTEGER NOT NULL,
    total_read_bytes INTEGER NOT NULL,
    writtem_bytes INTEGER NOT NULL,
    total_written_bytes INTEGER NOT NULL,
    PRIMARY KEY(id)
)
