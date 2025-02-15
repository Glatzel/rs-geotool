Set-Location $PSScriptRoot
Set-Location ..
if($env:CI){
    cargo doc --no-deps --all
}
else{
    & $PSScriptRoot/set-env.ps1
    pixi run cargo doc --no-deps --all
}

Remove-Item ./dist/rust-doc.zip -Force -ErrorAction SilentlyContinue
New-Item ./dist -ItemType Directory -ErrorAction SilentlyContinue
Compress-Archive ./target/doc "./dist/rust-doc.zip"
