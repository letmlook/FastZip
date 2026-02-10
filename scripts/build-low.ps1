# 低资源编译：限制 CPU/内存，降低进程优先级
# 用法: .\scripts\build-low.ps1 build
#       .\scripts\build-low.ps1 build -p fastzip-cli --release

$env:CARGO_BUILD_JOBS = "1"
$env:RUSTFLAGS = "-C codegen-units=1"

$cargoArgs = $args
if ($cargoArgs.Count -eq 0) {
    $cargoArgs = @("build")
}

$proc = Start-Process -FilePath "cargo" -ArgumentList $cargoArgs -PassThru -NoNewWindow
$proc.PriorityClass = [System.Diagnostics.ProcessPriorityClass]::BelowNormal
$proc.WaitForExit()
exit $proc.ExitCode
