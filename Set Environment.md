### INSTALLATION ON THE HOST ###
1. sudo snap install multipass
2. multipass launch 24.04 --name Server1  (Server2)
3. multipass set local.passphrase=your_secure_passphrase
4. multipass auth secure_passphrase
### OPEN TWO TERMINALS ON HOST and COONECT TO INSTANCES ###
5. multipass shell Server1
6. multipass shell Server2
### CREATE SHARED FOLDER mytest on both Instances ###
mkdir mytest1  ### on Server1
mkdir mytest2  ### on Server2
### CREATE SHARED FOLDERS mytest1 and mytest2 on HOST ###
### MAP SHARED folders from HOST ###
multipass mount /path/to/my_shared_folder/mytest1 Server1:/home/ubuntu/mytest1
multipass mount /path/to/my_shared_folder/mutest2 Server2:/home/ubuntu/mytest2

### Install Rust on HOST ### Open Third Terminal
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
-----------
# install Rust binaries #
cargo install sd
cargo install rg
-----------
# find `sd`` in folder $HOME/.cargo/bin #
# copy it to folder /path/to/my_shared_folder #
# Then copy to Instances #
multipass transfer sd Server1:.
multipass transfer sd Server2:.

### Install the bpfcc-tools package on HOST ###
 sudo apt install bpfcc-tools
1a) find binary biosnoop (whereis) and copy to /path/to/my_shared_folder
1b) Transfer binary to Instances
    multipass transfer biosnoop Server1:.
    multipass transfer biosnoop Server2:.
### Install Postgresql database on HOST ###
    sudo apt update
    sudo apt install postgresql postgresql-contrib
# Open Forth Terminal and open PG shell #
    sudo -u postgres psql
# Set password for user postgres #
    ALTER USER postgres PASSWORD 'postgres';
### Copy gather_metrics_VM.py to Instances ###
    multipass transfer gather_metrics_VM.py Server1:.   (Server2:.) ### Comment and Uncomment appropriate
                                                                    ### lines on Server2
### Create Rust Application pg_rust_data_inserter that reads JSON files received on Instances ###
### and Insert Row Into Database ###
    cargo new pg_rust_data_inserter
# Replace content of src/main.rs and Cargo.toml files with provided from Github #
    cargo build
# Copy binary file pg_rust_data_inserter/target/debug folder to /path/to/my_shared_folder #
### Copy to the same folder Python file inserter.py ###
### Create Rust Application axum-pg-tower for remote connection to Postgresql Database and output satic HTML page to Web Browser ###
    cargo new  axum-pg-tower
# Replace content of src/main.rs and Cargo.toml files with provided from Github #
    cargo build
# Copy binary file pg_rust_data_inserter/target/debug folder to /path/to/my_shared_folder or any other convinient place #

### INSTALLATION ON THE VM ###
###### Setup VM Environment ######
1) Connect to VM from HOST
      multipass shell Server1
2) Update & Upgrade
    sudo apt update && sudo apt upgrade -y
3) Install DuckDB on VM machine:
    sudo snap install duckdb
4) Install pip
    sudo apt install python3-pip  
5) Install Python venv
    sudo apt update && sudo apt upgrade 
    sudo apt install python3.12-venv 
6) Create virtual environment
    python3 -m venv env
    6a) Activate virtual environment
       source env/bin/activate
    6b) Install Python API for DuckDB
       pip install duckdb



