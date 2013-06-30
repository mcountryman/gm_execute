@echo off
cls

premake4 --os=windows 	--platform=x32 			--file=premake.lua vs2010
premake4 --os=macosx 	--platform=universal32 	--file=premake.lua gmake
premake4 --os=linux 	--platform=x32 			--file=premake.lua gmake

pause