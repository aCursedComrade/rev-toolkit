# simple script to run the injector based on target arch
# script assumes you are on 64bit env

if ($null -eq (Get-Command "cargo.exe" -ErrorAction SilentlyContinue)) {
    Write-Host "Cannot find 'cargo.exe' in PATH. Please install the toolchain to proceed." -ForegroundColor Red
    Exit
}

$arch = $args[0]
$injector_args = $($args | Select-Object -SkipIndex 0)

switch ($arch) {
    32 {
        cargo.exe run --profile release --target i686-pc-windows-msvc --bin dll-inject -- $injector_args
        Break
    }
    64 {
        cargo.exe run --profile release --bin dll-inject -- $injector_args
        Break
    }
    Default {
        "Invalid arguments provided"
        "ex: .\$($MyInvocation.MyCommand.Name) <32/64> <dll-injector arguments>"
    }
}
