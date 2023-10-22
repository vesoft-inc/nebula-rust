#!/usr/bin/env bash

<<'EG'
./cargo_publish.sh v3
./cargo_publish.sh v2
./cargo_publish.sh v1

./cargo_publish.sh collect
EG

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
type="$1"

# 
case $type in
  "v3")
    cd ${script_path_root}nebula-fbthrift-common-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-graph-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-meta-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-raftex-v3
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-storage-v3
    cargo publish -v
    sleep 2

    ;;
  "v2")
    cd ${script_path_root}nebula-fbthrift-common-v2
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-graph-v2
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-meta-v2
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-raftex-v2
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-storage-v2
    cargo publish -v
    sleep 2

    ;;
  "v1")
    cd ${script_path_root}nebula-fbthrift-common-v1
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-graph-v1
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-meta-v1
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-raftex-v1
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-storage-v1
    cargo publish -v
    sleep 2

    ;;
  "collect")
    cd ${script_path_root}nebula-fbthrift-common
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-graph
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-meta
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-raftex
    cargo publish -v
    sleep 2

    cd ${script_path_root}nebula-fbthrift-storage
    cargo publish -v
    sleep 2

    ;;
esac
