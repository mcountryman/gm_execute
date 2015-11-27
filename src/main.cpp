#include <string>
#include <iostream>
#include <stdio.h>

#include "GarrysMod/Lua/Interface.h"

using namespace GarrysMod;

#if _WIN32
	#define POPEN _popen
	#define PCLOSE _pclose
#else
	#define POPEN popen
	#define PCLOSE pclose
#endif

int os_execute(lua_State *state) {
	LUA->CheckType(1, Lua::Type::STRING);

	std::string shell = LUA->GetString(1);

	FILE* pipe = POPEN(shell.c_str(), "r");

	if (!pipe) {
		return 0;
	}

	std::string result = "";

	char buffer[128];
	while (!feof(pipe)) {
		if (fgets(buffer, 128, pipe) != NULL) {
			result += buffer;
		}
	}

	PCLOSE(pipe);

	LUA->PushString(result.c_str());

	return 1;
}

GMOD_MODULE_OPEN() {
	LUA->PushSpecial(Lua::SPECIAL_GLOB);
		LUA->GetField(-1, "os");
			LUA->PushString("execute");
			LUA->PushCFunction(os_execute);
		LUA->SetTable(-3);
	LUA->Pop();

	return 0;
}

GMOD_MODULE_CLOSE() {
	return 0;
}
