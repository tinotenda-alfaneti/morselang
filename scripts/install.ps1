# Morse Language Installer for Windows
# Run this script as Administrator if you want to install system-wide to C:\Windows

Write-Host "Building morse (Morse language compiler/interpreter)..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
$ditExe = "target\release\morse.exe"

if (-not (Test-Path $ditExe)) {
    Write-Host "Binary not found at $ditExe" -ForegroundColor Red
    exit 1
}

$ditPath = (Get-Location).Path + "\target\release"
$installed = $false

Write-Host "Attempting system-wide installation..." -ForegroundColor Cyan
$installPath = "C:\Windows\morse.exe"

try {
    Copy-Item $ditExe $installPath -Force -ErrorAction Stop
    Write-Host "System-wide installation successful!" -ForegroundColor Green
    Write-Host "Installed to: $installPath" -ForegroundColor Green
    $installed = $true
} catch {
    Write-Host "Cannot install to C:\Windows (requires Administrator)" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Installing to user PATH instead..." -ForegroundColor Cyan
    
    # Add to user PATH permanently
    $currentUserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($currentUserPath -notlike "*$ditPath*") {
        $newUserPath = $currentUserPath + ";" + $ditPath
        [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
        Write-Host "Added to user PATH: $ditPath" -ForegroundColor Green
        Write-Host ""
        Write-Host "Note: You may need to restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    } else {
        Write-Host "Already in PATH: $ditPath" -ForegroundColor Green
    }
    
    $env:PATH += ";$ditPath"
    Write-Host "Added to current session PATH" -ForegroundColor Green
    $installed = $true
}

if (-not $installed) {
    Write-Host "Installation failed!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Try these commands:" -ForegroundColor Cyan
Write-Host "  morse run example.mo" -ForegroundColor White
Write-Host "  morse build example.mo -o hello.exe" -ForegroundColor White
Write-Host "  .\hello.exe" -ForegroundColor White
Write-Host ""
Write-Host "Binary location: $ditPath\morse.exe" -ForegroundColor DarkGray
