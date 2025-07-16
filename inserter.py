import subprocess
import os, re, time
from datetime import datetime

output = ""
cwd = os.getcwd()
binary = os.path.join(cwd, 'pg_rust_data_inserter')
file1 = os.path.join(cwd, 'mytest1/result_R_Server1.json')
file2 = os.path.join(cwd, 'mytest2/result_R_Server2.json')
file3 = os.path.join(cwd, 'mytest1/result_W_Server1.json')
file4 = os.path.join(cwd, 'mytest2/result_W_Server2.json')

def run_rust_binary(binary):
    try:
        cmd = f"""{binary}"""
        print("cmd: ", {cmd})
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        print("Return Code:", result.returncode)
        print("Standard Output:", result.stdout)
        output = result.stdout
        # Get the output of the final command

    except subprocess.CalledProcessError as e:
        print(f"Command failed with error: {e}")
        print("Standard Error:", result.stderr)

i = 0

while True:
    time.sleep(10)
    if os.path.exists(file1) or os.path.exists(file2) or os.path.exists(file3) or os.path.exists(file4):
        print("Files exist")
        run_rust_binary(binary)
    else:
         print("Files do not exist")

    i += 1
    print("i: ",i)
 #   if i > 70:
    if i > 32:
        break
"""
        time.sleep(2) 
    if os.path.exists(file1):
        os.remove(file1)
    if os.path.exists(file2):
        os.remove(file2)
    if os.path.exists(file3):
        os.remove(file3)
    if os.path.exists(file4):
        os.remove(file4)
"""
 
    