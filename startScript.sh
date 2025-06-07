#!/bin/sh

env_generator() {
    echo "Generating .env..."
    {
        echo "DATABASE_URL='${DATABASE_URL}'"
        echo "SERVER_ADDRESS='${SERVER_ADDRESS}'"
    } > /app/.env 

    ls -la /app/.env
    cat /app/.env
}

env_generator

echo "Starting Rust application..."
exec ./crud_axum