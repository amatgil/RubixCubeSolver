#!/bin/sh

ROOT_DIR="/tmp/tubaitu_svgs"
SVG_DIR="$ROOT_DIR/svgs"
IMG_DIR="$ROOT_DIR/images"

mkdir -p $IMG_DIR

for f in $SVG_DIR/* 
do
    OUTPUT_NAME=$(basename $f ".svg")
    echo -ne "\rTurning $f into $IMG_DIR/$OUTPUT_NAME.png"
    convert -size 1024x1024 "$f" "$IMG_DIR/$OUTPUT_NAME.png" &
done
