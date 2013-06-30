@echo off
cls

set root_path=E:\Projects\c++\GarrysMod\gm_execute
set project_path=%root_path%\project\windows\vs2010
set builder_path=C:\Program Files (x86)\Microsoft Visual Studio 10.0\VC

cd "%root_path%"
premake4 --os=windows --platform=x32 --file=premake.lua vs2010

call "%builder_path%/vcvarsall.bat" x86

cd "%project_path%"
devenv "%project_path%/execute.sln" /rebuild "release|Win32"