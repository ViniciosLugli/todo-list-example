# ToDo List Example

The project is a simple ToDo list application that allows users to create, read, update, and delete tasks. It includes user registration and authentication using HTTP Basic Auth. Provide by a RESTful API implemented in [Rust](https://www.rust-lang.org/pt-BR) and a mobile dashboard using [Flutter](https://flutter.dev/).

This repository consists of two main components: a `backend API` implemented in [Rust](https://www.rust-lang.org/pt-BR) and a `frontend dashboard` using [Flutter](https://flutter.dev/).

## API Component (api/)

This api implements a simple ToDo list API in [Rust](https://www.rust-lang.org/pt-BR) using `TcpListener` to handle HTTP requests without a framework. The application can run synchronously or asynchronously, determined by cargo features. It uses `Arc<Mutex<T>>` for thread-safe state management and supports basic HTTP authentication along with CRUD operations for tasks associated with user accounts. The project is containerized with Docker for adaptable deployment and documented using Swagger and Insomnia.

### Features

-   Create, Read, Update, Delete (CRUD) operations on ToDo tasks.
-   User registration and authentication using HTTP Basic Auth.
-   Authorization to modify and delete tasks.
-   Configurable synchronous or asynchronous operation.
-   Thread-safe state management using `Arc<Mutex<T>>`.
-   Detailed logging with `pretty_env_logger`.
-   Password hashing with `bcrypt`.
-   JSON serialization with `serde`.
-   Containerization with Docker.
-   Unit tests for all modules.

### Architecture

#### Components

-   `main.rs`: Initializes the TCP server and handles incoming connections.
-   `server.rs`: Defines the `Server` struct and methods for request handling.
-   `task.rs`: Manages the `Task` and `PublicUser` structures.
-   `user.rs`: Manages the `User` structure, including password hashing and authentication.
-   `response.rs`: Utility for constructing HTTP responses.

#### Libraries

-   `serde`, `serde_json`: For JSON serialization.
-   `log`, `pretty_env_logger`: For logging.
-   `bcrypt`: For hashing passwords.
-   `base64`: For encoding and decoding Basic Auth headers.
-   `tokio` and `async-std`: For asynchronous operation.

### Prerequisites

-   Rust (latest stable version) or Docker (for containerized deployment).

### Installation and Running

#### Local Setup

1. Get the project:

    First, clone the repository:

    ```sh
    git clone git@github.com:ViniciosLugli/todo-list-example.git
    ```

    Then, navigate to the project and API directory, where the server is located:

    ```sh
    cd todo-list-example/api
    ```

2. Build and run the project synchronously:

    ```sh
    RUST_LOG=trace cargo run --features sync
    ```

    Build and run the project asynchronously:

    ```sh
    RUST_LOG=trace cargo run --features async
    ```

3. Run the tests:
    ```sh
    cargo test
    ```

#### Using Docker

-   Run the container for development synchronously:

    ```sh
    FEATURES=sync docker compose -f docker-compose-dev.yml up
    ```

-   Run the container for development asynchronously:

    ```sh
    FEATURES=async docker compose -f docker-compose-dev.yml up
    ```

-   Run the container for testing:
    ```sh
    docker compose -f docker-compose-test.yml up
    ```

### Usage

Once the server is running, it will listen on `0.0.0.0:3000`. You can interact with the API using any HTTP client.

#### API Endpoints

-   `POST /users`: Register a new user.
-   `POST /tasks`: Create a new task (requires Basic Auth).
-   `GET /tasks`: Retrieve all tasks (requires Basic Auth).
-   `PUT /tasks/{id}`: Update a specific task (requires Basic Auth).
-   `DELETE /tasks/{id}`: Delete a specific task (requires Basic Auth).

Swagger documentation is available at [assets/swagger_api.yaml](assets/swagger_api.yaml). Additionally, you can use the Insomnia file [assets/insomnia_collection.json](assets/insomnia_collection.json) to test the API.

### Benchmark Tests

Run the benchmark tests with Docker after starting the server:

```sh
docker compose -f docker-compose-benchmark.yml up
```

Expected output:

```sh
benchmark-1  | Total Duration for creating users: 2 ms
benchmark-1  | Total Duration for creating tasks: 7 ms
benchmark-1  | Total Duration for getting tasks: 5 ms
benchmark-1  | All requests have been sent and processed.
```

#### Abount the Benchmark tests results

-   All the requests are processed sequentially due to the server running synchronously and using a single thread to handle incoming connections. You can switch between synchronous and asynchronous modes by setting the `FEATURES` environment variable accordingly before executing commands.
-   The benchmark tests are designed to measure the server's performance under a high load of requests. The server will process 1000 requests for creating users, 1000 requests for creating tasks, and 1000 requests for getting tasks. The total duration for each operation is displayed at the end of the test.
-   The synchronous server is expected to take longer to process all requests compared to the asynchronous server due to the single-threaded nature of the former. While the asynchronous server can handle multiple requests concurrently, providing better performance under high loads.

## Dashboard Component (`dashboard/`)

### Overview

A mobile application template in Flutter designed for the ToDo List project. It includes necessary setups for a streamlined development process with a focus on usability and adaptability for developers.

### Used Tools

-   Flutter, Dart, Android Studio, Visual Studio Code, Git.

### Setup and Installation

Follow the detailed setup instructions in the [Flutter Mobile Template README](https://github.com/ViniciosLugli/flutter-mobile-template) to set up the Flutter project and configure the development environment with Android Studio.

### Running the Project

1. Navigate to the dashboard directory:

    ```sh
    cd todo-list-example/dashboard
    ```

2. Install dependencies and set up the environment:

    - Install Flutter dependencies: `flutter pub get`
    - Set up Android Studio and configure emulators.

3. Launch and run the project on an emulator:
    - Configure and launch an emulator: `flutter emulators --launch <device_name>`
    - Run the project: `flutter run`

## Demo

The following video demonstrates the ToDo List project in action:

[demo.webm](https://github.com/ViniciosLugli/todo-list-example/assets/40807526/ec303809-c90d-404a-9d0f-fa71017f1dfa)
