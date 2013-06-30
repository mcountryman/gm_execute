#include <string>
#include <iostream>
#include <stdio.h>

#include "ILuaModuleManager.h"

#if _WIN32
	#define pOpen  _popen
	#define pClose _pclose
#else
	#define pOpen  popen
	#define pClose pclose
#endif

LUA_FUNCTION(execute) {
	std::string shell = Lua()->GetStringOrError(1);

	FILE* pipe = pOpen(shell.c_str(), "r");

	if(!pipe)
		return 0;

	char buffer[128];
	std::string result = "";

	while(!feof(pipe)) {
		if(fgets(buffer, 128, pipe) != NULL)
			result += buffer;
	}

	pClose(pipe);
	Lua()->Push(result.c_str());

	return 1;
}

int open(lua_State* L) {
	ILuaObject* os = Lua()->GetGlobal("os");

	if(os) {
		os->SetMember("execute", execute);
		os->UnReference();
	}

	return 0;
}
int close(lua_State* L) {
	return 0;
}

GMOD_MODULE(open, close)