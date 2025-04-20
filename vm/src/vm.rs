use crate::class_loader::class_loading_error::ClassLoadingError;
use crate::class_loader::loaded_class::LoadedClass;
use crate::state::{Header, MessageData, GLOBAL_BOOL, SERVER_STATE, VIS_BOOL};
use serde_json::json;
//use crate::native::NativeMethodLoader;
use super::native::NativeStack;
use parser::instruction::*;

use super::class_loader::class_loader::ClassLoader;
use super::class_loader::loaded_class::NameDes;
use super::heap::Heap;
use super::jvm_error::JVMError;
use super::runtime::*;
use std::sync::Arc;
use tokio::sync::RwLock;

use std::env;
use std::path::PathBuf;

pub struct VM {
    pub stack: Arc<RwLock<Stack>>,
    pub class_loader: ClassLoader,
    pub heap: Arc<RwLock<Heap>>,
    pub native_stack: NativeStack,
}

impl VM {
    pub async fn new(heap_size: usize) -> Self {
        let init_json = MessageData {
            header: Header::DATA,
            json: json!({"header": "init", "memory size": heap_size}).to_string(),
        };
        {
            let mut queue = SERVER_STATE.lock().unwrap();
            queue.push_back(init_json);
        }
        let mut vm = VM {
            stack: Arc::new(RwLock::new(Stack::new())),
            class_loader: ClassLoader::new(),
            heap: Arc::new(RwLock::new(Heap::new(heap_size))),
            native_stack: NativeStack::new(),
        };
        vm.preload_classes()
            .await
            .expect("Failed to preload classes");
        vm.register_native_methods();
        vm
    }

    async fn preload_classes(&mut self) -> Result<(), ClassLoadingError> {
        let classes = [
            "java/lang/Object",
            "java/lang/String",
            "java/io/Serializable",
            "java/lang/Comparable",
            "java/lang/CharSequence",
            "java/lang/constant/Constable",
            "java/lang/constant/ConstantDesc",
            "java/lang/Class",
        ];
        let _ = self.class_loader.add_jar_entry(BASE.to_string());
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get exe directory");
        let lib_path = exe_dir
            .join("../../IO/")
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();
        let _ = self.class_loader.add_directory_entry(lib_path);
        for class_name in classes.iter() {
            self.class_loader.load_class(class_name, self).await?;
        }
        Ok(())
    }

    async fn preinitialize_classes(&mut self) -> Result<(), ClassLoadingError> {
        let classes = [
            "java/lang/Object",
            "java/lang/String",
            "java/io/Serializable",
            "java/lang/Comparable",
            "java/lang/CharSequence",
            "java/lang/constant/Constable",
            "java/lang/constant/ConstantDesc",
        ];
        let _ = self.class_loader.add_jar_entry(BASE.to_string());
        for class_name in classes.iter() {
            let class = self.class_loader.load_class(class_name, self).await?;
            LoadedClass::initialize(class, self).await.unwrap();
        }
        Ok(())
    }

    fn register_native_methods(&mut self) {
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get exe directory");
        let lib_path = exe_dir.join("../../IO/libnative_io.so");
        self.native_stack
            .register_library("native_io", lib_path)
            .expect("Failed to load libnative_io.so");
        let prints_key = NameDes {
            name: "prints".to_string(),
            des: "(Ljava/lang/String;)V".to_string(),
        };
        self.native_stack
            .register_method(prints_key, "native_io")
            .expect("Failed to register Java_ioTer_prints");
        let print_dec_key = NameDes {
            name: "printd".to_string(),
            des: "(D)V".to_string(),
        };
        self.native_stack
            .register_method(print_dec_key, "native_io")
            .expect("Failed to register Java_ioTer_printd");
        let print_int_key = NameDes {
            name: "printi".to_string(),
            des: "(I)V".to_string(),
        };
        self.native_stack
            .register_method(print_int_key, "native_io")
            .expect("Failed to register Java_ioTer_printi");
    }

    pub async fn invoke_main(&self, class_name: &str) -> Result<(), JVMError> {
        let main_class = self
            .class_loader
            .load_class(class_name, self)
            .await
            .map_err(|e| JVMError::Other(e.to_string()))
            .unwrap();
        let main_name_des = NameDes {
            name: "main".to_string(),
            des: "([Ljava/lang/String;)V".to_string(),
        };
        let (main_class, main_code) = Frame::lookup_method(&main_class, &main_name_des)?;
        let main_frame = Frame::new(main_class, &main_name_des, main_code);

        let mut stack = self.stack.write().await;
        {
            let vis_flag = VIS_BOOL.lock().unwrap();
            if *vis_flag {
                let code = convert_instructions(main_frame.code.code.clone());
                let stack_json = MessageData {
                header: Header::DATA,
                json: json!({"header": "stack", "name": main_frame.method_name_des.name, "action": "push", "locals": main_frame.locals.len(), "operands": main_frame.operands.len(), "code": code}).to_string(),
            };
                {
                    let mut queue = SERVER_STATE.lock().unwrap();
                    queue.push_back(stack_json);
                    let mut value = GLOBAL_BOOL.lock().unwrap();
                    *value = true;
                }
            }
        }
        let _ = stack.push_frame(main_frame)?;
        let _ = stack.execute_current_frame(self).await?;
        //println!("{:?}", damn);
        Ok(())
    }

    pub async fn allocate_object(
        &self,
        stack: &Stack,
        class_name: &str,
    ) -> Result<Value, JVMError> {
        let mut heap = self.heap.write().await;
        heap.allocate_object(stack, self, class_name).await
    }

    pub async fn allocate_array(
        &self,
        stack: &Stack,
        element_type: &str,
        length: usize,
    ) -> Result<Value, JVMError> {
        let mut heap = self.heap.write().await;
        heap.allocate_array(stack, self, element_type, length).await
    }

    pub async fn memory_snap(&self) {
        let heap = self.heap.read().await;
        heap.memory_json();
    }
}

pub fn convert_instructions(instructions: Vec<Instruction>) -> Vec<Operation> {
    instructions
        .into_iter()
        .map(|instruction| instruction.1)
        .collect()
}

const BASE: &str = "/usr/lib/jvm/java-24-openjdk/jmods/java.base.jmod";
