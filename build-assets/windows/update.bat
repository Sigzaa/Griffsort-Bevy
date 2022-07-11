echo Downloading Griffsort

mkdir .buffer

.\bundle\fetch.exe --repo="https://github.com/ggaast/griffsort_game" --tag="~>0.0.0" --github-oauth-token="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" --release-asset="win.zip" .buffer

echo Creating Backup
.\bundle\7z.exe a .\backup\back.zip Griffsort.exe config assets update.bat


echo Upgrading Griffsort

.\bundle\7z.exe x .buffer/*.zip -aoa -o.\

pause