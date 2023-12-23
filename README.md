# Todo API

This is a simple Todo API built using the Actix web framework and MongoDB. The API provides endpoints to manage a todo list.

## Endpoints

### 1. Get all Todos

- **Endpoint:** `/todos`
- **Method:** `GET`
- **Description:** Retrieve a list of all todos.
- **Response:**
  - Status Code: `200 OK`
  - Body: JSON array containing todo items.

### 2. Create a Todo

- **Endpoint:** `/todos`
- **Method:** `POST`
- **Description:** Create a new todo.
- **Request Body:**
  - JSON object containing todo details (e.g., title, description).
- **Response:**
  - Status Code: `201 Created` if the todo is successfully created.
  - Body: JSON representation of the created todo and older todos.

### 3. Delete a Todo

- **Endpoint:** `/todos/{id}`
- **Method:** `DELETE`
- **Description:** Delete a todo by its unique identifier.
- **Parameters:**
  - `id` (Path parameter) - Unique identifier of the todo to be deleted.
- **Response:**
  - Status Code: `204 No Content` if the todo is successfully deleted.
  - Status Code: `404 Not Found` if the todo with the specified ID is not found.

### 4. Register a User

- **Endpoint:** `/register`
- **Method:** `POST`
- **Description:** Register a new user.
- **Request Body:**
  - JSON object containing user registration details (e.g., username, password).
- **Response:**
  - Status Code: `201 Created` if the user is successfully registered.
  - Body: JSON representation of the registered user.

### 5. Login

- **Endpoint:** `/login`
- **Method:** `POST`
- **Description:** Authenticate and log in a user.
- **Request Body:**
  - JSON object containing user login details (e.g., username, password).
- **Response:**
  - Status Code: `200 OK` if the user is successfully authenticated.
  - Status Code: `401 Unauthorized` if the login credentials are invalid.
  - Body: JSON representation of the authenticated user.


## Technologies Used

- Actix web framework
- MongoDB

