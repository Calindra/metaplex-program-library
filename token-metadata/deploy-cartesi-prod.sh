
cd ../../rollups-examples
export ROLLUPS_EXAMPLES=`pwd`
cd -

cd ../../cartesi-solana
export CARTESI_SOLANA=`pwd`
cd -

cd ../core/rust/utils
export MPL_UTILS=`pwd`
cd -

docker run \
    -v `pwd`:/workdir \
    -v $ROLLUPS_EXAMPLES:/rollups-examples \
    -v $CARTESI_SOLANA:/cartesi-solana \
    -v $MPL_UTILS:/core/rust/utils \
    -w /workdir \
    --rm \
    cartesi/toolchain:0.11.0 \
    ./build-prod.sh


