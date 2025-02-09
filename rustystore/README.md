
# rustystore

This microservice purpose is to manage files uploaded by users to the storage service

## Features

- Insert file (just metadata)
- Delete
- Tag (on files)

## Requirements

- PostgreSQL

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file
You can copy .env.example

`PORT` - HTTP port

`RUST_LOG` - (info, debug, warn, ..)

## Installation

Install can be done using Cargo (rust package manager)

(Unoptimized)

```bash
  cargo build --profile=dev
```

(Optimized)

```bash
  cargo build --profile=release
```

## Run Locally

```bash
  cargo run
```

## Running Tests

To run tests, run the following command

```bash
  cargo test
```

To run only the integration tests

```bash
  cargo test --test integration
```

Integration tests require .env to be configured, the service need a database

## Authors

- [@valentinb-sixense](https://www.github.com/valentinb-sixense)

## License

[MIT](https://choosealicense.com/licenses/mit/)
