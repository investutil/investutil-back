# InvestUtil Backend

A modern investment utility backend service built with Rust and Axum, following Domain-Driven Design (DDD) principles.
See more infos about the project in this repo:
https://github.com/investutil/investutil-principal
frontend:
https://github.com/investutil/investutil-front

## Architecture

This project implements a clean DDD architecture with the following layers:

### 1. Domain Layer (`src/domain/`)
The core business logic layer, completely independent of external concerns.

- `entities/`: Core business objects (e.g., User)
- `repositories/`: Repository interfaces
- `value_objects/`: Value objects for domain concepts
- `events/`: Domain events
- `aggregates/`: Aggregate roots

### 2. Application Layer (`src/application/`)
Orchestrates the flow of data and implements use cases.

- `services/`: Application services (e.g., AuthService)
- `dtos/`: Data Transfer Objects
- `commands/`: Command handlers (CQRS)
- `queries/`: Query handlers (CQRS)

### 3. Infrastructure Layer (`src/infrastructure/`)
Implements technical concerns and interfaces with external systems.

- `persistence/`: Database implementations
  - `postgres/`: PostgreSQL repositories
- `messaging/`: Message queue implementations
- `logging/`: Logging infrastructure
- `config/`: Configuration management

### 4. Interfaces Layer (`src/interfaces/`)
Handles external communication and API endpoints.

- `api/`: API routes and handlers
- `controllers/`: Request handlers
- `middleware/`: HTTP middleware

## Technology Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - A modern, ergonomic web framework
- **Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication**: JWT (JSON Web Tokens)
- **Password Hashing**: bcrypt
- **API Documentation**: (Coming soon)

## Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL

### Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd investutil-back
   ```

2. Create a `.env` file:
   ```env
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/investutil
   JWT_SECRET=your_jwt_secret_key_here
   HOST=127.0.0.1
   PORT=8080
   ```

3. Set up the database:
   ```bash
   cargo sqlx database create
   cargo sqlx migrate run
   ```

4. Generate SQLx offline mode files:
   ```bash
   cargo sqlx prepare
   ```

5. Run the server:
   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8080`.

## API Endpoints

### Authentication
- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login and receive JWT token

### Market Data
- `GET /api/market/open` - Check if the market is open

## Development

### Project Structure
```
src/
├── domain/           # Domain layer
├── application/      # Application layer
├── infrastructure/   # Infrastructure layer
├── interfaces/       # Interface layer
├── lib.rs           # Library root
└── main.rs          # Application entry point
```

### Adding New Features

1. Define domain entities and repositories in the domain layer
2. Implement use cases in the application layer
3. Add infrastructure implementations as needed
4. Create API endpoints in the interfaces layer

### Running Tests

```bash
cargo test
```

## DDD Implementation Details

### Domain Layer
- Entities are the core business objects
- Repositories define interfaces for data access
- Value Objects represent immutable concepts
- Domain Events capture business events
- Aggregates maintain consistency boundaries

### Application Layer
- Services orchestrate domain objects
- DTOs handle data transfer
- Commands and Queries separate write/read operations (CQRS)

### Infrastructure Layer
- Repository implementations handle data persistence
- External service integrations
- Cross-cutting concerns (logging, configuration)

### Interfaces Layer
- RESTful API endpoints
- Request/Response handling
- Authentication middleware

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT token generation
- `HOST`: Server host (default: 127.0.0.1)
- `PORT`: Server port (default: 8080)

## Deployment

For deployment instructions, please refer to the [InvestUtil Infrastructure Repository](https://github.com/investutil/investutil-infra).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
