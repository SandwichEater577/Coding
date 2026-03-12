# TechniCasino

Online casino web application.

## Tech Stack

| Layer      | Technology                   |
| ---------- | ---------------------------- |
| Frontend   | TypeScript, HTML, CSS (Vite) |
| Backend    | Rust (Axum, Tokio, SQLx)     |
| Database   | PostgreSQL                   |
| Cache      | Redis                        |
| Real-time  | WebSockets (Axum)            |
| Containers | Docker Compose               |

## Project Structure

```
TechniCasino/
├── frontend/          # Vanilla TS + Vite
├── backend/           # Rust API server
├── database/          # SQL migrations
├── docker-compose.yml
└── .env.example
```

## Getting Started

```bash
# 1. Copy environment variables
cp .env.example .env

# 2. Start databases
docker compose up -d postgres redis

# 3. Run backend
cd backend && cargo run

# 4. Run frontend
cd frontend && npm install && npm run dev
```
