#! /usr/bin/env bash

set -e

NEBULA_THIRDPARYTY_HOME=

# Parsing options from arguments
while getopts "t:" opt; do
    case $opt in
        t)
            NEBULA_THIRDPARYTY_HOME=${OPTARG}
            ;;
        \?)
            echo "Invalid option: -${OPTARG}" >&2
            exit 1
            ;;
        :)
            echo "Option -${OPTARG} requires an argument." >&2
            exit 1
            ;;
    esac
done

for mod in common meta storage graph; do
    wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/src/common/interface/$mod.thrift
done

for mod in common meta storage graph; do
    $NEBULA_THIRDPARYTY_HOME/2.0/bin/thrift1 --strict --allow-neg-enum-vals --gen "mstch_rust" -o ./src/interface/$mod/ $mod.thrift
    mv ./src/interface/$mod/gen-rust/lib.rs ./src/interface/$mod
    rmdir ./src/interface/$mod/gen-rust
done

rm common.thrift graph.thrift meta.thrift storage.thrift
