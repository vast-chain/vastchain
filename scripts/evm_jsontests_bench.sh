#!/usr/bin/env bash

cargo build --release -p evmbin

./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmArithmeticTest
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmBitwiseLogicOperation
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmBlockInfoTest
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmEnvironmentalInfo
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmIOandFlowOperations
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmLogTest
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmPerformance
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmPushDupSwapTest
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmRandomTest
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmSha3Test
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmSystemOperations
./target/release/vast-evm stats-jsontests-vm ./vastcore/res/vast/tests/VMTests/vmTests
