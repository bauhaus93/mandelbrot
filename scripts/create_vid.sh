#!/bin/sh
rm -f output.mp4
ffmpeg -i seq_%6d.png -pix_fmt yuv420p output.mp4 &&\
    vlc output.mp4
