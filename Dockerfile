FROM ubuntu:noble

RUN set -eux; \
	groupadd -g 1005 builder; \
	useradd -g builder -u 1005 -m -s /bin/bash builder

RUN set -eux; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
# for Docker
	gosu curl ca-certificates \
# for makefile
	jq make \
# for rust's libcore
	gcc libc6-dev

RUN set -eux; \
	gosu builder mkdir /home/builder/arduboy-hello-rs; \
	gosu builder mkdir /home/builder/bin

RUN set -eux; \
	gosu builder bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none"; \
	gosu builder bash -c "curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | BINDIR=/home/builder/bin sh"

COPY --chown=builder:builder Cargo.lock Cargo.toml Makefile arduboy-hello-rs.ino hello.rs rust-toolchain.toml /home/builder/arduboy-hello-rs/
COPY --chown=builder:builder .cargo/config.toml /home/builder/arduboy-hello-rs/.cargo/

RUN set -eux; \
	cd /home/builder/arduboy-hello-rs; \
	gosu builder bash -c 'export PATH="/home/builder/bin:/home/builder/.cargo/bin:$PATH" && make PORT=fake setup && make'
