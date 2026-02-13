@echo off
REM Lance prepare-public-distribution.ps1 en contournant la strategie d'execution PowerShell.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0prepare-public-distribution.ps1"
pause
