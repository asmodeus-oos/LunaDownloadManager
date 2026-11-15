@echo off
setlocal
cd /d "%~dp0"

:: Check for administrative rights
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Requesting Administrator privileges to uninstall Mister Download Manager and install Luna...
    powershell -NoProfile -Command "Start-Process -FilePath '%~f0' -Verb RunAs"
    exit /b
)

echo ========================================================
echo   Luna Download Manager - Complete Clean Install Script
echo ========================================================
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0clean_install_luna.ps1"

echo.
echo Process complete.
timeout /t 5
