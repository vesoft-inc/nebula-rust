# How to update the files under src/interface

## Firstly: Generate the files

under Linux

```
wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/third-party/install-third-party.sh
wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/third-party/cxx-compiler-abi-version.sh
chmod 777 install-third-party.sh
chmod 777 cxx-compiler-abi-version.sh
bash install-third-party.sh --prefix=/home/user/third-party
wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/src/common/interface/common.thrift
wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/src/common/interface/graph.thrift
wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/src/common/interface/meta.thrift
wget https://raw.githubusercontent.com/vesoft-inc/nebula-common/master/src/common/interface/storage.thrift
/home/user/third-party/bin/thrift1 --strict --allow-neg-enum-vals --gen "mstch_rust" -o . common.thrift
/home/user/third-party/bin/thrift1 --strict --allow-neg-enum-vals --gen "mstch_rust" -o . graph.thrift
/home/user/third-party/bin/thrift1 --strict --allow-neg-enum-vals --gen "mstch_rust" -o . meta.thrift
/home/user/third-party/bin/thrift1 --strict --allow-neg-enum-vals --gen "mstch_rust" -o . storage.thrift
```

## Secondly manually modify the recursive type error

1. Change the fields `Vertex`, `Edge`, `Path`, `NList`, `NMap`, `NSet` and `DataSet` of `Value` to boxed. See the detail issue https://github.com/facebook/fbthrift/issues/438 .

2. Implement Ord for `Value` type. It's required by the `BTreeSet`.
