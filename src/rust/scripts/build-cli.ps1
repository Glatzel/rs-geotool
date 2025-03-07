param (
    [ValidateSet("../dist", "release", "debug")]
    [string]$config = "debug"
)

Set-Location $PSScriptRoot
Set-Location ..
& $PSScriptRoot/set-env.ps1
Remove-Item ../../dist/cli -Recurse -ErrorAction SilentlyContinue
Remove-Item ../../dist/pyxis*.7z -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "Build in $config mode."

if ($IsWindows) {
    Write-Output "::group::Build static"
    & $PSScriptRoot/set-env.ps1 -link static
    if ($config -ne "debug") {
        cargo build --profile $config -p pyxis-cli --features static
    }
    else {
        cargo build -p pyxis-cli --features static
    }
    New-Item ../../dist/cli/static -ItemType Directory -ErrorAction SilentlyContinue
    Copy-Item "target/$config/pyxis.exe" ../../dist/cli/static/pyxis.exe
    Write-Output "::endgroup::"

    Write-Output "::group::Pack pyxis-windows-x64-self-contained.7z"
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on `
        "../../dist/pyxis-cli-windows-x64-self-contained.7z" "../../dist/cli/static/pyxis.exe"
    Write-Output "::endgroup::"

    Write-Output "::group::Build dynamic"
    & $PSScriptRoot/set-env.ps1 -link dynamic
    if ($config -ne "debug") {
        cargo build --profile $config -p pyxis-cli
    }
    else {
        cargo build -p pyxis-cli
    }
    New-Item ../../dist/cli/dynamic -ItemType Directory -ErrorAction SilentlyContinue
    Copy-Item "target/$config/pyxis.exe" ../../dist/cli/dynamic/pyxis.exe
    Write-Output "::endgroup::"

    Write-Output "::group::Pack pyxis-windows-x64.7z"
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on `
        "../../dist/pyxis-cli-windows-x64.7z" "../../dist/cli/dynamic/pyxis.exe"
    Write-Output "::endgroup::"

    Write-Output "::group::Pack pyxis-windows-x64-proj.7z.7z"
    Copy-Item ../../vcpkg_deps/vcpkg_installed/dynamic/x64-windows/bin/*.dll ../../dist/cli/dynamic
    Copy-Item ../../vcpkg_deps/vcpkg_installed/dynamic/x64-windows/share/proj/proj.db ../../dist/cli/dynamic
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on `
        "../../dist/pyxis-cli-windows-x64-proj.7z" "../../dist/cli/dynamic/*"
    Write-Output "::endgroup::"
}
elseif ($IsLinux) {
    Write-Output "::group::Build static"
    & $PSScriptRoot/set-env.ps1 -link static
    if ($config -ne "debug") {
        cargo build --profile $config -p pyxis-cli --features static
    }
    else {
        cargo build -p pyxis-cli --features static
    }
    New-Item ../../dist/cli/static -ItemType Directory -ErrorAction SilentlyContinue
    Write-Output "::endgroup::"

    Write-Output "::group::Pack pyxis-linux-x64-self-contained.7z"
    Copy-Item "target/$config/pyxis" ../../dist/cli/static
    7z a -t7z -m0=LZMA2 -mmt=on -mx9 -md=4096m -mfb=273 -ms=on -mqs=on `
        "../../dist/pyxis-cli-linux-x64-self-contained.7z" "../../dist/cli/static/*"
    Write-Output "::endgroup::"
}
else {
    Write-Error "Unsupported system $os"
    exit 1
}
Set-Location $PSScriptRoot
Set-Location ../../../
