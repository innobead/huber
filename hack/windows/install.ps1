$tag = (Invoke-WebRequest "https://api.github.com/repos/innobead/huber/releases/latest" | ConvertFrom-Json).tag_name
$url = "https://github.com/innobead/huber/releases/download/$tag/huber-windows-amd64.exe"

Write-Host "Downloading the latest huber release: $tag from $url"

New-Item ~/.huber/bin -ItemType directory
Invoke-WebRequest -Uri $url -OutFile ~/.huber/bin/huber.exe
$path = (Resolve-Path ~/.huber/bin/huber.exe)

Write-Host "huber installed in $path!"