# FastZip Windows Install - Add context menu
# Run as current user, no admin required

param(
    [string]$FastZipPath = "",
    [switch]$Uninstall,
    [switch]$RestartExplorer
)

$ArchiveExtensions = @(".zip", ".7z", ".rar", ".tar", ".gz", ".tgz", ".xz", ".bz2", ".zst")
$BaseKey = "HKCU:\Software\Classes"

# AppliesTo filter - show menu only for archive extensions (more reliable than ProgID)
$AppliesTo = ($ArchiveExtensions | ForEach-Object { $_ }) -join " OR "

function Get-FastZipExe {
    if ($FastZipPath -and (Test-Path $FastZipPath)) {
        return (Resolve-Path $FastZipPath).Path
    }
    $inPath = Get-Command fastzip -ErrorAction SilentlyContinue
    if ($inPath) { return $inPath.Source }
    $default = "$env:LOCALAPPDATA\fastzip\fastzip.exe"
    if (Test-Path $default) { return $default }
    Write-Host "FastZip not found. Use -FastZipPath to specify path."
    Write-Host "  .\install-windows.ps1 -FastZipPath 'C:\path\to\fastzip.exe'"
    exit 1
}

function Add-ContextMenu {
    param([string]$ExePath)
    $exeArg = "`"$ExePath`""
    $shellKey = "$BaseKey\*\shell"
    if (!(Test-Path $shellKey)) { New-Item -Path $shellKey -Force | Out-Null }
    
    # Extract Here (flat)
    $extractKey = "$shellKey\FastZipExtract"
    New-Item -Path $extractKey -Force | Out-Null
    Set-ItemProperty -Path $extractKey -Name "(Default)" -Value "FastZip: Extract Here" -Type String
    Set-ItemProperty -Path $extractKey -Name "AppliesTo" -Value $AppliesTo -Type String
    New-Item -Path "$extractKey\command" -Force | Out-Null
    Set-ItemProperty -Path "$extractKey\command" -Name "(Default)" -Value "$exeArg x `"%1`" -f -q" -Type String
    
    # Smart Extract
    $smartKey = "$shellKey\FastZipSmart"
    New-Item -Path $smartKey -Force | Out-Null
    Set-ItemProperty -Path $smartKey -Name "(Default)" -Value "FastZip: Smart Extract" -Type String
    Set-ItemProperty -Path $smartKey -Name "AppliesTo" -Value $AppliesTo -Type String
    New-Item -Path "$smartKey\command" -Force | Out-Null
    Set-ItemProperty -Path "$smartKey\command" -Name "(Default)" -Value "$exeArg x `"%1`" -s -q" -Type String
    Write-Host "FastZip context menu added (Extract Here, Smart Extract)"
    Write-Host ""
    Write-Host "Windows 11: Right-click archive -> Show more options -> FastZip menu"
    Write-Host "Or restart Explorer: taskkill /f /im explorer.exe; Start-Process explorer"
}

function Remove-ContextMenu {
    $shellKey = "$BaseKey\*\shell"
    $extractKey = "$shellKey\FastZipExtract"
    $smartKey = "$shellKey\FastZipSmart"
    if (Test-Path $extractKey) { Remove-Item -Path $extractKey -Recurse -Force }
    if (Test-Path $smartKey) { Remove-Item -Path $smartKey -Recurse -Force }
    # Remove old keys from previous script version
    foreach ($progId in @("CompressedFolder") + $ArchiveExtensions) {
        $oldExtract = "$BaseKey\$progId\shell\FastZipExtract"
        $oldSmart = "$BaseKey\$progId\shell\FastZipSmart"
        if (Test-Path $oldExtract) { Remove-Item -Path $oldExtract -Recurse -Force }
        if (Test-Path $oldSmart) { Remove-Item -Path $oldSmart -Recurse -Force }
    }
    Write-Host "FastZip context menu removed"
}

if ($Uninstall) {
    Remove-ContextMenu
    exit 0
}

$exe = Get-FastZipExe
Add-ContextMenu -ExePath $exe

if ($RestartExplorer) {
    Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
    Start-Sleep -Seconds 1
    Start-Process explorer
    Write-Host "Explorer restarted."
}
