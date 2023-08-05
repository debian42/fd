FROM amd64/rust
ENV TZ=Europe/Berlin
WORKDIR /work
ADD . .
USER root
RUN cp /work/entrypoint.sh /entrypoint.sh
ENTRYPOINT [ "/entrypoint.sh" ]
RUN apt-get -y update && apt-get -y upgrade && apt-get -y install musl-tools && apt-get -y install musl-dev && apt-get -y install gcc && chmod ugo+x /entrypoint.sh
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo update && cargo test && cargo build --target=x86_64-unknown-linux-musl --release
RUN cargo install cargo-pgo
RUN rustup component add llvm-tools-preview
RUN cargo pgo build -- --target=x86_64-unknown-linux-musl
RUN cargo pgo test -- --target=x86_64-unknown-linux-musl
RUN /work/target/x86_64-unknown-linux-musl/release/fd -s"1.1.19 0:0:0" /work/misc/server-local.log
RUN cargo pgo optimize build target=x86_64-unknown-linux-musl ; exit 0
RUN cargo pgo optimize build -- --target=x86_64-unknown-linux-musl ; exit 0
RUN cargo pgo optimize run -- --target=x86_64-unknown-linux-musl ; exit 0
RUN cargo pgo optimize ; exit 0
RUN pwd && ls -ltraR && find / -name fd*
RUN ldd /work/target/x86_64-unknown-linux-musl/release/fd
RUN ldd /work/target/x86_64-unknown-linux-gnu/release/fd
RUN cat /etc/*-release 