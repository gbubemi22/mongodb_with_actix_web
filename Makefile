# Define the default target
all: run

# Target for running the application with cargo-watch
run:
	cargo watch -c -w src -x run
