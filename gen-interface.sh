#! /usr/bin/env bash

set -e

NEBULA_THIRDPARYTY_HOME=

NEBULA_REPO_VERSION=master

# Parsing options from arguments
while getopts "t:v:" opt; do
    case $opt in
        t)
            NEBULA_THIRDPARYTY_HOME=${OPTARG}
            ;;
        v)
            NEBULA_REPO_VERSION=${OPTARG}
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

echo 'Download from version '${NEBULA_REPO_VERSION}

for mod in common meta storage graph; do
    wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/${NEBULA_REPO_VERSION}/src/common/interface/$mod.thrift
done

for mod in common meta storage graph; do
    $NEBULA_THIRDPARYTY_HOME/2.0/bin/thrift1 --strict --allow-neg-enum-vals --gen "mstch_rust" -o ./interface/$mod/ $mod.thrift
    mv ./interface/$mod/gen-rust/lib.rs ./interface/$mod/src
    rmdir ./interface/$mod/gen-rust
done

rm common.thrift graph.thrift meta.thrift storage.thrift
