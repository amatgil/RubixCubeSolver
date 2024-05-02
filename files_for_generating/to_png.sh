#!/bin/sh

for f in ./images/* 
do
    #OUTPUT_NAME="${f%.*}.png"
    OUTPUT_NAME=$(basename $f ".svg")
    echo "Turning $f into pngs/$OUTPUT_NAME.png"
    convert -size 1024x1024 "$f" "pngs/$OUTPUT_NAME.png"
done
