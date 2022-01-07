#!/bin/bash

set -ex

mkdir -p bin/

clang -o bin/lib.o -c lib/*.c

clang -o bin/host -Wl,-export_dynamic host/*.c bin/lib.o

rm bin/lib.o

clang -o bin/plugin.dylib -shared -undefined dynamic_lookup plugin/*.c