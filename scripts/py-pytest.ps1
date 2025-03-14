$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
Set-Location ./src/python

if ($env:CI) {
    $markers = "not gpu"
}
else {
    $markers = ""
}
# run test
pixi run pytest `
    ./tests `
    -m $markers `
    --durations=10 `
    --junitxml=tests_report/junit.xml `
    -o junit_family=legacy `
    --cov `
    --cov-report term `
    --cov-report=xml:tests_report/coverage.xml `
    --cov-report=html:tests_report/htmlcov
Set-Location $ROOT
