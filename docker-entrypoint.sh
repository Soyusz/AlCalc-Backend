#!/bin/bash
set -e

echo "Running database migrations..."
diesel migration run
echo "Starting application..."

exec ./alcalc_backend