###### Install Multipass ######
snap install multipass

###### Installation libvirt library to substitute NVMe driver and have close functionality ######
### Because multipass does not provide installation NVMe driver ###
### To get close functionality there were taken steps described in Articles ###
(https://documentation.ubuntu.com/server/how-to/virtualisation/libvirt/)
https://documentation.ubuntu.com/multipass/en/latest/how-to-guides/customise-multipass/set-up-the-driver/)

sudo apt install cpu-checker
-------------------------
kvm-ok
INFO: /dev/kvm exists
KVM acceleration can be used
----------------------------
sudo apt update
sudo apt install qemu-kvm libvirt-daemon-system

sudo adduser $USER libvirt

sudo systemctl enable libvirtd
sudo systemctl status libvirtd

#############################
multipass set local.passphrase

multipass authenticate

multipass set local.driver=libvirt

snap connect multipass:libvirt

###### HOST machine Installation #########
1) Install the bpfcc-tools package  
sudo apt install bpfcc-tools
  1a) find binary biosnoop (whereis) and copy to working local folder
  1b) Transfer binary to VMs
      multipass transfer biosnoop Server1:.
      multipass transfer biosnoop Server2:.

2) Install Rust environment
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  2a) install `sd` binary
      cargo install sd
  2b) Copy binary from $HOME/.cargo/bin to working folder and transfer it to VMs
      multipass transfer sd Server1:.
      multipass transfer sd Server2:.

3) Installing PostgreSQL
      sudo apt update
      sudo apt install postgresql postgresql-contrib

    sudo -i -u postgres

4) Create Python script to automate metrics gathering on VM
   4) Transfer it to VMs
      multipass transfer gather_metrics.py Server1:.
      multipass transfer gather_metrics.py Server2:.

5) Mount working directory on HOST to working directory on Instance
      multipass mount /home/ilya/Documents/MyAmazon/NVMeSSD_Health_Monitoring/ Server1
      (https://documentation.ubuntu.com/multipass/en/latest/how-to-guides/manage-instances/share-data-with-an-instance/)

6) Create Rust Application that:
   a) adds info two records from record_W.json and record_R.json (PG_Rust_Client)
   b) can retrieve Records from PostgreSQL