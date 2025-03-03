use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use super::runtime::Value;
use super::object::{ObjectKind, Object};
use std::sync::Arc;

type NativeFn = unsafe extern "C" fn(*const c_char);

pub struct NativeMethodLoader {
    lib: Library,
    functions: HashMap<String, Symbol<'static, NativeFn>>,
}

impl NativeMethodLoader {
    pub fn new(lib_path: &str) -> Result<Self, String> {
        let lib = unsafe { Library::new(lib_path) }
            .map_err(|e| format!("Failed to load library: {}", e))?;
        Ok(NativeMethodLoader {
            lib,
            functions: HashMap::new(),
        })
    }

    pub fn load_function(&mut self, name: &str) -> Result<(), String> {
        let symbol: Symbol<NativeFn> = unsafe {
            self.lib.get(name.as_bytes())
                .map_err(|e| format!("Failed to load symbol {}: {}", name, e))?
        };
        let symbol: Symbol<'static, NativeFn> = unsafe { std::mem::transmute(symbol)};
        self.functions.insert(name.to_string(), symbol);
        Ok(())
    }

    pub fn invoke(&self, name: &str, args: &[Value]) -> Result<Value, String> {
        let func = self.functions.get(name)
            .ok_or_else(|| format!("Native function {} not found", name))?;
        //println!("Invoking native function: {}, args: {:?}", name, args);
        
        match name {
            "Java_ioTer_prints" => {
                if let [Value::Reference(Some(obj))] = args {
                    let string_value = extract_string(obj)?;
                    let c_string = CString::new(string_value)
                        .map_err(|e| format!("CString conversion failed: {}", e))?;
                    unsafe { (*func)(c_string.as_ptr()) };
                    Ok(Value::Int(0)) 
                } else {
                    Err(format!("Invalid arguments for printf: expected 1 String, got {:?}", args))
                }
            }
            _ => Err(format!("Unknown native function: {}", name)),
        }
    }
}

fn extract_string(obj: &Arc<Object>) -> Result<String, String> {
    //println!("Extracting string from obj: kind={:?}, class={:?}", obj.kind, obj.class.as_ref().map(|c| &c.class_name));
    
    if let ObjectKind::ClassInstance { fields } = &obj.kind {
        let fields = fields.borrow();
        //println!("Fields: {:?}", fields);
        for field in fields.iter() {
            if let Value::Reference(Some(char_array)) = field {
                if let ObjectKind::ArrayInstance { elements, element_type, .. } = &char_array.kind {
                    if element_type == "C" {
                        let chars: String = elements.borrow().iter()
                            .map(|v| match v { Value::Int(c) => *c as u8 as char, _ => '?' })
                            .collect();
                        return Ok(chars);
                    }
                }
            }
        }
        Err("No char array found in String object fields".to_string())
    } else {
        Err(format!("Object is not a ClassInstance: {:?}", obj.kind))
    }
}
