## Usage

```docker build -f docker/ubuntu/Dockerfile --tag vastcore/vast:branch_or_tag_name .```

## Usage - CentOS

Builds a lightweight non-root Vast docker image:
```
git clone https://github.com/vasttech/vast-vast.git
cd vast-vast
./scripts/docker/centos/build.sh
```

Fully customised build:
```
VAST_IMAGE_REPO=my-personal/vast \
VAST_BUILDER_IMAGE_TAG=build-latest \
VAST_RUNNER_IMAGE_TAG=centos-vast-experimental \
./scripts/docker/centos/build.sh
```

Default values:
```
# The image name
VAST_IMAGE_REPO - vast/vast

# The tag to be used for builder image, git commit sha will be appended
VAST_BUILDER_IMAGE_TAG - build

# The tag to be used for runner image
VAST_RUNNER_IMAGE_TAG - latest
```

All default ports you might use will be exposed:
```
#           secret
#      ipfs store     ui   rpc  ws   listener  discovery
#      ↓    ↓         ↓    ↓    ↓    ↓         ↓
EXPOSE 5001 8082 8083 8180 8545 8546 30303/tcp 30303/udp
```
