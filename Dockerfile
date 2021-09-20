# select a starting image to build off
FROM rust as build

# set our working directory in the container as /repo
WORKDIR /repo

# copy all our files across from our local repo to the /repo directory in the container
COPY Cargo.lock .
COPY Cargo.toml .

# cache dependencies by creating an empty
# lib.rs file and building the project
RUN mkdir src
RUN echo "// empty file" > src/lib.rs
RUN cargo build --release

# now copy the code over
COPY src src

# build the release
RUN cargo install --offline --path .

# use a node image for building the site
FROM node:16 as static

WORKDIR /svelte

COPY ./svelte .

RUN yarn install && yarn build

# use a slim image for actually running the container.
FROM rust:slim

WORKDIR /app

# allow requests to port 80
EXPOSE 80

# install the program onto the current image
COPY --from=build /usr/local/cargo/bin/aws-rust-api /usr/local/bin/aws-rust-api
COPY --from=static /svelte/build/ ./static

# copy config file
COPY Rocket.toml .

# this command is run when we actually start the container
CMD ["aws-rust-api"]
