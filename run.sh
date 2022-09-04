#!/bin/bash

set -eu

SINGLE_MAX_ID=25
TWIN_MAX_ID=35

cargo build --release --bin solver
cp target/release/solver ./solver_bin

# single experiment
dataset="`seq 1 $SINGLE_MAX_ID` `seq 36 40`"
echo $dataset
parallel --progress --result result ./solver_bin -i {} -s 6 ::: $dataset &

# twin experiment
TWIN_START_ID=`expr ${SINGLE_MAX_ID} + 1`
parallel --progress --result result ./solver_bin -i {} -s 5 -t ::: `seq ${TWIN_START_ID} ${TWIN_MAX_ID}` &

wait

for i in `seq 1 40` ; do
    cp result/1/${i}/stdout solution/${i}.txt
done

rm -r result
rm ./solver_bin
