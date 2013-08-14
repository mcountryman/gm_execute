##gm_execute
A garrysmod module that enables os.execute.

###Release
Linux:	
* Server: [gmsv_execute_linux.so](https://github.com/marvincountryman/gm_execute/blob/master/bin/linux/release/gmsv_execute_linux.so?raw=true) 
* Client: [gmcl_execute_linux.so](https://github.com/marvincountryman/gm_execute/blob/master/bin/linux/release/gmcl_execute_linux.so?raw=true)

Windows: 
* Server: [gmsv_execute_win32.dll](https://github.com/marvincountryman/gm_execute/blob/master/bin/windows/release/gmsv_execute_win32.dll?raw=true) 	
* Client: [gmcl_execute_win32.dll](https://github.com/marvincountryman/gm_execute/blob/master/bin/windows/release/gmcl_execute_win32.dll?raw=true)

###Reference
```lua
--[[
  Finds files and folders from a search path.

  @param  string  search  Shell input
  @return string          stdout
]]
os.execute(shell)
```
