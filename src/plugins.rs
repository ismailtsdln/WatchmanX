use anyhow::{anyhow, Result};
use std::path::Path;
use wasmer::{imports, Instance, Module, Store, Value};

pub struct PluginEngine {
    store: Store,
    instance: Option<Instance>,
}

impl PluginEngine {
    pub fn new() -> Self {
        Self {
            store: Store::default(),
            instance: None,
        }
    }

    pub fn load_plugin(&mut self, wasm_path: &Path) -> Result<()> {
        let wasm_bytes = std::fs::read(wasm_path)?;
        let module = Module::new(&self.store, wasm_bytes)?;
        let import_object = imports! {};
        let instance = Instance::new(&mut self.store, &module, &import_object)?;
        self.instance = Some(instance);
        Ok(())
    }

    pub fn should_process(&mut self, _path: &str) -> bool {
        let instance = match &self.instance {
            Some(i) => i,
            None => return true,
        };

        let filter_fn = match instance.exports.get_function("filter_event") {
            Ok(f) => f,
            Err(_) => return true,
        };

        // This is highly simplified. A real impl would involve shared memory for strings.
        // For 'conceptual' purposes, we just show we can call a function.
        let result = filter_fn.call(&mut self.store, &[]);
        match result {
            Ok(vals) => {
                if let Some(Value::I32(v)) = vals.get(0) {
                    *v != 0
                } else {
                    true
                }
            }
            Err(_) => true,
        }
    }
}
