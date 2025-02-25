use super::class_loader::class_loader::ClassLoader;
use super::class_loader::loaded_class::NameDes;
use super::jvm_error::JVMError;
use super::runtime::*;
use std::sync::{Arc, RwLock};

pub struct VM {
    pub stack: Arc<RwLock<Stack>>,
    pub class_loader: ClassLoader,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Arc::new(RwLock::new(Stack::new())),
            class_loader: ClassLoader::new(),
        }
    }

    pub async fn invoke_main(&self, class_name: &str) -> Result<(), JVMError> {
        let main_class = self
            .class_loader
            .load_class(class_name)
            .await
            .map_err(|e| JVMError::Other(e.to_string()))
            .unwrap();
        println!("{}",main_class.class_name);
        let main_name_des = NameDes {
            name: "main".to_string(),
            des: "([Ljava/lang/String;)V".to_string(),
        };
        let main_code = Frame::lookup_method(&main_class, &main_name_des)?;
        let main_frame = Frame::new(main_class, &main_name_des, main_code);
        let mut stack = self.stack.write().unwrap();
        let _ = stack.push_frame(main_frame)?;
        let damn = stack.execute_current_frame(self).await?;
        println!("{:?}",damn);
        Ok(())
    }
}
