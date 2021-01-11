FROM ubuntu:xenial

# metadata
ARG VCS_REF
ARG BUILD_DATE

LABEL io.vast.image.authors="devops-team@vast.io" \
	io.vast.image.vendor="Vast Technologies" \
	io.vast.image.title="vast/vast" \
	io.vast.image.description="Vast. The Fastest and most Advanced Vast Client." \
	io.vast.image.source="https://github.com/vasttech/vast-vast/blob/${VCS_REF}/\
scripts/docker/hub/Dockerfile" \
	io.vast.image.documentation="https://wiki.vast.io/Vast-Vast" \
	io.vast.image.revision="${VCS_REF}" \
	io.vast.image.created="${BUILD_DATE}"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		file curl jq; \
# apt cleanup
	apt-get autoremove -y; \
	apt-get clean; \
	rm -rf /tmp/* /var/tmp/* /var/lib/apt/lists/*; \
# add user
	groupadd -g 1000 vast; \
	useradd -m -u 1000 -g vast -s /bin/sh vast

WORKDIR /home/vast

# add vast-vast binary to docker image
COPY artifacts/x86_64-unknown-linux-gnu/vast /bin/vast
COPY tools/check_sync.sh /check_sync.sh

# switch to user vast here
USER vast

# check if executable works in this container
RUN vast --version

EXPOSE 5001 8080 8082 8083 8545 8546 8180 30303/tcp 30303/udp

ENTRYPOINT ["/bin/vast"]
