#!/usr/bin/env bash

<<'STEPS'
# on local
scp -P 22 build_on_remote.sh root@1.1.1.1:~/build_on_remote.sh

# on remote
cd ~
./build_on_remote.sh

# on local
./pull_one_local.sh root 1.1.1.1 22
STEPS

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
ssh_username="$1"
ssh_host="$2"
ssh_port="$3"

# 
# v3
# 
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-common-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-common-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-graph-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-graph-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-meta-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-meta-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v3/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v3/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-storage-v3/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v3/src/types.rs \
                                ${script_path_root}nebula-fbthrift-storage-v3/src/types.rs


# 
# v2
# 
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v2/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-common-v2/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v2/src/types.rs \
                                ${script_path_root}nebula-fbthrift-common-v2/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v2/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-graph-v2/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v2/src/types.rs \
                                ${script_path_root}nebula-fbthrift-graph-v2/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v2/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-meta-v2/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v2/src/types.rs \
                                ${script_path_root}nebula-fbthrift-meta-v2/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v2/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v2/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v2/src/types.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v2/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v2/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-storage-v2/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v2/src/types.rs \
                                ${script_path_root}nebula-fbthrift-storage-v2/src/types.rs

# 
# v1
# 
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v1/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-common-v1/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-common-v1/src/types.rs \
                                ${script_path_root}nebula-fbthrift-common-v1/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v1/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-graph-v1/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-graph-v1/src/types.rs \
                                ${script_path_root}nebula-fbthrift-graph-v1/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v1/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-meta-v1/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-meta-v1/src/types.rs \
                                ${script_path_root}nebula-fbthrift-meta-v1/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v1/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v1/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-raftex-v1/src/types.rs \
                                ${script_path_root}nebula-fbthrift-raftex-v1/src/types.rs

scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v1/src/lib.rs \
                                ${script_path_root}nebula-fbthrift-storage-v1/src/lib.rs
scp -P ${ssh_port} ${ssh_username}@${ssh_host}:/tmp/nebula-fbthrift-storage-v1/src/types.rs \
                                ${script_path_root}nebula-fbthrift-storage-v1/src/types.rs
