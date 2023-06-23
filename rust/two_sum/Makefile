# Variables
PROJECT_NAME := two_sum
CARGO := $(shell which cargo)
CARGO_FLAGS :=
CARGO_BUILD_FLAGS := --release

# Default rule (build the project)
all: build

# Build the project
.PHONY: build
build:
	$(CARGO) build $(CARGO_FLAGS)

# Run the project
.PHONY: run
run: build
	$(CARGO) run $(CARGO_FLAGS)

# Run tests
.PHONY: test
test:
	$(CARGO) test $(CARGO_FLAGS)

# Build the project in release mode
.PHONY: release
release:
	$(CARGO) build $(CARGO_FLAGS) $(CARGO_BUILD_FLAGS)

# Clean the project
.PHONY: clean
clean:
	$(CARGO) clean

# Format the code
.PHONY: fmt
fmt:
	$(CARGO) fmt

# Lint the code
.PHONY: lint
lint:
	$(CARGO) clippy

