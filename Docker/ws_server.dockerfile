# Use the official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /ws_server

# Copy the rest of the project files to the container
COPY ./ws_server/ ./

# Build the application
RUN cargo build

# Expose the service to a specific port
EXPOSE 8080

# Set the command to run the application
CMD ["cargo", "run", "--bin", "ws_server"]