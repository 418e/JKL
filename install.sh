#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  echo "Installing Tron for Unix..."
  curl -o tron https://tronlang.org/tron-lang && \
  sudo mv tron /usr/local/bin/ && \
  sudo chmod +x /usr/local/bin/tron
elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
  echo "Installing Tron for Windows..."
  powershell -Command "Invoke-WebRequest -Uri 'https://tronlang.org/tron-lang' -OutFile '$env:TEMP\tron.exe'; Move-Item -Path '$env:TEMP\tron.exe' -Destination '$env:ProgramFiles\Tron'; $env:PATH += ';$env:ProgramFiles\Tron'"
else
  echo "Unsupported OS: $OSTYPE"
  exit 1
fi