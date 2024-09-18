use super::class::{Class, ClassId, ClassRef};
use super::class_loader::ClassLoader;
use super::class_path::{ClassPath, ClassPathError};
use super::error::Error;
use indexmap::IndexMap;
use parser::{class_file::ClassFile, class_file_reader::ClassFileReader};
use std::{collections::HashMap, fmt};
use typed_arena::Arena;

pub struct ClassManager<'a> {
    class_path: ClassPath,
    class_by_id: HashMap<ClassId, ClassRef<'a>>,
    class_by_name: HashMap<String, ClassRef<'a>>,
    arena: Arena<Class<'a>>,
    next_id: u32,
    class_loader: ClassLoader<'a>,
}

impl<'a> Default for ClassManager<'a> {
    fn default() -> Self {
        Self {
            class_path: Default::default(),
            class_by_id: Default::default(),
            class_by_name: Default::default(),
            arena: Arena::with_capacity(100),
            next_id: 1,
            class_loader: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Resolved<'a> {
    Loaded(ClassRef<'a>),
    New(ClassToInitialize<'a>),
}

impl<'a> Resolved<'a> {
    pub fn get_class(&self) -> ClassRef<'a> {
        match self {
            Resolved::Loaded(class) => class,
            Resolved::New(class_to_initialize) => class_to_initialize.resolved_class,
        }
    }
}

pub trait ClassById<'a> {
    fn find_class_by_id(&self, class_id: ClassId) -> Option<ClassRef<'a>>;
}

impl<'a> ClassById<'a> for ClassManager<'a> {
    fn find_class_by_id(&self, class_id: ClassId) -> Option<ClassRef<'a>> {
        self.class_by_id.get(&class_id).cloned()
    }
}

#[derive(Debug)]
pub struct ClassToInitialize<'a> {
    resolved_class: ClassRef<'a>,
    pub initialize: Vec<ClassRef<'a>>,
}

impl<'a> ClassManager<'a> {
    pub fn append_path(&mut self, path: &str) -> Result<(), ClassPathError> {
        self.class_path.add(path)
    }

    pub fn find_class_by_name(&self, name: &str) -> Option<ClassRef<'a>> {
        self.class_by_name.get(name).cloned()
    }

    pub fn get_resolve_class(&mut self, name: &str) -> Result<Resolved<'a>, Error> {
        if let Some(loaded_class) = self.find_class_by_name(name) {
            Ok(Resolved::Loaded(loaded_class))
        } else {
            self.resolve_load_class(name).map(Resolved::New)
        }
    }

    fn resolve_load_class(&mut self, name: &str) -> Result<ClassToInitialize<'a>, Error> {
        let class_file_byte = self
            .class_path
            .resolve(name)
            .map_err(|err| Error::ClassLoadingError(err.to_string()))?
            .ok_or(Error::ClassNotFound(name.to_string()))?;
        let class_file = ClassFileReader::new(class_file_byte).parse();
        self.load_class(class_file)
    }

    fn load_class(&mut self, class_file: ClassFile) -> Result<ClassToInitialize<'a>, Error> {
        let reference_class = load_super_interface(&class_file)?;
        let loaded_class = self.allocate(class_file, reference_class)?;
        self.register(loaded_class.resolved_class);
        Ok(loaded_class)
    }

    fn load_super_interface(
        &mut self,
        class_file: &ClassFile,
    ) -> Result<IndexMap<String, Resolved<'a>>, Error> {
        let mut resolved_classes: IndexMap<String, Resolved<'a>> = Default::default();
        //Todo traverse to constant info and retirieve the name of super class
        if let Some(superclass_name) = &class_file.super_class.to_string() {
            self.resolve_collect(superclass_name, &mut resolved_classes)?;
        }
        //Todo traverse to constant info and retrieve the name of interfaces
        for interface in class_file.interfaces.iter {
            self.resolve_collect(interface, &mut resolved_classes)?;
        }
        Ok(resolved_classes)
    }

    fn resolve_collect(
        &mut self,
        name: &str,
        resolved_classes: &mut IndexMap<String, Resolved<'a>>,
    ) -> Result<(), Error> {
        let class = self.get_resolve_class(name)?;
        resolved_classes.insert(name.to_string(), class);
        Ok(())
    }

    fn allocate(
        &mut self,
        class_file: ClassFile,
        resolved_classes: IndexMap<String, Resolved<'a>>,
    ) -> Result<ClassToInitialize<'a>, Error> {
        let next_id = self.next_id;
        self.next_id += 1;
        let id = ClassId::new(next_id);
        let class = Self::new_class(class_file, id, &resolved_classes)?;
        let class_ref = self.arena.alloc(class);
        let mut class_init: Vec<ClassRef<'a>> = Vec::new();
        for resolved in resolved_classes.values() {
            if let Resolved::New(new_class) = resolved {
                for to_initialize in new_class.initialize.iter() {
                    class_init.push(to_initialize);
                }
            }
        }
        class_init.push(class_ref);
        Ok(ClassToInitialize {
            resolved_class: class_ref,
            initialize: class_init,
        })
    }

    fn new_class(
        &mut self,
        class_file: ClassFile,
        id: ClassId,
        resolved_classes: &IndexMap<String, Resolved<'a>>,
    ) -> Result<Class<'a>, Error> {
        //Todo superclass should be string
        let superclass = class_file
            .super_class
            .as_ref()
            .map(|name| resolved_classes.get(name).unwrap.get_class());
        //Todo interfaces should be string
        let interfaces: Vec<ClassRef<'a>> = class_file
            .interfaces
            .iter()
            .map(|name| resolved_classes.get(name).unwrap.get_class())
            .collect();
        let super_fields_count = match superclass {
            Some (superclass) => superclass.num_total_fields,
            None => 0
        };
        let fields_count = class_file.fields.len();
        Ok(Class{
            id,
            //Todo this_class should be parsed too
            name: class_file.this_class,
            constants: class_file.constant_pool,
            flags: class_file.access_flags,
            superclass
            interfaces
            fields: class_file.fields,
            methods: class_file.methods,
            first_field_index: super_fields_count,
            total_fields: super_fields_count + fields_count,
        })
    }

    fn register(&mut self, class: ClassRef<'a>){
        self.class_by_name.insert(class.name.clone(), class);
        self.class_by_id.insert(class.id, class);
        self.class_loader.register(class);
    }
}
