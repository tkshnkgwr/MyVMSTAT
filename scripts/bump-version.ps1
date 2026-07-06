param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# セマンティックバージョン (X.Y.Z) のバリデーション
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

Write-Host "Bumping version to $NewVersion..."
$utf8 = New-Object System.Text.UTF8Encoding -ArgumentList $false

# 1. Cargo.toml の更新
$cargoPath = "Cargo.toml"
if (Test-Path $cargoPath) {
    Write-Host "Updating $cargoPath..."
    $lines = [System.IO.File]::ReadAllLines($cargoPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match '^version\s*=\s*"[^"]+"') {
            $lines[$i] = "version = `"$NewVersion`""
            break
        }
    }
    [System.IO.File]::WriteAllLines($cargoPath, $lines, $utf8)
} else {
    Write-Warning "$cargoPath not found."
}

# 2. docs/TEST_REPORT.md の更新
$testPath = "docs/TEST_REPORT.md"
if (Test-Path $testPath) {
    Write-Host "Updating $testPath..."
    $content = [System.IO.File]::ReadAllText($testPath, $utf8)
    $newContent = $content -replace '正しいバージョン（[^）]+）が表示されることを確認', "正しいバージョン（$NewVersion）が表示されることを確認"
    [System.IO.File]::WriteAllText($testPath, $newContent, $utf8)
} else {
    Write-Warning "$testPath not found."
}

Write-Host "Version bump completed successfully!"
