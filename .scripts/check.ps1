#!/usr/bin/env pwsh

New-Item -ItemType Directory -Force temp | Out-Null
Remove-Item -Recurse ./temp/* -Force -ErrorAction SilentlyContinue

foreach ($file in Get-ChildItem) {
    if (-not(Test-Path (Join-Path $file.FullName "cargo-generate.toml"))) {
        continue
    }
    Push-Location temp
    $projectName = "$($file.BaseName)_test_proj"
    cargo-generate generate -p $file.FullName -n $projectName --template-values-file ../.github/workflows/template_values.toml
    Push-Location $projectName
    cargo make check --allow-no-vcs
    Pop-Location
    Pop-Location
}