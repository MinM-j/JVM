use super::class::ClassRef;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ClassLoader<'a> {
    classes: HashMap<String, ClassRef<'a>>,
}

impl<'a> ClassLoader<'a> {
    pub fn register(&mut self, class: ClassRef<'a>) {
        self.classes.insert(class.name.clone(), class);
    }

    pub fn find(&self, name: &str) -> Option<ClassRef<'a>> {
        self.classes.get(name).cloned()
    }
}
