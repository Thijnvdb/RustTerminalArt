#!/bin/sh
if [ ! -d "/home/$USER/bin" ] 
then
    echo "Directory ~/bin does not exists."
    exit -1
else
    cargo build --release
    cp ./target/release/termart ~/bin/termart
    echo "updated termart binary!"
    exit 0
fi