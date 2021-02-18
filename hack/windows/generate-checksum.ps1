foreach($file in Get-ChildItem $args[0])
{
    if (Test-Path -Path "$file" -Include "*.exe" -PathType Leaf)
    {
        $hash=(Get-FileHash -Algorithm SHA256 $file | Select-Object -ExpandProperty Hash)
        $hash | Out-File "$file.sha256"
    }
}
