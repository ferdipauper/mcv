@echo off
rem Seret folder tekstur ke file ini, atau jalankan: convert_tga.bat "C:\folder\tekstur"
if "%~1"=="" (
  echo Seret folder tekstur ke file ini.
  pause
  exit /b 1
)
python "%~dp0convert_tga.py" "%~1" --mcv
pause
