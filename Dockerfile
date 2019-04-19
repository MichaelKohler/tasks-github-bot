FROM rust as build

COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/tasks_github_bot /build-out/


FROM alpine:latest

COPY --from=build /build-out/tasks_github_bot /

CMD /tasks_github_bot