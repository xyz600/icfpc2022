#!/bin/bash

set -eu

MAX_ID=25

cargo build --release --bin solver
cp target/release/solver ./solver_bin

parallel --progress --result result ./solver_bin -i {} ::: `seq 1 ${MAX_ID}`

for i in `seq 1 ${MAX_ID}`; do
    cp result/1/${i}/stdout solution/${i}.txt
done

rm -r result
rm ./solver_bin
