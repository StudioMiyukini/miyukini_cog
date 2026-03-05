param(
    [int]$Port = 8765,
    [switch]$NoBrowser
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$mipRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location -LiteralPath $mipRoot

function Get-PythonCommand {
    $py = Get-Command py -ErrorAction SilentlyContinue
    if ($py) {
        return @{ Cmd = "py"; Args = @("-3") }
    }

    $python = Get-Command python -ErrorAction SilentlyContinue
    if ($python) {
        return @{ Cmd = "python"; Args = @() }
    }

    throw "Python introuvable. Installe Python 3 puis relance."
}

$pythonInfo = Get-PythonCommand
$url = "http://127.0.0.1:$Port/index.html"

Write-Host "MIP root : $mipRoot"
Write-Host "Portail  : $url"
Write-Host ""
Write-Host "Arret du serveur : Ctrl+C"

if (-not $NoBrowser) {
    Start-Process $url | Out-Null
}

$args = @() + $pythonInfo.Args + @("-m", "http.server", "$Port", "--bind", "127.0.0.1")
& $pythonInfo.Cmd @args
