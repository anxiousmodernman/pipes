#!/bin/bash


function sigint_handler {
   echo "my last message: goodbye."
   sleep 1
   exit 13
}

trap sigint_handler INT

while true; do
    # print the date, sleep for a second, repeat
    date
    sleep 1
done
