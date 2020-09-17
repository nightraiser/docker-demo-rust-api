FROM rustlang/rust:nightly

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
EXPOSE 8000
CMD [ "cargo", "run" ]