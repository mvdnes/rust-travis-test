#!/bin/bash

set -e
shopt -s nullglob

PROJECT=mytest
TARGET=$HOME/kcov
KCOV_RELEASE=v27

wget https://github.com/SimonKagstrom/kcov/archive/$KCOV_RELEASE.tar.gz
tar xzf $KCOV_RELEASE.tar.gz
cd kcov-*
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=$TARGET
make
make install
cd ../..
export PATH=$TARGET/bin:$PATH

#actual coverage testing:

cargo rustc -- --test -C 'link-args=-Wl,--no-gc-sections' -g
kcov --exclude-pattern=/.cargo target/kcov-lib target/debug/$PROJECT

for d in tests/*; do
    basename=$(basename $d);
    testname=${basename%.*};

    cargo rustc --test $testname -- -C 'link-args=-Wl,--no-gc-sections' -g
    kcov --exclude-pattern=/.cargo target/kcov-$testname target/debug/$testname-*
done

kcov --merge --coveralls-id=$TRAVIS_JOB_ID target/kcov target/kcov-*
