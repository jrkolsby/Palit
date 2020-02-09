#!/bin/sh

faust -lang c -cn mydsp $1 > plugin_part.c;
cat faust.h plugin_part.c > plugin.c;
gcc -c -fpic plugin.c;
gcc -shared -o plugin.so plugin.o;
rm plugin_part.c;
rm plugin.c;
rm plugin.o;
