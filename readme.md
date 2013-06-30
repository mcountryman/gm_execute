gm_execute
===========

Download:
-----------
Linux:
	[Server](https://github.com/marvincountryman/gm_execute/blob/master/bin/linux/release/gmsv_execute_linux.so?raw=true)
	[Client](https://github.com/marvincountryman/gm_execute/blob/master/bin/linux/release/gmcl_execute_linux.so?raw=true)
Windows:
	[Server](https://github.com/marvincountryman/gm_execute/blob/master/bin/windows/release/gmsv_execute_win32.dll?raw=true)
	[Client](https://github.com/marvincountryman/gm_execute/blob/master/bin/windows/release/gmcl_execute_win32.dll?raw=true)

Example:
-----------
```lua
require "execute"
local stdout = os.execute("git clone http://www.github.com/marvincountryman/gm_execute.git")
```
