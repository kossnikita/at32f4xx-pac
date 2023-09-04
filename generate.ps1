New-item -ItemType Directory . -Name svd -ErrorAction Ignore
Get-ChildItem "." -Filter *.pack | 
Foreach-Object {
    $pack = $_
    Expand-Archive $pack
    Push-Location $pack.BaseName
    Get-ChildItem "./SVD/" -Filter *.svd | 
    Foreach-Object {
        $svd = $_
        # Micro-patch
        (Get-Content $svd).replace('<access>read-write </access>', '<access>read-write</access>') | Set-Content $svd
        Copy-Item $svd -Destination ../svd | Out-Null
    }
    Pop-Location
    Remove-Item $pack.BaseName -Recurse
}
Get-ChildItem "./patches/" | 
Foreach-Object {
    svdtools patch $_
}
Get-ChildItem "./svd/" -Filter *.svd.patched | 
Foreach-Object {
    $svd = $_
    $dirName = $svd.BaseName.replace('xx_v2.svd','').ToLower()
    New-item -ItemType Directory .. -Name $dirName -ErrorAction Ignore
    Copy-Item $svd -Destination ../$dirName | Out-Null
    Push-Location ../$dirName
    cargo init
    svd2rust -m -g -s --pascal_enum_values --max_cluster_size --atomics --atomics_feature atomics -i $svd.Name
    if ($LastExitCode) {
        Pop-Location
        return;
    }
    # Remove-Item $svd.Name
    form -i mod.rs -o src/
    Remove-Item mod.rs
    cargo fmt
    Copy-Item device.x -Destination src | Out-Null
    Rename-Item -Path src/lib.rs -NewName mod.rs
    $path = ($PSScriptRoot + "/src/" + $dirName)
    Copy-Item -Path src/* -Destination $path -Recurse -Force | Out-Null
    Pop-Location
    Remove-Item ../$dirName -Recurse -Force
}
# Remove-Item svd -Recurse