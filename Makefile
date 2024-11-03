dev:
	cd client && bun run build
	cd server && RUST_LOG=debug cargo run
