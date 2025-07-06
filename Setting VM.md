###### Createing an VM #######
multipass launch noble --cpus 4 --disk 20G --memory 8G --name Server1  ### second Instance with name Server2

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
7) Install bcc-tools libraries:
       sudo apt install bpfcc-tools
       