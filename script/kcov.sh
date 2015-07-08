set -e

PROJECT=test
TARGET=$HOME/kcov

wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
mkdir kcov-master/build
cd kcov-master/build
cmake .. -DCMAKE_INSTALL_PREFIX=$TARGET
make
make install
cd ../..
export PATH=$TARGET/bin:$PATH

cargo rustc -- --test -C 'link-args=-Wl,--no-gc-sections'

kcov --coveralls-id=$TRAVIS_JOB_ID --exclude-pattern=/.cargo target/kcov target/debug/$PROJECT
