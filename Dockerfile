# select a starting image to build off
FROM rust as build

# set our working directory in the container as /repo
WORKDIR /repo

# copy all our files across from our local repo to the /repo directory in the container
COPY Cargo.lock .
COPY Cargo.toml .
COPY src src

# build the release
RUN cargo install --path .

# allow requests to port 80
EXPOSE 80

COPY Rocket.toml . 

# this command is run when we actually start the container
CMD ["aws-rust-api"]
