#!/bin/sh

if [ "$1" == "" ]; then
    echo "Missing key id"
    exit
fi

mkdir test-store-enc
echo "$1" > test-store-enc/.gpg-id

cd test-store
for file in *
do
  gpg -o "../test-store-enc/$file" --recipient "$1" -e "$file" 
done
