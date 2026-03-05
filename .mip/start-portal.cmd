@echo off
setlocal
set SCRIPT_DIR=%~dp0
powershell -ExecutionPolicy Bypass -NoProfile -File "%SCRIPT_DIR%start-portal.ps1" %*
