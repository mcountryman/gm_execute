sdkroot = "E:/Projects/sourcesdk/"

solution "execute"
	language "C++"
	location(string.format("project/%s/%s", os.get(), _ACTION))

	local suffixes = {
		["windows"] = "win32",
		["macosx"]	= "osx",
		["linux"]	= "linux",
	}
	local suffix = suffixes[os.get()]

	flags 		{
		"NoEditAndContinue",
		"StaticRuntime",
		"EnableSSE",
		"FloatFast",
		"Optimize",
		"NoPCH",
	}
	defines {
		"NO_SDK"
	}
	includedirs {
		"include/"
	}
	configurations {
		"debug",
		"release"
	}

	configuration "debug"
		flags	{
			"Symbols"
		}
		defines {
			"DEBUG"
		}
		linkoptions {
			"/INCREMENTAL:NO"
		}

		targetdir(string.format("bin/%s/debug/", os.get()))
	configuration "release"
		defines {
			"NDEBUG",
			"RELEASE"
		}
		targetdir(string.format("bin/%s/release/", os.get()))
		
	project "client"
		files 	{
			"source/**.*",
			"include/**.*"
		}
		defines {
			"CLIENT",
			"GMMODULE"
		}
		kind "SharedLib"
		targetname(string.format("gmcl_execute_%s", suffix))
	project "server"
		files {
			"source/**.*",
			"include/**.*"
		}
		defines {
			"SERVER",
			"GMMODULE"
		}
		kind "SharedLib"
		targetname(string.format("gmsv_execute_%s", suffix))