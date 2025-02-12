use super::class_loader::class_loader::ClassLoader;
use super::class_loader::loaded_class::NameDes;
use super::runtime::*;
use super::jvm_error::JVMError;
use std::cell::RefCell;

pub struct VM {
    pub stack: RefCell<Stack>,
    pub class_loader: ClassLoader,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: RefCell::new(Stack::new()),
            class_loader: ClassLoader::new(),
        }
    }

    pub async fn invoke_main(&self, class_name: &str) -> Result<(), JVMError> {
        let main_class = self.class_loader.load_class(class_name).await.map_err(|e| JVMError::Other(e.to_string())).unwrap();
        let main_name_des = NameDes {
            name: "main".to_string(),
            des: "([Ljava/lang/String;)V".to_string(),
        };
        let main_frame = Frame::new(main_class, &main_name_des);
        let _ = self.stack.borrow_mut().push_frame(main_frame)?;
        let _ = self.stack.borrow_mut().execute_current_frame(self).await?;
        Ok(())
    }
}
