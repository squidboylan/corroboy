#!/bin/bash

for i in `seq 0 9` A B C D E F ; do
    for j in `seq 0 9` A B C D E F ; do
        res="$(grep "0x${i}${j} =>" src/gameboy/cpu/mod.rs)"
        if [ -z "$res" ] ; then
            echo "0x${i}${j}"
        fi
    done
done

