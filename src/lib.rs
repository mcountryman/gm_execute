pub mod execute;

use anyhow::Result;
use execute::os_execute;
use mlua::{lua_State, prelude::*};

fn open(lua: Lua) -> Result<u32> {
  let os: LuaTable = lua.globals().get("os")?;

  set_field(&os, "execute", lua.create_function(os_execute)?)?;

  Ok(0)
}

fn close(lua: Lua) -> Result<u32> {
  let os: LuaTable = lua.globals().get("os")?;

  unset_field(&os, "execute")?;

  Ok(0)
}

/// Sets a field in a table while keeping a temporary reference to original value at
/// a field prefixed with `__gm_execute_original_`.
fn set_field<'lua, V: ToLua<'lua>>(
  table: &LuaTable<'lua>,
  name: &str,
  value: V,
) -> mlua::Result<()> {
  let original_name = format!("__gm_execute_original_{}", name);
  let original: LuaValue<'lua> = table.get(name)?;

  table.set(original_name, original)?;
  table.set(name, value)
}

/// Copies field in table matching name prefixed with `__gm_execute_original_` to field.
fn unset_field<'lua>(table: &LuaTable<'lua>, name: &str) -> LuaResult<()> {
  let original_name = format!("__gm_execute_original_{}", name);
  let original: LuaValue<'lua> = table.get(original_name)?;

  table.set(name, original)
}

/// Called when Garry's Mod initializes this module.
///
/// # Safety
/// Called from an unsafe context and attempts to "safely" wrap the lua state.  We also
/// have to return the number of entries on the stack which if done incorrectly can corrupt
/// the lua state.
#[no_mangle]
pub unsafe extern "C" fn gmod13_open(lua: *mut lua_State) -> u32 {
  open(Lua::init_from_ptr(lua)).unwrap()
}

/// Called when Garry's Mod de-initializes this module.
///
/// # Safety
/// Called from an unsafe context and attempts to "safely" wrap the lua state.  We also
/// have to return the number of entries on the stack which if done incorrectly can corrupt
/// the lua state.
#[no_mangle]
pub unsafe extern "C" fn gmod13_close(lua: *mut lua_State) -> u32 {
  close(Lua::init_from_ptr(lua)).unwrap()
}
