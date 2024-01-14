@echo off
echo Downloading Tron...
powershell -Command "(New-Object System.Net.WebClient).DownloadFile('https://tronlang.org/tron-lang', 'tron.exe')"
echo Installing Tron...
move /y tron.exe C:\Windows\System32\
echo Done!
pause