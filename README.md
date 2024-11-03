# jsPsych-Axum-Askama

A minimal template project combining jsPsych (JavaScript) with Rust's Axum web framework and Askama templating engine. This repository serves as a proof-of-concept and starting point for building web-based psychological experiments.

## Purpose

This is a small test project created to:
- Demonstrate basic integration between jsPsych, Axum, and Askama
- Provide a minimal working template for web experiments
- Test the feasibility of using Rust as a backend for jsPsych experiments
- Serve as a reference for future larger-scale implementations

## Features

- Frontend: jsPsych for psychological experiments
- Backend: Rust-based Axum web framework
- HTML templating with Askama
- Static file serving with tower-http
- JSON data collection endpoint

## Development Setup

### Prerequisites

- Rust
- Bun (JavaScript runtime & package manager)
- xh (or curl) for API testing

### Frontend (Client)
The frontend uses Bun for faster development and package management:

```bash
# Install dependencies
cd client
bun install

# Run development server
cd client
bun run dev
```

### Server

The server runs on port 3000 and handles:
- HTML template rendering
- Static file serving
- Trial data collection via POST endpoint

### Development Notes

#### Managing static files
Static files are served using tower-http routing

#### Testing the API
Test the trial endpoint with:
```bash
make dev
xh -d :3000/trial < exp.json
```
