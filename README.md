# Raw ToDo List API Example

This project implements a simple ToDo list API in Rust using `TcpListener` for handling HTTP requests without a framework, all the code is synchronous and uses `Arc<Mutex<T>>` to handle the state of the application. It utilizes basic HTTP authentication and CRUD operations on ToDo tasks associated with user accounts. The project also demonstrates the use of Rust's `Arc` and `Mutex` for state management across threads. The project is containerized with Docker for easy deployment. The API is documented using Swagger and Insomnia.

## Features

-   Create, Read, Update, Delete (CRUD) operations on ToDo tasks.
-   User registration and authentication using HTTP Basic Auth.
-   Authorization to modify and delete tasks.
-   Thread-safe state management using `Arc<Mutex<T>>` (if you want to setup the application to use multiple threads).
-   Detailed logging with `pretty_env_logger`.
-   Password hashing with `bcrypt`.
-   JSON serialization with `serde`.
-   Containerization with Docker.
-   Unit tests for all modules.

## Architecture

### Components

-   `main.rs`: Sets up the TCP server and handles incoming connections.
-   `server.rs`: Defines the `Server` struct and methods for request handling.
-   `task.rs`: Defines the `Task` and `PublicUser` structures.
-   `user.rs`: Defines the `User` structure with methods for password hashing and authentication.
-   `response.rs`: Utility for constructing HTTP responses.

### Libraries

-   `serde`, `serde_json`: For JSON serialization.
-   `log`, `pretty_env_logger`: For logging.
-   `bcrypt`: For hashing passwords.
-   `base64`: For encoding and decoding Basic Auth headers.

## Prerequisites

-   Rust (latest stable version) or Docker (for containerized deployment).

## Installation and Running

### Local Setup

1. Clone the repository:

    ```sh
    git clone git@github.com:ViniciosLugli/raw-todo-list-api-example.git
    cd raw-todo-list-api-example
    ```

2. Build and run the project:

    ```sh
    RUST_LOG=trace cargo run --release
    ```

    > Note: The `RUST_LOG` environment variable sets the log level. You can set it to `info`, `debug`, or `trace`.

3. Run the tests:
    ```sh
    cargo test
    ```

### Using Docker

-   Run the container for development:

    ```sh
    docker compose -f docker-compose-dev.yml up
    ```

-   Run the container for testing:
    ```sh
    docker compose -f docker-compose-test.yml up
    ```

## Usage

Once the server is running, it will listen on `0.0.0.0:3000`. You can interact with the API using any HTTP client.

### API Endpoints

-   `POST /users`: Register a new user.
-   `POST /tasks`: Create a new task (requires Basic Auth).
-   `GET /tasks`: Retrieve all tasks (requires Basic Auth).
-   `PUT /tasks/{id}`: Update a specific task (requires Basic Auth).
-   `DELETE /tasks/{id}`: Delete a specific task (requires Basic Auth).

You can find the swagger documentation on [assets/swagger_api.yaml](assets/swagger_api.yaml). also you can use the insomnia file [assets/insomnia_collection.json](assets/insomnia_collection.json) to test the API.

### Behavior of the API requests

Now that the server is running, you can interact with the API using any HTTP client, for the example, we will use `Insomnia` to analyze the behavior of the API requests.

#### Results of the requests

The server is a simple ToDo list API that allows users to create, read, update, and delete tasks using raw TCP connections. The server uses HTTP Basic Auth for user authentication and authorization. The server is running synchronously and uses a single thread to handle incoming connections, because of this, the server can only handle one request at a time, for example, if you try to create a task while the server is processing another request, the server will not respond until the current request is completed and the server is ready to process the next request, this can be tested using the `Insomnia` tool and sending multiple requests at the same time, you will see that the requests are a little slow to respond. You also can use `Curl` with `Bash` to test the API and see the behavior of the requests.

To run the tests, you can use the following commands:

```sh
docker compose -f docker-compose-dev.yml up
# After the server is running you can run the benchmark tests
docker compose -f docker-compose-benchmark.yml up
```

the output of the tests will be something like this:

```sh
benchmark-1  | Total Duration for creating users: 4 ms
benchmark-1  | Total Duration for creating tasks: 2 ms
benchmark-1  | Total Duration for getting tasks: 4 ms
benchmark-1  | All requests have been sent and processed.
```

all the requests are processed sequentially, this is because the server is running synchronously and uses a single thread to handle incoming connections.
