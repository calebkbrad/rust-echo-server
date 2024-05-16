FROM rust:1.67

COPY ./echo_server ./

RUN cargo build --release

CMD ["./target/release/echo_server"]