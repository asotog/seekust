# Seekust

This is a utility to explore files and files content

## Installation

### Linux

```bash
# Download the binary
curl -L -o seekust https://github.com/asotog/seekust/releases/download/latest/seekust-linux-amd64

# Make it executable
chmod +x seekust

# Move to a directory in your PATH
sudo mv seekust /usr/local/bin/
```

### macOS

```bash
# Download the binary
curl -L -o seekust https://github.com/asotog/seekust/releases/download/latest/seekust-macos-amd64

# Make it executable
chmod +x seekust

# Move to a directory in your PATH
sudo mv seekust /usr/local/bin/
```

## Usage

```bash
# Basic usage (search any text in current directory)
seekust "your search text"

# Examples
seekust "TODO"          # Find files containing "TODO"
seekust "fn main"       # Find files containing "fn main"
```

## Development

### Getting Started

To build and run the application, follow these steps:

1. Ensure you have Rust and Cargo installed on your machine. You can install them from [rust-lang.org](https://www.rust-lang.org/).

2. Clone the repository:

   ```bash
   git clone <repository-url>
   cd seekust
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the application:

   ```bash
   cargo run
   ```

### Running Tests

To run the integration tests, use the following command:

```bash
cargo test
```

### Project Structure

- `src/main.rs`: Entry point of the application.
- `tests/integration_tests.rs`: Contains integration tests for the application.
- `Cargo.toml`: Configuration file for Cargo.
