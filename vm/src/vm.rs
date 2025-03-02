use crate::class_loader::class_loading_error::ClassLoadingError;
use crate::class_loader::loaded_class::LoadedClass;

use super::class_loader::class_loader::ClassLoader;
use super::class_loader::loaded_class::NameDes;
use super::heap::Heap;
use super::jvm_error::JVMError;
use super::runtime::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct VM {
    pub stack: Arc<RwLock<Stack>>,
    pub class_loader: ClassLoader,
    pub heap: Arc<RwLock<Heap>>,
}

impl VM {
    pub async fn new(heap_size: usize) -> Self {
        let mut vm = VM {
            stack: Arc::new(RwLock::new(Stack::new())),
            class_loader: ClassLoader::new(),
            heap: Arc::new(RwLock::new(Heap::new(heap_size))),
        };
        vm.preload_classes()
            .await
            .expect("Failed to preload classes");
        vm.preinitialize_classes()
            .await
            .expect("Failed to preinitialize classes");
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
        ];
        let _ = self.class_loader.add_jar_entry(BASE.to_string());
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
        {
            let mut stack = self.stack.write().await;
            let _ = stack.push_frame(main_frame)?;
            let damn = stack.execute_current_frame(self).await?;
            println!("{:?}", damn);
        }
        Ok(())
    }

    pub async fn allocate_object(&self, class_name: &str) -> Result<Value, JVMError> {
        let mut heap = self.heap.write().await;
        heap.allocate_object(self, class_name).await
    }

    pub async fn allocate_array(
        &self,
        element_type: &str,
        length: usize,
    ) -> Result<Value, JVMError> {
        let mut heap = self.heap.write().await;
        heap.allocate_array(self, element_type, length).await
    }
}

const BASE: &str = "/usr/lib/jvm/java-23-openjdk/jmods/java.base.jmod";
