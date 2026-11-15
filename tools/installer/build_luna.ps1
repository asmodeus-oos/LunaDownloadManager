param([string]$SignCommand = $env:MDM_SIGN_COMMAND)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$dist = Join-Path $root "target\dist"
$bin = Join-Path $root "target\release"

$cargo = Get-Content (Join-Path $root "Cargo.toml") -Raw
$version = [regex]::Match($cargo, '(?m)^version\s*=\s*"([^"]+)"').Groups[1].Value
if (-not $version) { $version = "6.0.0" }

$iscc = @($env:ISCC, (Get-Command ISCC.exe -ErrorAction SilentlyContinue).Source,
  "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe", "$env:ProgramFiles\Inno Setup 6\ISCC.exe",
  "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe") |
  Where-Object { $_ -and (Test-Path $_) } | Select-Object -First 1

if (-not $iscc) { throw "Inno Setup 6 (ISCC.exe) not found" }

New-Item -ItemType Directory -Force $dist | Out-Null
$isccArgs = @("/Qp", "/DAppVersion=$version")
if ($SignCommand) { $isccArgs += @("/DSign", "/Smdmsign=$SignCommand") }

Write-Host "Compiling installer with ISCC..." -ForegroundColor Cyan
& $iscc @isccArgs (Join-Path $PSScriptRoot "luna.iss")
if ($LASTEXITCODE) { throw "ISCC failed with exit code $LASTEXITCODE" }

function New-Zip([string]$name, [bool]$portable) {
  $stage = Join-Path $dist "stage\$name"
  if (Test-Path $stage) { Remove-Item -LiteralPath $stage -Recurse -Force }
  New-Item -ItemType Directory -Force $stage | Out-Null
  foreach ($exe in "lunadownloadmanager.exe", "lunadownloadmanager-nmhost.exe", "lunadownloadmanager-cli.exe", "lunadownloadmanager-mcp.exe") {
    Copy-Item (Join-Path $bin $exe) $stage
  }
  New-Item -ItemType Directory -Force (Join-Path $stage "extension") | Out-Null
  if (Test-Path (Join-Path $root "target\extension\chromium")) {
    Copy-Item (Join-Path $root "target\extension\chromium") (Join-Path $stage "extension\chromium") -Recurse
  }
  if (Test-Path (Join-Path $root "target\extension\luna-firefox.xpi")) {
    Copy-Item (Join-Path $root "target\extension\luna-firefox.xpi") (Join-Path $stage "extension")
  }
  if (Test-Path (Join-Path $root "LICENSE")) {
    Copy-Item (Join-Path $root "LICENSE") (Join-Path $stage "LICENSE.txt")
  }
  if (Test-Path (Join-Path $root "ATTRIBUTION.md")) {
    Copy-Item (Join-Path $root "ATTRIBUTION.md") (Join-Path $stage "ATTRIBUTION.txt")
  }
  if ($portable) {
    New-Item -ItemType Directory -Force (Join-Path $stage "data") | Out-Null
    Set-Content -Encoding utf8 (Join-Path $stage "data\README.txt") @"
This folder makes Luna Download Manager portable: the download list and settings are kept
here instead of in your profile, and nothing is written to the registry. Delete the folder to
use the profile instead. Browser integration needs the installer.
"@
  }
  $zip = Join-Path $dist "$name.zip"
  if (Test-Path $zip) { Remove-Item -LiteralPath $zip -Force }
  for ($try = 1; $try -le 10; $try++) {
    try {
      $base = Split-Path -Parent $stage
      $archive = [System.IO.Compression.ZipFile]::Open($zip, [System.IO.Compression.ZipArchiveMode]::Create)
      try {
        foreach ($f in Get-ChildItem -LiteralPath $stage -Recurse -File | Sort-Object FullName) {
          $entry = $f.FullName.Substring($base.Length).TrimStart('\', '/').Replace('\', '/')
          [void][System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile($archive, $f.FullName,
            $entry, [System.IO.Compression.CompressionLevel]::Optimal)
        }
      } finally { $archive.Dispose() }
      break
    } catch {
      if (Test-Path $zip) { Remove-Item -LiteralPath $zip -Force }
      if ($try -eq 10) { throw }
      Start-Sleep -Seconds 2
    }
  }
}

Add-Type -AssemblyName System.IO.Compression, System.IO.Compression.FileSystem
Write-Host "Creating portable zip..." -ForegroundColor Cyan
New-Zip "LunaDownloadManager-$version-portable" $true
Write-Host "Creating x64 zip..." -ForegroundColor Cyan
New-Zip "LunaDownloadManager-$version-x64" $false
Remove-Item -LiteralPath (Join-Path $dist "stage") -Recurse -Force

# Copy browser extension zips into dist as well for easy distribution
if (Test-Path (Join-Path $root "target\extension\luna-chromium.zip")) {
  Copy-Item (Join-Path $root "target\extension\luna-chromium.zip") (Join-Path $dist "LunaDownloadManager-$version-chromium-extension.zip") -Force
}
if (Test-Path (Join-Path $root "target\extension\luna-firefox.xpi")) {
  Copy-Item (Join-Path $root "target\extension\luna-firefox.xpi") (Join-Path $dist "LunaDownloadManager-$version-firefox-extension.xpi") -Force
}

$files = Get-ChildItem $dist -File | Where-Object { $_.Name -like "LunaDownloadManager-$version-*" }
$files | ForEach-Object { "{0}  {1}" -f (Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower(), $_.Name } |
  Set-Content -Encoding ascii (Join-Path $dist "SHA256SUMS.txt")

Write-Host "Built release artifacts in ${dist}:" -ForegroundColor Green
$distFiles = Get-ChildItem $dist -File
$distFiles | ForEach-Object { "{0,-45} {1,8:N1} MB" -f $_.Name, ($_.Length / 1MB) }
