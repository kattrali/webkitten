extern crate hlua;

use self::hlua::{Lua,LuaError};

pub struct ScriptingRuntime {
    lua: Lua,
    app: Application,
}

impl ScriptingRuntime {

    pub fn new<A: Application>(app: A) -> Self {
        ScriptingRuntime {
            lua: Lua::new(),
            app: app,
        }
    }

    pub fn execute(&self, script: &str) {
    }

    pub fn add_window(&self, uri: &str) {
    }
}
