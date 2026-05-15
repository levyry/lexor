#!/bin/bash

#while true; do 
#  # A netcat kimenetét egy belső blokknak adjuk át
#  nc -l -p 1500 | while read line; do
#    echo "got: $line"
#    if [[ "$line" == *"/stop"* ]]; then exit; fi
#    npx ski eval $line
#    printf "trallala"
#  done
#done

read line

echo "Beérkező kérés: $line" >&2
res=$(npx ski eval $line)
echo -e "$res"
