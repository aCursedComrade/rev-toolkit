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
$outpath = Split-Path $MyInvocation.MyCommand.Source.ToString() -Parent
$exportout = "$outpath\exports.txt"
dumpbin /nologo /out:$exportout /exports $path

$exports = Get-Content $exportout
# the following can break in the future
$totalfuncs = ($exports[11].Trim() -split '\s+')[0]
$totalnames = ($exports[12].Trim() -split '\s+')[0]
$exports = $exports | Select-Object -Skip 16 | Select-Object -First $totalfuncs

Write-Host "[*] dumpbin reported $totalfuncs functions with $totalnames named exports | Extracted list has $($exports.Length) exports" -ForegroundColor Yellow
Write-Host "[+] Found $($exports.Length) exports (written to: $exportout)" -ForegroundColor Blue

# create the module definition file
$definition = "$outpath\forward.def.txt"
if (Test-Path $definition) { Remove-Item $definition }
Add-Content -Path $definition "; Update the given placeholders and add/remove exports to your case"
Add-Content -Path $definition "LIBRARY helper.dll"
Add-Content -Path $definition "EXPORTS"
$exports | ForEach-Object -Process {
    $func = ($_.Trim() -split '\s+')[3]
    Add-Content -Path $definition "`t$func=TargetModule.$func"
}
Write-Host "[+] Module definition file written to $definition" -ForegroundColor Blue

# create the rust module with dummy functions
$modfile = "$outpath\forward.rs.txt"
if (Test-Path $modfile) { Remove-Item $modfile }
Add-Content -Path $modfile "#![allow(non_snake_case)]"
$exports | ForEach-Object -Process {
    # TODO certain names have ordinals attached to them
    # in the format <function>@<ordinal>, need to consider removal?
    $func = ($_.Trim() -split '\s+')[3]
    Add-Content -Path $modfile "`n#[no_mangle]`nextern `"C`" fn $func() {}"
}
Write-Host "[+] Rust module written to $modfile" -ForegroundColor Blue

Write-Host "[+] Done! Make sure to double check the output files for any anomalies." -ForegroundColor Green
