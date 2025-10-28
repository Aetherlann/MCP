#!/bin/bash
# Send an image using Kitty graphics protocol
# Usage: ./send-kitty-image.sh <image-file>

if [ $# -eq 0 ]; then
    echo "Usage: $0 <image-file>"
    exit 1
fi

IMAGE_FILE="$1"

if [ ! -f "$IMAGE_FILE" ]; then
    echo "Error: File '$IMAGE_FILE' not found"
    exit 1
fi

# Read image and encode to base64
IMAGE_DATA=$(base64 < "$IMAGE_FILE" | tr -d '\n')

# Send Kitty graphics protocol
# Format: ESC_Gf=100,a=T,t=d;<base64>ESC\
# f=100 = PNG format
# a=T = transmit and display
# t=d = direct transmission

printf '\e_Gf=100,a=T,t=d;%s\e\\' "$IMAGE_DATA"
echo ""
