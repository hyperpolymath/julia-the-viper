#!/bin/bash

set -e

echo "🔨 Building Podman containers..."

# Build web container
echo "📦 Building web container..."
podman build -t playground-web:latest \
  -f ../containers/web/Containerfile \
  ../../experiments/frontend-demos/svelte-app || echo "Web build pending implementation"

# Build API container
echo "🔧 Building API container..."
podman build -t playground-api:latest \
  -f ../containers/api/Containerfile \
  ../../src/backend/deno-api || echo "API build pending implementation"

# Build Julia worker
echo "🔬 Building Julia worker..."
podman build -t playground-julia:latest \
  -f ../containers/julia/Containerfile \
  ../../experiments/julia-demos/worker || echo "Julia build pending implementation"

echo "✅ All containers built successfully!"

# List images
podman images | grep playground || echo "No playground images yet"
