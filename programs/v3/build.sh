#!/bin/bash
# USAGE: build.sh [project]

all=(mul test add_test hello_world echo)

if [[ -z $1 ]] || [[ "$1" == "all" ]] ; then
  for proj in ${all[@]} ; do
    mccasm $proj.asm -o bin/$proj.img.bin
  done
  exit 0
fi

mccasm $1.asm -o bin/$1.img.bin
