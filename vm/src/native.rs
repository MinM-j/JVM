use super::class_loader::loaded_class::NameDes;
use super::object::{Object, ObjectKind};
use super::parse_des::{parse_descriptor, parse_return_type};
use super::runtime::Value;
use libffi::middle::{Arg, Cif, CodePtr, Type};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::sync::Arc;
use std::path::PathBuf;

pub struct NativeMethodLoader {
    lib: Library,
    functions: HashMap<String, *const ()>,
}

pub struct NativeStack {
    native_loaders: HashMap<String, Arc<NativeMethodLoader>>, // Library name -> loader
    native_method_map: HashMap<NameDes, String>,              // Method -> library name
}

impl NativeStack {
    pub fn new() -> Self {
        NativeStack {
            native_loaders: HashMap::new(),
            native_method_map: HashMap::new(),
        }
    }

    pub fn register_library(&mut self, lib_name: &str, lib_path: PathBuf) -> Result<(), String> {
        let mut loader = NativeMethodLoader::new(lib_path)
            .map_err(|e| format!("Failed to load library {}: {}", lib_name, e))?;
        if lib_name == "native_io" {
            loader.load_function("Java_ioTer_prints")?;
            loader.load_function("Java_ioTer_printd")?;
            loader.load_function("Java_ioTer_printi")?;
        } else if lib_name == "math" {
            loader.load_function("Java_Math_add")?;
        }
        self.native_loaders
            .insert(lib_name.to_string(), Arc::new(loader));
        Ok(())
    }

    pub fn register_method(&mut self, name_des: NameDes, lib_name: &str) -> Result<(), String> {
        if self.native_loaders.contains_key(lib_name) {
            self.native_method_map
                .insert(name_des, lib_name.to_string());
            Ok(())
        } else {
            Err(format!("Library {} not registered", lib_name))
        }
    }

    pub fn invoke(
        &self,
        name: &str,
        class_name: &str,
        args: &[Value],
        descriptor: &str,
    ) -> Result<Value, String> {
        let lib_name = self
            .native_method_map
            .iter()
            .find(|(key, _)| format!("Java_{}_{}", class_name, key.name) == name)
            .map(|(_, lib)| lib.to_string())
            .ok_or_else(|| format!("No library mapped for method: {}", name))?;

        let loader = self
            .native_loaders
            .get(&lib_name)
            .ok_or_else(|| format!("Library {} not found", lib_name))?;

        loader.invoke(name, args, descriptor)
    }
}

impl NativeMethodLoader {
    pub fn new(lib_path: PathBuf) -> Result<Self, String> {
        let lib = unsafe { Library::new(lib_path) }
            .map_err(|e| format!("Failed to load library: {}", e))?;
        Ok(NativeMethodLoader {
            lib,
            functions: HashMap::new(),
        })
    }

    pub fn load_function(&mut self, name: &str) -> Result<(), String> {
        let symbol: Symbol<unsafe extern "C" fn()> = unsafe {
            self.lib
                .get(name.as_bytes())
                .map_err(|e| format!("Failed to load symbol {}: {}", name, e))?
        };
        let raw_ptr = *symbol as *const ();
        self.functions.insert(name.to_string(), raw_ptr);
        Ok(())
    }

    pub fn invoke(&self, name: &str, args: &[Value], descriptor: &str) -> Result<Value, String> {
        let func_ptr = self
            .functions
            .get(name)
            .ok_or_else(|| format!("Native function {} not found", name))?;

        //println!("Invoking native function: {}, args: {:?}", name, args);

        let mut cif_args = Vec::new();
        let mut call_args = Vec::new();
        let mut c_strings = Vec::new();

        let param_types = parse_descriptor(descriptor)?;
        let return_type = param_types.return_type;

        match name {
            "Java_ioTer_prints" => {
                if args.len() != 1 {
                    return Err(format!("Expected 1 arguments for add, got {}", args.len()));
                }
                match &args[0] {
                    Value::Reference(Some(obj)) => {
                        let string_value = extract_string(obj)?;
                        let c_string = CString::new(string_value)
                            .map_err(|e| format!("CString conversion failed: {}", e))?;
                        c_strings.push(c_string); // Keep CString alive
                        cif_args.push(Type::pointer());
                        call_args.push(Arg::new(&c_strings.last().unwrap().as_ptr()));
                    }
                    _ => return Err("Expected string argument".to_string()),
                }
            }
            "Java_ioTer_printd" => {
                if args.len() != 1 {
                    return Err(format!(
                        "Expected 1 argument for printNum, got {}",
                        args.len()
                    ));
                }
                let number = match &args[0] {
                    Value::Int(i) => *i as f64,
                    Value::Float(f) => *f as f64,
                    Value::Double(d) => *d,
                    _ => return Err("Argument must be a number (int, float, double)".to_string()),
                };
                cif_args.push(Type::f64());
                call_args.push(Arg::new(&number));
            }
            "Java_ioTer_printi" => {
                if args.len() != 1 {
                    return Err(format!(
                        "Expected 1 argument for printNum, got {}",
                        args.len()
                    ));
                }
                let number = match &args[0] {
                    Value::Int(i) => *i ,
                    _ => return Err("Argument must be a number (int)".to_string()),
                };
                cif_args.push(Type::i32());
                call_args.push(Arg::new(&number));
            }
            _ => return Err(format!("Unknown native function: {}", name)),
        }

        let cif = Cif::new(cif_args, parse_return_type(&return_type)?);
        let code_ptr = CodePtr::from_ptr(*func_ptr as *const c_void);

        unsafe {
            if return_type == "V" {
                cif.call::<()>(code_ptr, &call_args);
                Ok(Value::Int(0))
            } else {
                let result: i32 = cif.call(code_ptr, &call_args);
                Ok(Value::Int(result))
            }
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
                if let ObjectKind::ArrayInstance {
                    elements,
                    element_type,
                    ..
                } = &char_array.kind
                {
                    if element_type == "C" {
                        let chars: String = elements
                            .borrow()
                            .iter()
                            .map(|v| match v {
                                Value::Int(c) => *c as u8 as char,
                                _ => '?',
                            })
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
