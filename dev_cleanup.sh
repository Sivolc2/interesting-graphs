#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧹 Development Environment Cleanup Script${NC}"
echo "======================================================"

# Function to kill processes by pattern
kill_processes() {
    local pattern=$1
    local description=$2
    local pids=$(pgrep -f "$pattern" 2>/dev/null)
    
    if [ ! -z "$pids" ]; then
        echo -e "${YELLOW}Killing $description processes...${NC}"
        kill -9 $pids 2>/dev/null && echo -e "${GREEN}✅ Killed $description processes${NC}" || echo -e "${RED}⚠️  Could not kill some $description processes${NC}"
    else
        echo -e "ℹ️  No $description processes found"
    fi
}

# Function to free port
free_port() {
    local port=$1
    local pids=$(lsof -ti:$port 2>/dev/null)
    
    if [ ! -z "$pids" ]; then
        echo -e "${YELLOW}Freeing port $port...${NC}"
        kill -9 $pids 2>/dev/null && echo -e "${GREEN}✅ Freed port $port${NC}" || echo -e "${RED}⚠️  Could not free port $port${NC}"
    else
        echo "ℹ️  Port $port is free"
    fi
}

echo ""
echo "🔄 Stopping development processes..."

# Rust/Leptos processes
kill_processes "cargo-leptos" "cargo-leptos"
kill_processes "cargo.*watch" "cargo watch"
kill_processes "cargo.*build" "cargo build"
kill_processes "cargo.*run" "cargo run"
kill_processes "rustc.*frontend" "rustc frontend compilation"
kill_processes "rustc.*backend" "rustc backend compilation"

# Node.js/Frontend processes
kill_processes "npm.*start" "npm start"
kill_processes "npm.*dev" "npm dev"
kill_processes "yarn.*start" "yarn start"
kill_processes "yarn.*dev" "yarn dev"
kill_processes "pnpm.*dev" "pnpm dev"
kill_processes "next.*dev" "Next.js dev"
kill_processes "vite.*dev" "Vite dev"
kill_processes "webpack.*dev" "Webpack dev"
kill_processes "react-scripts" "React dev server"

# Python processes
kill_processes "python.*manage.py.*runserver" "Django dev server"
kill_processes "flask.*run" "Flask dev server"
kill_processes "uvicorn" "FastAPI/Uvicorn"
kill_processes "gunicorn" "Gunicorn"

echo ""
echo "🔌 Checking common development ports..."

# Common development ports
declare -a ports=("3000" "3001" "4000" "5000" "8000" "8080" "8081" "8888" "9000")

for port in "${ports[@]}"; do
    free_port $port
done

# Wait for processes to terminate
sleep 2

echo ""
echo "🔍 Final cleanup check..."

# Check for any remaining development processes
REMAINING_RUST=$(pgrep -f "cargo-leptos|cargo.*watch|cargo.*dev" 2>/dev/null)
REMAINING_NODE=$(pgrep -f "npm.*dev|yarn.*dev|pnpm.*dev|next.*dev" 2>/dev/null)

if [ ! -z "$REMAINING_RUST" ] || [ ! -z "$REMAINING_NODE" ]; then
    echo -e "${YELLOW}⚠️  Some processes are still running, force killing...${NC}"
    [ ! -z "$REMAINING_RUST" ] && kill -9 $REMAINING_RUST 2>/dev/null
    [ ! -z "$REMAINING_NODE" ] && kill -9 $REMAINING_NODE 2>/dev/null
    echo -e "${GREEN}✅ Force killed remaining processes${NC}"
else
    echo -e "${GREEN}✅ All development processes cleaned up successfully${NC}"
fi

# Clean up common temporary files/directories
echo ""
echo "🗑️  Cleaning temporary files..."

# Node.js
[ -d "node_modules/.cache" ] && rm -rf node_modules/.cache && echo "✅ Cleaned node_modules/.cache"

# Rust
[ -d "target/debug/incremental" ] && rm -rf target/debug/incremental && echo "✅ Cleaned Rust incremental compilation cache"

# Next.js
[ -d ".next" ] && rm -rf .next && echo "✅ Cleaned Next.js .next directory"

# Vite
[ -d "dist" ] && rm -rf dist && echo "✅ Cleaned Vite dist directory"

echo ""
echo -e "${GREEN}🎯 Cleanup Summary:${NC}"
echo "   ✅ Development servers: stopped"
echo "   ✅ Build processes: stopped"
echo "   ✅ Development ports: freed"
echo "   ✅ Temporary files: cleaned"
echo ""
echo -e "${BLUE}Ready to start fresh development servers! 🚀${NC}"

# Optional: Show what ports are still in use
echo ""
echo "📊 Currently listening ports:"
lsof -i -P | grep LISTEN | head -5 