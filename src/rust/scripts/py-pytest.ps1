Set-Location $PSScriptRoot
Set-Location ..

# run test
pixi run pytest `
    ./crates/pyxis-py/tests `
    --durations=10 `
    --junitxml=tests_report/junit.xml `
    -o junit_family=legacy `
    --cov `
    --cov-report term `
    --cov-report=xml:tests_report/coverage.xml `
    --cov-report=html:tests_report/htmlcov
    Set-Location $PSScriptRoot
    Set-Location ../../../
