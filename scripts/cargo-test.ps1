Set-Location $PSScriptRoot
Set-Location ..
New-Item ./target/llvm-cov-target/debug -ItemType Directory -ErrorAction SilentlyContinue
& $PSScriptRoot/set-env.ps1

Write-Output "::group::nextest"
pixi run cargo +nightly llvm-cov --no-report --all-features --workspace --branch nextest
$code = $LASTEXITCODE
Write-Output "::endgroup::"

Write-Output "::group::doctest"
pixi run cargo +nightly llvm-cov --no-report --all-features --workspace --branch --doc
$code = $code + $LASTEXITCODE
Write-Output "::endgroup::"

Write-Output "::group::report"
pixi run cargo +nightly llvm-cov report
Write-Output "::endgroup::"

Write-Output "::group::lcov"
if ( $env:CI ) {
    pixi run cargo +nightly llvm-cov report --lcov --output-path lcov.info
}
Write-Output "::endgroup::"

Write-Output "::group::result"
$code = $code + $LASTEXITCODE
if ($code -ne 0) {
    Write-Output "Test failed."
}
else {
    Write-Output "Test successed."
}
Write-Output "::endgroup::"
exit $code
