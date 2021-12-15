#!/bin/bash
set -eu

if [ "$#" != 1 ] ; then
    echo "The build folder has to be given"
    exit 1
fi

for d in */ ; do
    DIR=${d::-1}

    if [ "$DIR" = "$1" ] ; then
        cd "$DIR"

        rustc "main.rs"
        ./main
    fi

done
