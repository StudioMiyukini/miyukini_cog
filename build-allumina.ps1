# Build allumina_prototype (MGE workspace)
# Ajoute Cargo au PATH si necessaire et utilise le bon manifest

$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $cargoBin) {
    $env:Path = "$cargoBin;$env:Path"
}

Set-Location $PSScriptRoot
cargo build -p allumina_prototype --manifest-path mge/Cargo.toml @args
