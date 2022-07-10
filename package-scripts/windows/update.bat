rm -R .buffer
mkdir -p ./backup .buffer

fetch --repo="https://github.com/ggaast/griffsort_game" --tag="~>0.0.0" --github-oauth-token="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" --progress --release-asset="linux.zip" .buffer

echo Creating Backup

zip -r ./backup/backup.zip Griffsort assets config scripts

echo Upgrading Griffsort
unzip -o .buffer/*linux.zip -d .buffer; rm linux.zip;

rm -R .buffer

echo Press Enter to exit
read junk
