Set-Location $PSScriptRoot
Set-Location ..
$pkg_config = Resolve-Path .pixi/envs/dev/Library/bin
$dll_path = Resolve-Path vcpkg_deps/vcpkg_installed/x64-windows/bin
$env:PATH = $env:PATH + ";$pkg_config;$dll_path"
$env:PKG_CONFIG_PATH = Resolve-Path vcpkg_deps/vcpkg_installed/x64-windows/lib/pkgconfig

if ( $env:CI ) {
    Write-Output "::group::nextest"
    pixi run cargo llvm-cov nextest
    Write-Output "::endgroup::"

    # Write-Output "::group::doctest"
    # pixi run cargo llvm-cov --doc
    # Write-Output "::endgroup::"

    Write-Output "::group::cov"
    pixi run cargo llvm-cov --doctests --all-features --lcov --output-path lcov.info
    Write-Output "::endgroup::"
}
else {
    pixi run cargo llvm-cov --all-features
}
