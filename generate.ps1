New-item -ItemType Directory . -Name svd -ErrorAction Ignore
Get-ChildItem "." -Filter *.pack | 
Foreach-Object {
    $pack = $_
    Expand-Archive $pack
    Push-Location $pack.BaseName
    Get-ChildItem "./SVD/" -Filter *.svd | 
    Foreach-Object {
        $svd = $_
        Copy-Item $svd -Destination ../svd
    }
    Pop-Location
    Remove-Item $pack.BaseName -Recurse
}
Get-ChildItem "./patches/" -Filter *.yaml | 
Foreach-Object {
    svdtools patch $_
}
Get-ChildItem "./svd/" -Filter *.svd.patched | 
Foreach-Object {
    $svd = $_
    $dirName = $svd.BaseName.replace('xx_v2.svd','').ToLower()
    New-item -ItemType Directory .. -Name $dirName -ErrorAction Ignore
    Copy-Item $svd -Destination ../$dirName
    Push-Location ../$dirName
    cargo init
    svd2rust -m -g -s --pascal_enum_values --max_cluster_size --atomics --atomics_feature atomics -l warn -i $svd.Name
    if ($LastExitCode) {
        Pop-Location
        return;
    }
    # Remove-Item $svd.Name
    form -i mod.rs -o src/
    Remove-Item mod.rs
    cargo fmt
    Copy-Item device.x -Destination src
    Rename-Item -Path src/lib.rs -NewName mod.rs
    $path = ($PSScriptRoot + "/src/" + $dirName)
    Copy-Item -Path src/* -Destination $path -Recurse -Force
    Pop-Location
    Remove-Item ../$dirName -Recurse -Force
}
svdtools html ./html (Get-ChildItem ".\svd\" -Filter *.patched)
$index = "./html/index.html"
(Get-Content $index).replace('comparisons.html', 'comparison/index.html') | Set-Content $index
svdtools htmlcompare ./html/comparison (Get-ChildItem ".\svd\" -Filter *.patched)
# Remove-Item svd -Recurse