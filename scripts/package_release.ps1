# =====================================================================
#  🔱 ITZAMNA RELEASE PACKAGING SCRIPT 🔱
#  Compiles release binaries for Desktop & Backend, creates zip & installer
# =====================================================================

$ErrorActionPreference = "Stop"

Write-Host "`n🔱 [1/4] Compiling Itzamna Desktop in --release mode..." -ForegroundColor Cyan
cargo build --release --bin itzamna-desktop --manifest-path itzamna-desktop/Cargo.toml

Write-Host "`n🌙 [2/4] Packaging Ixchel Backend runnable JAR..." -ForegroundColor Cyan
& "C:\Program Files\scala-cli-x86_64-pc-win32\scala-cli.exe" --power package --server=false ixchel -o ixchel.jar -f

Write-Host "`n📦 [3/4] Staging Distribution Directory..." -ForegroundColor Cyan
$distDir = "dist/windows"
New-Item -ItemType Directory -Force -Path $distDir | Out-Null
Copy-Item "target/release/itzamna-desktop.exe" "$distDir/" -Force
Copy-Item "itzamna-desktop/assets" "$distDir/assets" -Recurse -Force
Copy-Item "ixchel.jar" "$distDir/" -Force

$zipPath = "dist/itzamna-windows-v0.1.0.zip"
if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
Compress-Archive -Path "$distDir/*" -DestinationPath $zipPath -Force
Write-Host "✅ Created portable archive: $zipPath" -ForegroundColor Green

Write-Host "`n🏛️ [4/4] Checking Inno Setup Compiler..." -ForegroundColor Cyan
$iscc = Get-Command iscc -ErrorAction SilentlyContinue
if ($iscc) {
    Write-Host "Found ISCC at $($iscc.Source). Compiling setup installer..." -ForegroundColor Green
    & iscc installer/itzamna.iss
} else {
    Write-Host "ℹ️ Inno Setup compiler (iscc) not found in PATH. Portable zip generated in dist/." -ForegroundColor Yellow
}

Write-Host "`n🎉 Consecration complete! Distributable packages ready in dist/`n" -ForegroundColor Green
