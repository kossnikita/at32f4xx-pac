New-item -ItemType Directory . -Name svd -ErrorAction Ignore | Out-Null
Get-ChildItem ./PACKs/Keil5/ -Filter *.pack | 
Foreach-Object -Parallel {
    $pack = $_
    Expand-Archive $pack
    Push-Location $pack.BaseName
    Get-ChildItem ./SVD/ -Filter *.svd | 
    Foreach-Object {
        $svd = $_
        Copy-Item $svd -Destination ../svd
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
    $dirName = $svd.BaseName.replace('xx_v2.svd','').ToLower()
    New-item -ItemType Directory . -Name $dirName -ErrorAction Ignore | Out-Null
    $svd_name = ($svd.BaseName.replace('xx_v2.svd','').ToLower() + ".svd")
    Copy-Item $svd -Destination ./$dirName/$svd_name
    Push-Location ./$dirName
    cargo init -q
    svd2rust -m -g -s --pascal_enum_values --max_cluster_size --atomics --atomics_feature atomics -l warn -i $svd_name
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
$index = "./html/index.html"
svdtools htmlcompare ./html/comparison (Get-ChildItem ./svd/ -Filter *.patched) | Out-Null
(Get-Content $index).replace('comparisons.html', 'comparison/index.html') | Set-Content $index
# Remove-Item svd -Recurse