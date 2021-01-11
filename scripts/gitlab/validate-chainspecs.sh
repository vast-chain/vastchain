#!/bin/bash
set -e # fail on any error
set -u # treat unset variables as error
echo "________Running validate_chainspecs.sh________"

ERR=0

echo "________Validate chainspecs________"
time cargo build --release -p chainspec --verbose --color=always

for spec in vastcore/res/*.json; do
    if ! ./target/release/chainspec "$spec"; then ERR=1; fi
done

for spec in vastcore/res/vast/*.json; do
    if ! ./target/release/chainspec "$spec"; then ERR=1; fi
done

echo "________Mainnet contains Istanbul EIPs________"
for eip in $(grep --only-matching "eip.*Transition" vastcore/res/vast/istanbul_test.json); do
    if ! grep -q $eip vastcore/res/vast/foundation.json; then
        echo "ERROR: $eip is missing in the foundation json spec"
        ERR=1
    fi
done

#show sccache statistics
sccache --show-stats
exit $ERR
