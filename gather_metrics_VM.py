import subprocess
import os, re, time
import datetime
from datetime import datetime
from datetime import timedelta
import os.path
import subprocess
from subprocess import run
from io import StringIO
from subprocess import Popen, PIPE
import duckdb
from datetime import datetime, timezone
import time

LOG_NUM = 0
LOG_FILENAME = "LOG" + str(LOG_NUM) + ".csv"
one_s = timedelta(seconds=1)
i = 0
while True:
    time.sleep(5)  

    output = ""
    try:
        cmd = f"""sudo  '/home/ubuntu/biosnoop' -Q -t '30'"""
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        print("Return Code:", result.returncode)
        print("Standard Output:", result.stdout)
        output = result.stdout
    except subprocess.CalledProcessError as e:  
        print(f"Command failed with error: {e}")
        print("Standard Error:", result.stderr)


    utc_time = datetime.now(timezone.utc)
    utc_time_R = utc_time + one_s
    utc_time_W = utc_time   
 
    with open(LOG_FILENAME,'w') as f_obj:
        f_obj.write(output)

    print("hello")

### convert LOG file format to CSV ###
#---     sd "[' ']+|\t" ',' nvmedata.csv

    try:
        cmd = f"""'/home/ubuntu/sd' "[' ']+|\\t" ',' {LOG_FILENAME}"""
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        cmd = f"""sudo  '/home/ubuntu/biosnoop' -Q -t '30'"""
        result = subprocess.run(cmd, shell=True, capture_output=
    except subprocess.CalledProcessError as e:
        print(f"Command failed with error: {e}")
        print("Stderr:", e.stderr)

### Ingest CSV Data INTO Metrics TABLE of DuckDB database ###

    csv_filename = LOG_FILENAME

### IP for Server1:  '192.168.122.194'
### IP for Server2:  '192.168.122.18'
    with duckdb.connect("nvme.db") as con:
        con.sql("CREATE OR REPLACE TABLE machine_signature (instance_name varchar,instance_ip varchar);")
        con.sql("INSERT INTO machine_signature VALUES('Server1','192.168.122.194');")
#        con.sql("INSERT INTO machine_signature VALUES('Server2','92.168.122.18');")  ### Server2 IP Address
        con.sql("CREATE OR REPLACE TABLE metrics (ts time, COMM varchar, PID int64, DISK varchar, t varchar, SECTOR int64, BYTES int64, QUE double, LAT double)")
        con.sql("COPY metrics FROM 'LOG0.csv'(FORMAT CSV, strict_mode false);")
 #   con.table("metrics").show(), sum_bytes, sum_lat, count_t

        con.sql("CREATE OR REPLACE TABLE output_W AS SELECT t, SUM(BYTES) AS sum_bytes, SUM(LAT) as sum_lat, COUNT(t) AS count_t FROM metrics WHERE t='W' GROUP by t")
        con.sql("CREATE OR REPLACE TABLE output_R AS SELECT t, SUM(BYTES) AS sum_bytes, SUM(LAT) as sum_lat, COUNT(t) AS count_t FROM metrics WHERE t='R' GROUP by t")
 
        con.sql("CREATE OR REPLACE TABLE timestamp_W (ts TIMESTAMPTZ)")
        con.sql("CREATE OR REPLACE TABLE timestamp_R (ts TIMESTAMPTZ)")

        con.execute("INSERT INTO timestamp_W VALUES (?)", [utc_time_W])
        con.execute("INSERT INTO timestamp_R VALUES (?)", [utc_time_R])

        con.sql("CREATE OR REPLACE TABLE result_W AS SELECT * FROM timestamp_W,machine_signature,output_W")
        con.sql("CREATE OR REPLACE TABLE result_R AS SELECT * FROM timestamp_R,machine_signature,output_R")
        con.execute("COPY result_W TO 'mytest1/result_W_Server1.json' (ARRAY)")
        con.execute("COPY result_R TO 'mytest1/result_R_Server1.json' (ARRAY)")
#        con.execute("COPY result_W TO 'mytest2/result_W_Server2.json' (ARRAY)") ### Name of Second Instance
#        con.execute("COPY result_R TO 'mytest2/result_R_Server2.json' (ARRAY)")

    i += 1
    if i > 10:
        break




 
