#!/bin/sh

set -e # fail on any error

VERSION=$(cat ./tools/VERSION)
TRACK=$(cat ./tools/TRACK)
echo "Vast version = ${VERSION}"
echo "Vast track = ${TRACK}"

test "$Docker_Hub_User_Vast" -a "$Docker_Hub_Pass_Vast" \
    || ( echo "no docker credentials provided"; exit 1 )
docker login -u "$Docker_Hub_User_Vast" -p "$Docker_Hub_Pass_Vast"
echo "__________Docker info__________"
docker info

# we stopped pushing nightlies to dockerhub, will push to own registry prb.
case "${SCHEDULE_TAG:-${CI_COMMIT_REF_NAME}}" in
    "$SCHEDULE_TAG")
        echo "Docker TAG - 'vast/vast:${SCHEDULE_TAG}'";
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "vast/vast:${SCHEDULE_TAG}" \
            --file tools/Dockerfile .;
        docker push "vast/vast:${SCHEDULE_TAG}";;
    "stable")
        echo "Docker TAGs - 'vast/vast:${VERSION}-${CI_COMMIT_REF_NAME}', 'vast/vast:stable'";
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "vast/vast:${VERSION}-${CI_COMMIT_REF_NAME}" \
            --tag "vast/vast:latest" \
            --tag "vast/vast:stable" \
            --file tools/Dockerfile .;
        docker push "vast/vast:${VERSION}-${CI_COMMIT_REF_NAME}";
        docker push "vast/vast:stable";
        docker push "vast/vast:latest";;
    v[0-9]*.[0-9]*)
        echo "Docker TAG - 'vast/vast:${VERSION}-${TRACK}'"
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "vast/vast:${VERSION}-${TRACK}" \
            --file tools/Dockerfile .;
        docker push "vast/vast:${VERSION}-${TRACK}";;
    *)
        echo "Docker TAG - 'vast/vast:${VERSION}-${CI_COMMIT_REF_NAME}'"
        docker build --no-cache \
            --build-arg VCS_REF="${CI_COMMIT_SHA}" \
            --build-arg BUILD_DATE="$(date -u '+%Y-%m-%dT%H:%M:%SZ')" \
            --tag "vast/vast:${VERSION}-${CI_COMMIT_REF_NAME}" \
            --file tools/Dockerfile .;
        docker push "vast/vast:${VERSION}-${CI_COMMIT_REF_NAME}";;
esac

docker logout
