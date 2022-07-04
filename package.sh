#!/bin/sh
rm -R package
mkdir -p package/linux package/windows

cp target/release/Griffsort package/linux
cp -r ./assets package/linux
zip -r package/griffsort-lin.zip package/linux

cp target/x86_64-pc-windows-gnu/release/Griffsort.exe package/windows
cp -r ./assets package/windows
zip -r package/griffsort-win.zip package/windows

