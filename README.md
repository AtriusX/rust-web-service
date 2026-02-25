# Web Service Example

This project is a testing ground for gaining a bit of experience in writing web servers with Rust.
The project relies on Axum and SQLx as the primary dependencies which drive the lifecycle of API requests.
As time goes on, this project will be slowly iterated upon as I learn more about the language.

## Setup

Setting up the app requires you to have a postgres database available to you. The recommended way to configure this
would be to deploy it over Docker:

```bash
docker run --name database -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres
```

Once the database is up, you should define a `.env` file in the project root with the following content:
```dotenv
DATABASE_URL=postgresql://postgres:password@localhost:5432/postgres
# Optional, but can be used to define the log levels for individual crates and files
RUST_LOG=debug
```

After you have that configured, you can run `cargo run` to start the web server. It is recommended that you use either
[Insomnia](https://insomnia.rest/) or [Postman](https://www.postman.com/) to perform your requests.

## Endpoints

The app contains a few endpoints you can perform as of current. At this time none of the provided endpoints have 
authentication associated with them, but at a later point this may be experimented with.

### Users

- `POST /user` - Allows you to create a new user entry in the app. Will error if the body contains an existing ID.
- `GET /user/{id}` - Retrieves a single user instance from the database, or 404 if the user doesn't exist.
- `GET /users` - Retrieves all currently stored users in the application, or an empty array if none exist.
- `PUT /user` - Updates an existing user in the app. Will error if the body does not have an associated ID, or 404 
    if the user does not exist.
- `DELETE /user/{id}` - Deletes a user from the app regardless of if one exists or not. Will return 404 if the user did
    not exist already, but have no other side effects.

## Tests

The app can be tested by running `cargo test`. Unlike Spring Boot applications (which I'm more familiar with), Rust
defines their unit tests within a test module in the same file marked with `#[cfg(test)]`. These modules do not get
compiled into the final app, but can make individual files a bit more verbose to parse through. At the same time, this
could be potentially beneficial for keeping tests grouped together with the code they are related to, meaning parts can
be pulled in and out more easily than elsewhere.

### Examples

The tests present in this app are provided in the [user_controller](src/users/user_controller.rs), 
[user_manager](src/users/user_manager.rs), and [user_repository](src/users/user_repository.rs) files, and demonstrates a
full top to bottom test of all elements in the stack.

Running integration style tests over the controller and repository levels of the app proves to be a decent experience,
as SQLx has built in testing macros for generating a transactional connection pool for each test. This means tests can
hit the database without anything being left behind. Unit tests with mocking however will need more work though. 
Currently, the approach I've seen recommended is using dynamic traits, but for that, the entire trait will need to be
reimplemented any time you want to change the implementation. There appear to be some mocking crates that are worth
looking into, but those will be explored at a later time.