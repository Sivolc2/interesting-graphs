#!/bin/bash

echo "🧹 Cleaning up development processes and ports..."

# Kill cargo-leptos processes
echo "Killing cargo-leptos processes..."
pkill -f "cargo-leptos" 2>/dev/null && echo "✅ Killed cargo-leptos processes" || echo "ℹ️  No cargo-leptos processes found"

# Kill any cargo build processes
echo "Killing cargo build processes..."
pkill -f "cargo build" 2>/dev/null && echo "✅ Killed cargo build processes" || echo "ℹ️  No cargo build processes found"

# Kill any rustc processes (compilation processes)
echo "Killing rustc compilation processes..."
pkill -f "rustc.*frontend" 2>/dev/null && echo "✅ Killed rustc frontend processes" || echo "ℹ️  No rustc frontend processes found"
pkill -f "rustc.*backend" 2>/dev/null && echo "✅ Killed rustc backend processes" || echo "ℹ️  No rustc backend processes found"

# Check for processes on common development ports
echo "Checking for processes on development ports..."

# Port 3000 (default leptos/react dev server)
PORT_3000=$(lsof -ti:3000 2>/dev/null)
if [ ! -z "$PORT_3000" ]; then
    echo "Killing processes on port 3000..."
    kill -9 $PORT_3000 2>/dev/null && echo "✅ Freed port 3000" || echo "⚠️  Could not free port 3000"
else
    echo "ℹ️  Port 3000 is free"
fi

# Port 8080 (alternative dev server port)
PORT_8080=$(lsof -ti:8080 2>/dev/null)
if [ ! -z "$PORT_8080" ]; then
    echo "Killing processes on port 8080..."
    kill -9 $PORT_8080 2>/dev/null && echo "✅ Freed port 8080" || echo "⚠️  Could not free port 8080"
else
    echo "ℹ️  Port 8080 is free"
fi

# Port 8000 (another common dev port)
PORT_8000=$(lsof -ti:8000 2>/dev/null)
if [ ! -z "$PORT_8000" ]; then
    echo "Killing processes on port 8000..."
    kill -9 $PORT_8000 2>/dev/null && echo "✅ Freed port 8000" || echo "⚠️  Could not free port 8000"
else
    echo "ℹ️  Port 8000 is free"
fi

# Wait a moment for processes to fully terminate
sleep 2

# Final check for any remaining cargo-leptos or related processes
REMAINING=$(pgrep -f "cargo-leptos|leptos.*watch" 2>/dev/null)
if [ ! -z "$REMAINING" ]; then
    echo "⚠️  Some processes are still running, force killing them..."
    kill -9 $REMAINING 2>/dev/null && echo "✅ Force killed remaining processes"
else
    echo "✅ All development processes cleaned up successfully"
fi

echo ""
echo "🎯 Cleanup Summary:"
echo "   - Cargo-leptos processes: stopped"
echo "   - Build processes: stopped" 
echo "   - Development ports (3000, 8080, 8000): freed"
echo ""
echo "You can now run 'cargo leptos watch' or start other development servers." 