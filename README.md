# Rust Starter - Mono Architecture

This is a monolithic Rust application designed for a CRUD (Create, Read, Update, Delete) operation on a user table.

## Project Structure

- **src/**: Contains the source code of the application.
  - **main.rs**: The main entry point of the application.
  - **lib.rs**: Library module containing utility functions and data models.
  - **handlers/**: Handlers for different HTTP routes.
    - **user_handler.rs**: Handles user-related CRUD operations.
- **sql_file/**: Contains SQL files for database schema creation.
  - **user.sql**: SQL script to create the `users` table.

## Dependencies

The project uses the following dependencies:

- **tokio**: Asynchronous runtime for writing efficient asynchronous applications.
- **hyper**: A fast HTTP library for Rust.
- **sqlx**: Asynchronous SQL toolkit and driver for PostgreSQL, MySQL, and SQLite.
- **dotenv**: Loads environment variables from a `.env` file.

## Setup

1. Clone the repository:
   ```sh
   git clone https://github.com/your-repo/rust_starter.git
   cd rust_starter
   ```

2. Create a `.env` file in the root directory with the following content:
   ```
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

3. Build and run the application:
   ```sh
   cargo build
   cargo run
   ```

## API Endpoints

- **POST /users**: Create a new user.
  - Request Body: JSON object with `username`, `password`, and `birthdate`.
  - Response: Created user details.

- **GET /users**: Get all users.
  - Response: List of user details.

- **GET /users/{id}**: Get a specific user by ID.
  - Response: User details.

- **PUT /users/{id}**: Update an existing user.
  - Request Body: JSON object with `username`, `password`, and `birthdate`.
  - Response: Updated user details.

- **DELETE /users/{id}**: Delete a user.
  - Response: Confirmation message.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request to improve the project.
