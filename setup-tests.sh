#!/bin/bash

if [ "$1" == "" ]; then
    echo "Missing key id"
    exit
fi

KEY=$1

mkdir -p test-store-enc
echo "$1" > test-store-enc/.gpg-id

function encrypt {
    for file in $1/*
    do
        if [ -d $file ] 
        then
            mkdir -p "../test-store-enc/$file"
            encrypt $file
        else
            gpg --yes --batch -o "../test-store-enc/$file" --recipient "$KEY" -e "$file" 
        fi
    done
}

cd test-store
encrypt .
