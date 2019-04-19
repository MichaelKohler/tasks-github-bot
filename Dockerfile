FROM rust as build

RUN mkdir -p tasks_github_bot
WORKDIR /tasks_github_bot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN ls -al .

RUN cargo build --release


FROM rust

COPY --from=build /tasks_github_bot/target/release/tasks_github_bot .

CMD ["./tasks_github_bot"]
