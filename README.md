# gm_execute
A Garry's Mod module that enables `os.execute`.

## Release
For releases, click the releases tab.

## Reference
```lua
-- Loads the `gm_execute` module.
require "execute"

--- Executes supplied command on shell.
-- If the command executes with status code zero the function returns the stdout as a 
-- string.  If the status is non-zero the function returns the stderr as a string.
-- @param command The shell command to execute.
-- @returns Either `stdout` or `stderr` from shell.
os.execute(command)
```
