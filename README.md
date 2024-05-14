# ToDo List Example

The project is a simple ToDo list application that allows users to create, read, update, and delete tasks. It includes user registration and authentication using HTTP Basic Auth. Provide by a RESTful API implemented in [Rust](https://www.rust-lang.org/pt-BR) and a mobile dashboard using [Flutter](https://flutter.dev/).

This repository consists of two main components: a `backend API` implemented in [Rust](https://www.rust-lang.org/pt-BR) and a `frontend dashboard` using [Flutter](https://flutter.dev/).

## API Component (api/)

This api implements a simple ToDo list API in [Rust](https://www.rust-lang.org/pt-BR) using Ntex, an asynchronous web framework for Rust. It includes features aimed at facilitating the development of high-performance, scalable web services with minimal boilerplate code.

### Features

-   User creation and authentication using JSON Web Tokens (JWT) for secure access to the API.
-   Create, Read, Update, Delete (CRUD) operations on ToDo tasks with persistent storage in a PostgreSQL database and permissions based on user authentication.
-   Asynchronous operation using the Tokio runtime for efficient handling of multiple concurrent requests.
-   Password hashing using the `bcrypt` crate for secure storage of user credentials in the database.
-   Fully integrates with Serde for robust data serialization and deserialization. This feature simplifies exchanging JSON data between servers and clients, seamlessly mapping Rust structures to JSON format and vice versa.
-   Uses `dotenvy` to manage environment variables through a `.env` file, streamlining the configuration process and ensuring sensitive credentials are kept out of the code.
-   Incorporates `pretty_env_logger`, an environment-aware logger for detailed and configurable logging that helps in monitoring and debugging the application efficiently during development and in production.

### Architecture

The API is structured as a RESTful web service with the following components:

#### Components

-   `main.rs`: Initializes the server and routes requests to the appropriate handlers.
-   `error.rs`: Defines custom error types for the API.
-   `utils/`: Contains utility functions for the API, such as password hashing and token generation.
-   `middleware/`: Contains middleware functions for authentication and error handling.
-   `routes/`: Contains the route handlers for the API endpoints.
-   `states/`: Contains the application state and database connection pool.
-   `repository/`: Contains the repository functions for the API endpoints.

#### Libraries

-   [ntex](https://ntex.rs/): An asynchronous web framework for Rust built on top of Tokio.
-   [ntex-cors](https://docs.rs/ntex-cors/latest/ntex_cors/): A CORS middleware for the ntex web framework.
-   [serde](https://serde.rs/) and [serde_json](https://docs.rs/serde_json/): A serialization/deserialization library for Rust.
-   [log](https://docs.rs/log/0.4.14/log/) and [pretty_env_logger](https://docs.rs/pretty_env_logger/0.4.0/pretty_env_logger/): Logging libraries for Rust.
-   [dotenvy](https://docs.rs/dotenvy/0.15.7/dotenvy/) and [dotenvy_macro](https://docs.rs/dotenvy_macro/0.15.7/dotenvy_macro/): A library for loading environment variables from a `.env` file.
-   [prisma-client-rust](https://prisma.brendonovich.dev/) (forked from [prisma](https://www.prisma.io/)): A database client for Rust that provides a type-safe API for interacting with a database.
-   [tokio](https://tokio.rs/): An asynchronous runtime for Rust.
-   [bcrypt](https://docs.rs/bcrypt/0.15.1/bcrypt/): A library for hashing passwords using the bcrypt algorithm.
-   [jsonwebtoken](https://docs.rs/jsonwebtoken/9.3.0/jsonwebtoken/): A library for creating and verifying JSON Web Tokens (JWT).
-   [chrono](https://docs.rs/chrono/0.4.30/chrono/): A date and time library for Rust.

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

2. Set environment variables:

    Copy the `.env.example` file to `.env` and fill in the required values:

    ```sh
    cp .env.example .env
    ```

    Update the `.env` file with the required values if necessary:

    ```sh
    # JWT secret key
    JWT_SECRET="SUPERSECRET"

    # Server log level
    RUST_LOG=info

    # Database URL of docker container
    DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres?schema=public"
    ```

3. Build and run the project:

    ```sh
    RUST_LOG=trace cargo run --release
    ```

4. Run the tests:
    ```sh
    cargo test
    ```

#### Using Docker

-   Run the container for development:

    ```sh
    docker compose -f docker-compose-dev.yml up
    ```

-   Run the container for testing:
    ```sh
    docker compose -f docker-compose-test.yml up
    ```

### Usage

Once the server is running, it will listen on `0.0.0.0:3000`. You can interact with the API using any HTTP client.

#### API Endpoints

You can use the Insomnia file [assets/insomnia_collection.json](assets/insomnia_collection.json) to test the API and see the available endpoints.
![image](https://github.com/ViniciosLugli/todo-list-example/assets/40807526/ee57f10e-cf6d-4df7-834f-fe0befee6224)

## Dashboard Component (`dashboard/`)

### Overview

A mobile application template in Flutter designed for the ToDo List project. It includes necessary setups for a streamlined development process with a focus on usability and adaptability for developers. The flutter app has bindings to the Rust calls using the `flutter_rust_bridge` plugin to interact with the API.

### Used Tools

-   [Flutter](https://flutter.dev/): A UI toolkit for building natively compiled applications for mobile, web, and desktop from a single codebase.
-   [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) A Flutter plugin for calling Rust functions from Dart.

### Architecture

The Flutter project is structured as a mobile application with the following components:

#### Components

The app is divided into two main components:

-   [dart sources](./dashboard/lib/): Initializes the Flutter app and routes requests to the appropriate screens.
-   [rust sources](./dashboard/rust/): Contains the Rust code that will be called from the Flutter app.

### Setup and Installation

Follow the detailed setup instructions in the [Flutter Mobile Template README](https://github.com/ViniciosLugli/flutter-mobile-template) to set up the Flutter project and configure the development environment with [Android Studio](https://developer.android.com/studio) and an Android emulator or physical device.

### Prerequisites

-   [Flutter SDK](https://flutter.dev/docs/get-started/install) (latest stable version)
-   [Android Studio](https://developer.android.com/studio) with an Android emulator or a physical device.
-   [API](./api) server running locally or in a container.
-   [Rust](https://www.rust-lang.org/pt-BR) (latest stable version)

### Running the Project

1. Navigate to the dashboard directory:

    ```sh
    cd todo-list-example/dashboard
    ```

2. Install dependencies and set up the environment:

    - Install Flutter dependencies: `flutter pub get`
    - Set up Android Studio and configure emulators.
    - Install the Rust bridge: `cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.33`

3. Set the environment variables:

    Copy the `.env.example` file to `.env` and fill in the required values:

    ```sh
    cp .env.example .env
    ```

    Update the `.env` file with the required values:

    ```sh
    API_URL = "http://YOUR_API_IP:3000"
    ```

    If you are running the API locally, you can use the command:

    ```sh
    hostname -I
    ```

    This command will return the IP address of your machine. Use this IP address in the `.env` file.

4. Launch and run the project on an emulator:
    - Configure and launch an emulator: `flutter emulators --launch <device_name>`
    - Run the project: `flutter_rust_bridge_codegen generate && flutter run`

## Demo

The following video demonstrates the ToDo List project in action:

[demo.webm](https://github.com/ViniciosLugli/todo-list-example/assets/40807526/ec303809-c90d-404a-9d0f-fa71017f1dfa)
