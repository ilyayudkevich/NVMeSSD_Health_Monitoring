sudo -u postgres psql

### DROP DATABASE metrics_db;

CREATE DATABASE metrics_db;
CREATE USER nvme_user WITH PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE metrics_db TO nvme_user;

\l  ### List Databases
\c metrics_db    ### Connect to metrics_db Database
### DROP TABLE nvme_metrics;
CREATE TABLE IF NOT EXISTS nvme_metrics (
    ts TIMESTAMPTZ,
    instance_name VARCHAR(10),
    instance_IP VARCHAR(15),
    t VARCHAR(2),
    sum_bytes INT,
    sum_lat REAL,
    count_t INT    
);

CREATE UNIQUE INDEX idx_nvme_metrics_unique
ON nvme_metrics(ts);

\d ### List Database OBJECTS

metrics_db=# \d
                    List of relations
 Schema |           Name           |   Type   |  Owner   
--------+--------------------------+----------+----------
 public | nvme_metrics        | table    | postgres
 public | nvme_metrics_id_seq | sequence | postgres
(2 rows)

sudo -u postgres psql



GRANT ALL PRIVILEGES ON TABLE nvme_metrics TO nvme_user;
### GRANT ALL PRIVILEGES ON SEQUENCE nvme_metrics_id_seq to nvme_user;


-------------------
{"instance_name":"Server1", "instance_IP":"192.168.122.194","TIMESTAMP":"15:58:05.922","T":"W","sum(BYTES)":81920,"sum(LAT)":9.696000000000002,"count(T)":14}