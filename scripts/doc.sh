#!/usr/bin/env sh
# generate documentation only for partiy and vastcore libraries

cargo doc --no-deps --verbose --all --exclude vast-ipfs-api &&
	echo '<meta http-equiv=refresh content=0;url=vastcore/index.html>' > target/doc/index.html
