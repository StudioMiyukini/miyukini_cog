Set-Location "C:\Users\miyuk\Cursor\Miyukini-COG"

# Map: relative path -> (id, do_, human)
$data = @{
}

$ok = 0; $skip = 0
foreach ($rel in $data.Keys) {
  $fullpath = Join-Path (Get-Location) $rel
  if (-not (Test-Path $fullpath)) { Write-Host "MISSING: $rel"; $skip++; continue }
  $lines = Get-Content $fullpath -Encoding UTF8
  if ($lines | Where-Object { $_ -match '@id:' }) { Write-Host "SKIP(already @id): $rel"; $skip++; continue }

  # Find last consecutive //! line at top
  $lastDocLine = -1
  for ($i = 0; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match '^//!') { $lastDocLine = $i }
    elseif ($i -eq 0) { break }
    elseif ($lastDocLine -ge 0) { break }
  }
  if ($lastDocLine -lt 0) { Write-Host "NODOC: $rel"; $skip++; continue }

  $id   = $data[$rel][0]
  $do_  = $data[$rel][1]
  $hum  = $data[$rel][2]
  $mscm = @("//!", "//! @id: $id @do: $do_", "//! @role: ui @layer: service", "//! @human: $hum")

  $before = $lines[0..$lastDocLine]
  if ($lastDocLine + 1 -lt $lines.Count) {
    $after = $lines[($lastDocLine + 1)..($lines.Count - 1)]
  } else {
    $after = @()
  }
  $newlines = $before + $mscm + $after
  Set-Content $fullpath -Value $newlines -Encoding UTF8
  Write-Host "OK: $rel"
  $ok++
}
Write-Host "`n=== $ok updated, $skip skipped ==="
