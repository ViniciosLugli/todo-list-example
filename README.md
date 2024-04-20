# Raw ToDo List API Example

# Raw ToDo List API Example

This project implements a simple ToDo list API in Rust using `TcpListener` for handling HTTP requests without a framework. It utilizes basic HTTP authentication and CRUD operations on ToDo tasks associated with user accounts. The project also demonstrates the use of Rust's `Arc` and `Mutex` for state management across threads.

## Features

-   Create, Read, Update, Delete (CRUD) operations on ToDo tasks.
-   User registration and authentication using HTTP Basic Auth.
-   Thread-safe state management using `Arc<Mutex<T>>`.
-   Detailed logging with `pretty_env_logger`.
-   Password hashing with `bcrypt`.
-   JSON serialization with `serde`.
-   Containerization with Docker.
-   Unit tests for request handling.

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
    cargo run --release
    ```

### Using Docker

1. Run the container:
    ```sh
    docker compose -f docker-compose-dev.yml up
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
