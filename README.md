# error-server
Log errors in production to a proper DB for easy debugging!

## Cargo config

See the comments in `.cargo/config.yml` and install the required tools for your platform

## Pre-commit hooks

You will need to install `lefthook` tool and generate the Git hooks:

- Follow the [installation instructions](https://github.com/evilmartians/lefthook/blob/master/docs/full_guide.md#installation)
- `lefthook install`


Then, you should be able to verify the installation:

```
Lefthook run pre-commit
```

the output should look like this:

```
✔️  fmt
✔️  test
✔️  build
✔️  clippy
```

## Installation

First, install rustup

```sh
# Be sure to have installed build-essential, i.e. sudo apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
```

Ensure you have the correct rust version (compare with [rust-toolchain.toml](rust-toolchain.toml) file)

```sh
rustup default stable
rustc --version
cargo clippy --version
cargo fmt --version
```

```sh
# Build local
cargo build

# Run local
cargo run
```

```sh

# Build container
docker build -t error-server .

# Run container
docker run --rm -p 8080:8080 --name build error-server
```



## Objectives

### User don't have valid token
- user send request to error server with a token
- error server send request to auth server endpoint /is-allowed/:id with token
- auth server return false  (status 403)
- error server return status 403

### User have valid token and there is data
- user send request to error server with a token
- error server send request to auth server endpoint /is-allowed/:id with token
- auth server return true (status 200)
- error server  search DB with query from user (default query applied)
- error service return data in request

###  User have valid token and there is no data
- user send request to error server with a token
- error server send request to auth server endpoint /is-allowed/:id with token
- auth server return true (status 200)
- error server  search DB with query from user, but no data found or query invalid
- error service return error in request (status 40X)

### User have valid token and there is data, user request pagination query
- user send request to error server with a token with pagination query params
- error server send request to auth server endpoint /is-allowed/:id with token
- auth server return true (status 200)
- error server  search DB with query from user (pagination query applied)
- error service return data in request

### User have valid token and there is data, user request specific user data
- user send request to error server with a token with id of user
- error server send request to auth server endpoint /is-allowed/:id with token
- auth server return true (status 200)
- error server  search DB with query from user
- error service return data in request
