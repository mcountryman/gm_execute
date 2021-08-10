# gm_execute
A Garry's Mod module that enables `os.execute`.

## Release
For releases, click the releases tab.

## Reference
```lua
-- Loads the `gm_execute` module.
require "execute"

--- Executes supplied command on shell.
-- 
-- @param command The shell command to execute.
-- @returns Either `stdout` or `stderr`.
os.execute(command)
```
