#!/bin/bash

set -eu

SINGLE_MAX_ID=25
TWIN_MAX_ID=35

cargo build --release --bin solver
cp target/release/solver ./solver_bin

# single experiment
# parallel --progress --result result ./solver_bin -i {} -s 2 ::: `seq 1 ${SINGLE_MAX_ID}`
# for i in `seq 1 ${MAX_ID}`; do
#     cp result/1/${i}/stdout solution/${i}.txt
# done

# twin experiment
TWIN_START_ID=`expr ${SINGLE_MAX_ID} + 1`
parallel --progress --result result ./solver_bin -i {} -s 3 -t ::: `seq ${TWIN_START_ID} ${TWIN_MAX_ID}`
for i in `seq ${TWIN_START_ID} ${TWIN_MAX_ID}`; do
    cp result/1/${i}/stdout solution/${i}.txt
done

rm -r result
rm ./solver_bin
