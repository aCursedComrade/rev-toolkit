# retrieves exports of specified DLL using `dumpbin.exe`
# and formats the output for usage with function forwarding

if ($null -eq (Get-Command "dumpbin.exe" -ErrorAction SilentlyContinue)) {
    Write-Host "[!] This script requires 'dumpbin.exe' to run. It is included in MSVC build tools." -ForegroundColor Red
    Exit
}

if (1 -gt $args.Length) {
    Write-Host "[!] Missing arguments. Provide the path to DLL to extract information." -ForegroundColor Red
    Write-Host "`t.\$($MyInvocation.MyCommand.Name) \path\to\dll\file.dll" -ForegroundColor Red
    Exit
}

$path = $args[0]
$outpath = "$env:TEMP\exports.txt"
dumpbin /nologo /nopdb /out:$outpath /exports $path

$exports = Get-Content $outpath
# the following can break in the future
$totalnames = ($exports[12].Trim() -split '\s+')[0]
$exports = $exports | Select-Object -Skip 16 | Select-Object -SkipLast 8

Write-Host "[!] Make sure to compare dumpbin output and formatted outputs for extra safety" -BackgroundColor DarkYellow -ForegroundColor Black
Write-Host "[*] dumpbin reported $totalnames export names | Extracted list has $($exports.Length) exports" -ForegroundColor Yellow
Write-Host "[+] Found $($exports.Length) exports (written to: $outpath)" -ForegroundColor Blue

$namelist = @()
$exports | ForEach-Object -Process {
    $export = ($_.Trim() -split '\s+')[3]
    $namelist += $export
}

# create the module definition file
$definition = "$env:TEMP\forward.def.txt"
if (Test-Path $definition) { Remove-Item $definition }
Add-Content -Path $definition "; Update the given placeholders and add/remove exports to your case"
# Add-Content -Path $definition "LIBRARY YourModuleName"
Add-Content -Path $definition "EXPORTS"
$namelist | ForEach-Object -Process {
    Add-Content -Path $definition "`t$_=YourTargetModule.$_"
}
Write-Host "[+] Module definition file written to $definition" -ForegroundColor Blue

# create the rust module with dummy functions
$modfile = "$env:TEMP\forward.rs.txt"
if (Test-Path $modfile) { Remove-Item $modfile }
Add-Content -Path $modfile "#![allow(non_snake_case)]"
$namelist | ForEach-Object -Process {
    Add-Content -Path $modfile "#[no_mangle]`nfn $_() {}"
}
Write-Host "[+] Rust module written to $modfile" -ForegroundColor Blue

Write-Host "[+] Done! Please, double check the output files for any anomalies." -ForegroundColor Green
