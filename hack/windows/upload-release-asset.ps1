$tag_name=((hub release -d) -split '\n')[0]

foreach($file in Get-ChildItem $args[0])
{
    if (Test-Path -Path "$file" -Include "*.exe" -PathType Leaf)
    {
        hub release edit -d "$file" -m "" "$tag_name"
    }
}
