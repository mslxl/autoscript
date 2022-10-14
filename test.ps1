cargo build
$env:RUST_BACKTRACE=1
Get-ChildItem .\sample\*.aa | ForEach-Object {
    .\target\debug\autoscript.exe $_.FullName -i
    if ($LastExitCode -ne 0) {
        Write-Output "Test Failure, file:"
        Write-Output $_.FullName
        exit $LastExitCode
    }
}
Write-Output "Test Success"
exit 0