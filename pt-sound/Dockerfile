# Use phusion/baseimage as base image. To make your builds
# reproducible, make sure you lock down to a specific version, not
# to `latest`! See
# https://github.com/phusion/baseimage-docker/blob/master/Changelog.md
# for a list of version numbers.
FROM phusion/baseimage:0.11
# FROM ubuntu:18.04

# Use baseimage-docker's init system.
CMD ["/sbin/my_init"]

WORKDIR /usr/local/

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y \
    # Install development tools
    sudo wget python3 git vim tmux \
    # Install faust dependencies
    pkg-config build-essential cmake libmicrohttpd-dev \
    # Install sound libraries
    libasound2-dev

# Install LLVM
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
RUN sudo apt-add-repository "deb https://apt.llvm.org/xenial/ llvm-toolchain-xenial-3.8 main"
RUN sudo apt-get update
RUN sudo apt-get install -y clang-3.8 lldb-3.8

RUN mkdir faust
WORKDIR /usr/local/faust/

# Install faust
RUN git clone https://github.com/grame-cncm/faustinstaller.git .
RUN echo "2\n" > ./install.docker.sh
RUN ./install.compiler.sh

WORKDIR /usr/local/pt-sound/
#RUN faust2api -alsa midisynth.dsp

# Clean up APT when done.
RUN apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
