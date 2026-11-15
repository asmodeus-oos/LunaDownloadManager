# Cleanly uninstalls Mister Download Manager with NO leftovers,
# and installs Luna Download Manager into "C:\Program Files\Luna Download Manager".
$ErrorActionPreference = "Continue"
$logPath = "C:\Users\vlad\Downloads\RivaDownloadManager\clean_install_luna.log"
Start-Transcript -Path $logPath -Force

Write-Host "=== 1. Terminating running processes ===" -ForegroundColor Cyan
Stop-Process -Name 'misterdownloadmanager' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'misterdownloadmanager-nmhost' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'lunadownloadmanager' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'lunadownloadmanager-nmhost' -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

Write-Host "=== 2. Uninstalling Mister Download Manager ===" -ForegroundColor Cyan
$misterUninstaller = "C:\Program Files\Mister Download Manager\unins000.exe"
if (Test-Path $misterUninstaller) {
    Write-Host "Running uninstaller: $misterUninstaller"
    $proc = Start-Process -FilePath $misterUninstaller -ArgumentList "/VERYSILENT /SUPPRESSMSGBOXES /NORESTART" -Wait -PassThru
    Start-Sleep -Seconds 2
}

$misterDir = "C:\Program Files\Mister Download Manager"
if (Test-Path $misterDir) {
    Write-Host "Removing leftover directory: $misterDir"
    Remove-Item -LiteralPath $misterDir -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Host "=== 3. Cleaning up Mister registry keys ===" -ForegroundColor Cyan
$regRoots = @("HKLM", "HKCU")
$regPaths = @(
    "Software\Google\Chrome\NativeMessagingHosts\com.misterdownloadmanager.host",
    "Software\Mozilla\NativeMessagingHosts\com.misterdownloadmanager.host",
    "Software\Chromium\NativeMessagingHosts\com.misterdownloadmanager.host",
    "Software\Microsoft\Edge\NativeMessagingHosts\com.misterdownloadmanager.host",
    "Software\BraveSoftware\Brave-Browser\NativeMessagingHosts\com.misterdownloadmanager.host"
)

foreach ($root in $regRoots) {
    foreach ($p in $regPaths) {
        $key = "$root\$p"
        reg delete $key /f 2>$null
    }
}

reg delete "HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\{667DB4D8-32C2-4487-A0E5-5825BF29778C}_is1" /f 2>$null

Write-Host "=== 4. Cleaning up shortcuts ===" -ForegroundColor Cyan
Get-ChildItem -Path "$env:ProgramData\Microsoft\Windows\Start Menu\Programs" -Filter "*Mister*" -Recurse -ErrorAction SilentlyContinue | Remove-Item -Force
Get-ChildItem -Path ([Environment]::GetFolderPath("Desktop")) -Filter "*Mister*" -ErrorAction SilentlyContinue | Remove-Item -Force
Get-ChildItem -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs" -Filter "*Mister*" -Recurse -ErrorAction SilentlyContinue | Remove-Item -Force

Write-Host "=== 5. Migrating AppData to Luna Download Manager ===" -ForegroundColor Cyan
$oldAppData = "$env:APPDATA\misterdownloadmanager"
$newAppData = "$env:APPDATA\lunadownloadmanager"

if (Test-Path $oldAppData) {
    if (-not (Test-Path $newAppData)) {
        New-Item -ItemType Directory -Force $newAppData | Out-Null
    }
    Get-ChildItem -Path $oldAppData -Recurse | ForEach-Object {
        $dest = Join-Path $newAppData $_.FullName.Substring($oldAppData.Length)
        if ($_.PSIsContainer) {
            New-Item -ItemType Directory -Force $dest -ErrorAction SilentlyContinue | Out-Null
        } else {
            Copy-Item -LiteralPath $_.FullName -Destination $dest -Force -ErrorAction SilentlyContinue
        }
    }
    Remove-Item -LiteralPath $oldAppData -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "Migrated AppData from $oldAppData to $newAppData"
}

Write-Host "=== 6. Installing Luna Download Manager ===" -ForegroundColor Cyan
$lunaDir = "C:\Program Files\Luna Download Manager"
$root = "C:\Users\vlad\Downloads\RivaDownloadManager"
$binDir = if (Test-Path "$root\target\release\lunadownloadmanager.exe") { "$root\target\release" } else { "$root\target\debug" }
$extDir = "$root\target\extension"

New-Item -ItemType Directory -Force "$lunaDir\extension" | Out-Null

$exes = @("lunadownloadmanager.exe", "lunadownloadmanager-nmhost.exe", "lunadownloadmanager-cli.exe", "lunadownloadmanager-mcp.exe")
foreach ($exe in $exes) {
    $src = Join-Path $binDir $exe
    if (Test-Path $src) {
        Copy-Item -LiteralPath $src -Destination (Join-Path $lunaDir $exe) -Force
        Write-Host "Installed: $exe (from $binDir)"
    } else {
        Write-Warning "Binary not found: $src"
    }
}

if (Test-Path "$extDir\chromium") {
    Copy-Item -LiteralPath "$extDir\chromium" -Destination "$lunaDir\extension\chromium" -Recurse -Force
}
if (Test-Path "$extDir\luna-firefox.xpi") {
    Copy-Item -LiteralPath "$extDir\luna-firefox.xpi" -Destination "$lunaDir\extension\luna-firefox.xpi" -Force
}
if (Test-Path "$extDir\luna-chromium.zip") {
    Copy-Item -LiteralPath "$extDir\luna-chromium.zip" -Destination "$lunaDir\extension\luna-chromium.zip" -Force
}

Copy-Item -LiteralPath "$root\LICENSE" -Destination "$lunaDir\LICENSE.txt" -Force -ErrorAction SilentlyContinue
Copy-Item -LiteralPath "$root\ATTRIBUTION.md" -Destination "$lunaDir\ATTRIBUTION.txt" -Force -ErrorAction SilentlyContinue

Write-Host "=== 7. Registering Native Messaging Hosts ===" -ForegroundColor Cyan
$nmhost = "$lunaDir\lunadownloadmanager-nmhost.exe"
if (Test-Path $nmhost) {
    & $nmhost --register --all-users
    & $nmhost --register
}

Write-Host "=== 8. Creating Shortcuts ===" -ForegroundColor Cyan
$wsh = New-Object -ComObject WScript.Shell
$lnkPath = "$env:ProgramData\Microsoft\Windows\Start Menu\Programs\Luna Download Manager.lnk"
$lnk = $wsh.CreateShortcut($lnkPath)
$lnk.TargetPath = "$lunaDir\lunadownloadmanager.exe"
$lnk.Description = "Luna Download Manager"
$lnk.WorkingDirectory = $lunaDir
$lnk.Save()
Write-Host "Created shortcut: $lnkPath"

$deskPath = [System.IO.Path]::Combine([Environment]::GetFolderPath("Desktop"), "Luna Download Manager.lnk")
$deskLnk = $wsh.CreateShortcut($deskPath)
$deskLnk.TargetPath = "$lunaDir\lunadownloadmanager.exe"
$deskLnk.Description = "Luna Download Manager"
$deskLnk.WorkingDirectory = $lunaDir
$deskLnk.Save()
Write-Host "Created desktop shortcut: $deskPath"

Write-Host "=== 9. Updating Scheduled Task ===" -ForegroundColor Cyan
schtasks /delete /tn "PreviewMDM" /f 2>$null
schtasks /create /tn "PreviewLuna" /tr "`"$lunaDir\lunadownloadmanager.exe`"" /sc ONCE /st 23:59 /f 2>$null

Write-Host "=== DONE: Luna Download Manager installed successfully! ===" -ForegroundColor Green
Stop-Transcript
