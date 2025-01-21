# Docker + React/Typescript + Rust

## Docker Commands

### Build and Start All Services (CLI simplifies everything)

#### Build and run all services:

```bash
    docker compose up --build
```

-   Frontend Dev Server (HMR): http://localhost:3000
-   Backend Serving Production Build (SSR): http://localhost:8080

### Stop and Remove Containers

#### Stop and clean up running containers:

```bash
    docker compose down
```

### Rebuild Specific Services

#### Rebuild and start only a specific service:

-   Frontend Dev Server (HMR):

```bash
    docker compose up --build react-frontend
```

-   Frontend Build:

```bash
    docker compose up --build react-frontend-build
```

-   Backend Build & Run:

```bash
    docker compose up --build rust-backend
```

### Check Logs

#### View logs for a specific service:

```bash
    docker logs <container_name>
```

## Setting Up Projects

### React/TypeScript Project

#### 1. Create a new project:

```bash
    yarn create vite frontend --template react-ts
```

```bash
    cd frontend
```

```bash
    yarn install
```

#### 2. Start the development server:

```bash
    yarn dev
```

#### 3. Build for production:

```bash
    yarn build
```

### Rust Project

#### 1. Create a new project:

```bash
    cargo new backend
```

```bash
    cd backend
```

#### 2. Run the project:

```bash
    cargo run
```

#### 3. Build for production:

```bash
    cargo build --release
```

#### 4. Add dependencies (edit Cargo.toml):

```toml
    [dependencies]
    actix-web = "4.0"
    serde = { version = "1.0", features = ["derive"] }
```

#### Then build:

```bash
    cargo build
```
