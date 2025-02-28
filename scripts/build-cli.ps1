param (
    [ValidateSet("dist", "release", "debug")]
    [string]$config = "debug"
)

Set-Location $PSScriptRoot
Set-Location ..
& $PSScriptRoot/set-env.ps1
Remove-Item dist/cli -Recurse -ErrorAction SilentlyContinue
Remove-Item ./dist/geotool*.7z -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Build in $config mode."
function build-static {
    Write-Output "::group::Build static"
    if ($config -ne "debug") {
        pixi run cargo build --profile $config --bin geotool --features static
    }
    else {
        pixi run cargo build --bin geotool --features static
    }
    New-Item ./dist/cli/static -ItemType Directory -ErrorAction SilentlyContinue
    Copy-Item "target/$config/geotool.exe" ./dist/cli/static/geotool.exe
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on -sccUTF-8 -bb0 -bse0 -bsp2 `
        "-wdist/cli/static" -mtc=on -mta=on "dist/geotool-windows-x64-self-contained.7z" "./dist/cli/static/geotool.exe"
    Write-Output "::endgroup::"
}
function build-dynamic {
    Write-Output "::group::Build dynamic"
    if ($config -ne "debug") {
        pixi run cargo build --profile $config --bin geotool
    }
    else {
        pixi run cargo build --bin geotool
    }
    New-Item ./dist/cli/dynamic -ItemType Directory -ErrorAction SilentlyContinue
    Copy-Item "target/$config/geotool.exe" ./dist/cli/dynamic/geotool.exe
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on -sccUTF-8 -bb0 -bse0 -bsp2 `
        "-wdist/cli/dynamic" -mtc=on -mta=on "dist/geotool-windows-x64.7z" "./dist/cli/dynamic/geotool.exe"

    Copy-Item ./vcpkg_deps/vcpkg_installed/dynamic/x64-windows/bin/*.dll ./dist/cli/dynamic
    Copy-Item ./vcpkg_deps/vcpkg_installed/dynamic/x64-windows/share/proj/proj.db ./dist/cli/dynamic
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on -sccUTF-8 -bb0 -bse0 -bsp2 `
        "-wdist/cli/dynamic" -mtc=on -mta=on "dist/geotool-windows-x64-proj.7z" "./dist/cli/dynamic/*"
    Write-Output "::endgroup::"
}
build-static
build-dynamic
