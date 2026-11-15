$ErrorActionPreference = "Continue"

$installDir = "$env:LOCALAPPDATA\Programs\Luna Download Manager"
$root = "C:\Users\vlad\Downloads\RivaDownloadManager"
$binDir = "$root\target\release"
$extDir = "$root\target\extension"

Write-Host "=== 1. Terminating running processes ===" -ForegroundColor Cyan
Stop-Process -Name 'misterdownloadmanager' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'misterdownloadmanager-nmhost' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'lunadownloadmanager' -Force -ErrorAction SilentlyContinue
Stop-Process -Name 'lunadownloadmanager-nmhost' -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1

Write-Host "=== 2. Creating directory structure ===" -ForegroundColor Cyan
New-Item -ItemType Directory -Force "$installDir\extension" | Out-Null

Write-Host "=== 3. Copying binaries ===" -ForegroundColor Cyan
$exes = @("lunadownloadmanager.exe", "lunadownloadmanager-nmhost.exe", "lunadownloadmanager-cli.exe", "lunadownloadmanager-mcp.exe")
foreach ($exe in $exes) {
    Copy-Item -LiteralPath (Join-Path $binDir $exe) -Destination (Join-Path $installDir $exe) -Force
    Write-Host "Copied binary: $exe"
}

Write-Host "=== 4. Copying extensions ===" -ForegroundColor Cyan
Copy-Item -LiteralPath "$extDir\chromium" -Destination "$installDir\extension\chromium" -Recurse -Force
Copy-Item -LiteralPath "$extDir\luna-firefox.xpi" -Destination "$installDir\extension\luna-firefox.xpi" -Force
Copy-Item -LiteralPath "$extDir\luna-chromium.zip" -Destination "$installDir\extension\luna-chromium.zip" -Force
Copy-Item -LiteralPath "$root\LICENSE" -Destination "$installDir\LICENSE.txt" -Force -ErrorAction SilentlyContinue
Copy-Item -LiteralPath "$root\ATTRIBUTION.md" -Destination "$installDir\ATTRIBUTION.txt" -Force -ErrorAction SilentlyContinue

Write-Host "=== 5. Registering Native Messaging Host for Luna ===" -ForegroundColor Cyan
$nmhost = "$installDir\lunadownloadmanager-nmhost.exe"
& $nmhost --register

Write-Host "=== 6. Migrating AppData from misterdownloadmanager to lunadownloadmanager ===" -ForegroundColor Cyan
$oldApp = "$env:APPDATA\misterdownloadmanager"
$newApp = "$env:APPDATA\lunadownloadmanager"
if (Test-Path $oldApp) {
    if (-not (Test-Path $newApp)) {
        New-Item -ItemType Directory -Force $newApp | Out-Null
    }
    Get-ChildItem -Path $oldApp -Recurse | ForEach-Object {
        $dest = Join-Path $newApp $_.FullName.Substring($oldApp.Length)
        if ($_.PSIsContainer) {
            New-Item -ItemType Directory -Force $dest -ErrorAction SilentlyContinue | Out-Null
        } else {
            Copy-Item -LiteralPath $_.FullName -Destination $dest -Force -ErrorAction SilentlyContinue
        }
    }
    Remove-Item -LiteralPath $oldApp -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "AppData migrated and old misterdownloadmanager removed."
}

Write-Host "=== 7. Cleaning up old shortcuts and creating Luna shortcuts ===" -ForegroundColor Cyan
Get-ChildItem -Path ([Environment]::GetFolderPath('Desktop')) -Filter "*Mister*" -ErrorAction SilentlyContinue | Remove-Item -Force
Get-ChildItem -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs" -Filter "*Mister*" -Recurse -ErrorAction SilentlyContinue | Remove-Item -Force

$wsh = New-Object -ComObject WScript.Shell
$deskLnkPath = [System.IO.Path]::Combine([Environment]::GetFolderPath('Desktop'), "Luna Download Manager.lnk")
$deskLnk = $wsh.CreateShortcut($deskLnkPath)
$deskLnk.TargetPath = "$installDir\lunadownloadmanager.exe"
$deskLnk.Description = "Luna Download Manager"
$deskLnk.WorkingDirectory = $installDir
$deskLnk.Save()

$startLnkPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Luna Download Manager.lnk"
$startLnk = $wsh.CreateShortcut($startLnkPath)
$startLnk.TargetPath = "$installDir\lunadownloadmanager.exe"
$startLnk.Description = "Luna Download Manager"
$startLnk.WorkingDirectory = $installDir
$startLnk.Save()

Write-Host "=== 8. Updating Task Scheduler Preview Task ===" -ForegroundColor Cyan
schtasks /delete /tn "PreviewMDM" /f 2>$null
schtasks /delete /tn "PreviewLuna" /f 2>$null
$action = New-ScheduledTaskAction -Execute "$installDir\lunadownloadmanager.exe" -WorkingDirectory $installDir
$trigger = New-ScheduledTaskTrigger -Once -At '23:59'
Register-ScheduledTask -TaskName "PreviewLuna" -Action $action -Trigger $trigger -Force
Start-ScheduledTask -TaskName "PreviewLuna"

Write-Host "=== SUCCESS: Luna Download Manager installed and running! ===" -ForegroundColor Green
