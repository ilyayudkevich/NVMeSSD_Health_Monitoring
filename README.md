This project is illustrative to demonstrate how Health of NVMe SSD Drives
Could be monitored on Network (either on Cloud or Data Center).
In this project it was used monitoring such Characteristic as Latency
of Write/Read Operations. After gathering such information during Time,
one can judge about Health of MVMe SSD Drive by comparison Latency overtime
with itself or with the same characteristics related to another NVMe SSD Drives.

Simulating Environment.
1) Data Center simulated on HOST laptop running Ubuntu 24.04
2) Two VMs created using Multipass
3) Metrics on VMs have being gathered by running utility biosnoop from the bpfcc-tools
   They are written to LOG file and ingested in DuckDB, then Aggregating Characterics 
   received and written to RESULT file in JSON format. 
4) That file pulled from Data Center, ingested into PostgreSQL DB with Inserter created in Rust.
5) Running Rust Axum Web Server on Data Center connected to PostgreSQL DB let 
   Remote User retrieve data for further Analysis and possible Actions and show that as static HTML page.

Detailed Information about Setting Environment one can find in following supplied documents:
   Setting_Environment.md


For Illustrative purpose there used following simplifications:
a) Time Period for aggregation selected equal to 30 sec.
b) Polling VMs from DC is doing every 5 sec and pulling Metrics if new JSON File is ready

Scaling this approach can be done, for example, by reducing aggregation interval, using Kafka broker on DC,
sending information to broker from Server (VM) using kafka utility kcat,
reading kafka messages using Rust Kafka Consumer and adding new Record to PostgreSQL DB. 

### Running Test ###
    ##- Terminals 1 and 2
    1. Activate virtual environments on VMs
       source env/bin/activate
    2. Run Python script
       python3 gather_metrics_VM.py
    ##- Terminal 3
    3. Run Python script
       python3 inserter.py
    ##- Terminal 4
    4. Run Rust web application(binary or cargo run and find HTML page http:localhost:3000 or in public folder)
        

  
