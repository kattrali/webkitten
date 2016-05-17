extern crate hlua;

use self::hlua::{Lua,LuaError};

pub struct ScriptingRuntime {
    lua: Lua
}

impl ScriptingRuntime {

    pub fn new() -> Self {
        ScriptingRuntime {
            lua: Lua::new()
        }
    }

    pub fn execute(&self, script: &str, app: &Application) {

    }

    pub fn add_window(&self, uri: &str) {
    }
}
