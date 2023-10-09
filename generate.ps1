New-item -ItemType Directory . -Name svd -ErrorAction Ignore | Out-Null
Get-ChildItem ./PACKs/Keil5/ -Filter *.pack | 
Foreach-Object -Parallel {
    $pack = $_
    Expand-Archive $pack
    Push-Location $pack.BaseName
    Get-ChildItem ./SVD/ -Filter *.svd | 
    Foreach-Object {
        $svd = $_
        $svd_path = "../svd/" + $svd.Name.replace('xx_v2','').ToLower()
        Copy-Item $svd -Destination $svd_path
    }
    Pop-Location
    Remove-Item $pack.BaseName -Recurse
}
Get-ChildItem ./patches/ -Filter *.yaml | 
Foreach-Object -Parallel {
    svdtools patch $_
}
Get-ChildItem ./svd/ -Filter *.svd.patched | 
Foreach-Object -Parallel {
    $svd = $_
    $dirName = $svd.BaseName.replace('.svd','').ToLower()
    New-item -ItemType Directory . -Name $dirName -ErrorAction Ignore | Out-Null
    $svd_name = $svd.BaseName.ToLower()
    Copy-Item $svd -Destination ./$dirName/$svd_name
    Push-Location ./$dirName
    cargo init -q
    svd2rust -m -g -s --pascal_enum_values --max_cluster_size --atomics --atomics_feature atomics --impl_debug -l warn -i $svd_name
    if ($LastExitCode) {
        Pop-Location
        return;
    }
    form -i mod.rs -o src/
    Remove-Item mod.rs
    cargo fmt
    Copy-Item $svd_name -Destination src
    Copy-Item device.x -Destination src
    Rename-Item -Path src/lib.rs -NewName mod.rs
    $path = ("../src/" + $dirName)
    Remove-Item $path -Recurse
    Copy-Item -Path src -Destination $path -Recurse
    Pop-Location
    Remove-Item ./$dirName -Recurse -Force
}
svdtools html ./html (Get-ChildItem ./svd/ -Filter *.patched) | Out-Null
svdtools html ./html/original (Get-ChildItem ./svd/ -Filter *.svd) | Out-Null
svdtools htmlcompare ./html/comparison (Get-ChildItem ./svd/ -Filter *.patched) | Out-Null
svdtools htmlcompare ./html/original/comparison (Get-ChildItem ./svd/ -Filter *.svd) | Out-Null
$index = "./html/original/index.html"
(Get-Content $index).replace('comparisons.html', 'comparison/index.html') | Set-Content $index
$index = "./html/index.html"
(Get-Content $index).replace('comparisons.html', 'comparison/index.html') | Set-Content $index
# Remove-Item svd -Recurse