#!/bin/sh

script_dir="$(dirname "$0")"

mkdir -p "$script_dir/_build"
cd "$script_dir/_build"

cc -g -fPIC -I/usr/lib/erlang/usr/include/ -shared -o libhello.so ../hello.c
