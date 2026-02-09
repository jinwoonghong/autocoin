@echo off
echo ========================================
echo Building AutoCoin WebSocket Server
echo ========================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Rust is not installed. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

cargo --version >nul 2>&1
if errorlevel 1 (
    echo ERROR: Cargo is not installed. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo Building the project...
cargo build --release

if errorlevel 1 (
    echo ERROR: Build failed. Please check the code for compilation errors.
    pause
    exit /b 1
)

echo ========================================
echo Starting server in background
echo ========================================

REM Start the server
start "AutoCoin Server" /B .\target\release\autocoin.exe --host 0.0.0.0 --port 8080

echo Server starting...
timeout /t 3 /nobreak >nul

echo ========================================
echo Testing WebSocket connection
echo ========================================

REM Test WebSocket
echo Testing WebSocket connection...
node web_test.js

echo ========================================
echo Testing HTTP endpoints
echo ========================================

REM Test HTTP endpoints
echo Testing HTTP endpoints...
node http_test.js

echo ========================================
echo Opening dashboard
echo ========================================

REM Open dashboard in browser
start http://localhost:8080/dashboard

echo ========================================
echo Done! Check the output above for test results
echo ========================================

pause