FROM rust:1.89.0

RUN apt update
RUN apt install -y cmake libxrandr-dev libopengl-dev libxinerama-dev libxcursor-dev libxi-dev
