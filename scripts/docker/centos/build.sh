#!/usr/bin/env sh

# The image name
VAST_IMAGE_REPO=${VAST_IMAGE_REPO:-vast/vast}
# The tag to be used for builder image
VAST_BUILDER_IMAGE_TAG=${VAST_BUILDER_IMAGE_TAG:-build}
# The tag to be used for runner image
VAST_RUNNER_IMAGE_TAG=${VAST_RUNNER_IMAGE_TAG:-latest}

echo Building $VAST_IMAGE_REPO:$VAST_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")
docker build --no-cache -t $VAST_IMAGE_REPO:$VAST_BUILDER_IMAGE_TAG-$(git log -1 --format="%H") . -f scripts/docker/centos/Dockerfile.build

echo Creating $VAST_BUILDER_IMAGE_TAG-$(git log -1 --format="%H"), extracting binary
docker create --name extract $VAST_IMAGE_REPO:$VAST_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")
mkdir scripts/docker/centos/vast
docker cp extract:/build/vast-vast/target/release/vast scripts/docker/centos/vast

echo Building $VAST_IMAGE_REPO:$VAST_RUNNER_IMAGE_TAG
docker build --no-cache -t $VAST_IMAGE_REPO:$VAST_RUNNER_IMAGE_TAG scripts/docker/centos/ -f scripts/docker/centos/Dockerfile

echo Cleaning up ...
rm -rf scripts/docker/centos/vast
docker rm -f extract
docker rmi -f $VAST_IMAGE_REPO:$VAST_BUILDER_IMAGE_TAG-$(git log -1 --format="%H")

echo Echoing Vast version:
docker run $VAST_IMAGE_REPO:$VAST_RUNNER_IMAGE_TAG --version

echo Done.
