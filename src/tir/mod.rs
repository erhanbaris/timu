use std::{borrow::Cow, collections::HashMap, rc::Rc};

use context::TirContext;
use error::TirError;
use signature::{ModuleSignature, SignatureHolder, build_module_signature};

use crate::ast::{FileAst, UseAst};

mod context;
mod error;
mod signature;

#[derive(Debug)]
pub struct ProjectModule<'base> {
    name: &'base str,
    modules: Vec<Rc<ProjectModule<'base>>>,
    imported_modules: HashMap<Cow<'base, str>, Rc<ModuleSignature<'base>>>,
    signatures: SignatureHolder<'base>,
}

impl<'base> ProjectModule<'base> {
    #[cfg(test)]
    fn new(name: &'base str) -> Self {
        Self {
            name,
            modules: Vec::new(),
            imported_modules: HashMap::new(),
            signatures: SignatureHolder::default(),
        }
    }
}

pub fn build(files: Vec<Rc<FileAst<'_>>>) -> Result<(), TirError<'_>> {
    let mut context: TirContext = TirContext::default();

    for file in files.iter() {
        build_module_signature(&mut context, file.clone())?;
    }

    for file in files.into_iter() {
        build_file(&mut context, file)?;
    }

    println!("Context: {:#?}", context);
    Ok(())
}

fn build_file<'base>(context: &mut TirContext<'base>, file_ast: Rc<FileAst<'base>>) -> Result<(), TirError<'base>> {
    let uses = file_ast.get_uses();
    let mut module = ProjectModule {
        name: file_ast.file.name(),
        modules: Vec::new(),
        imported_modules: HashMap::new(),
        signatures: Default::default(),
    };

    for use_item in uses {
        build_use(context, &mut module, use_item)?;
    }

    context.modules.push(module.into());

    Ok(())
}

fn build_use<'base>(context: &'_ TirContext<'base>, module: &mut ProjectModule<'base>, use_item: &UseAst<'base>) -> Result<(), TirError<'base>> {
    if let Some(signature) = context.get_signature(&use_item.import) {
        println!("Module found: {}", module.name);
        if let Some(old_signature) = module.imported_modules.insert(use_item.import.clone(), signature.clone()) {
            return Err(TirError::SignatureAlreadyDefined {
                old_signature,
            });
        }
    } else {
        println!("Module not found: {}", use_item);
        return Err(TirError::ModuleNotFound {
            module: use_item.import.clone().into(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{file::SourceFile, nom_tools::State};

    use super::ProjectModule;

    #[test]
    fn find_module_test_1() {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), ""));

        let state = State {
            file: source_file.clone(),
        };
        let mut module1 = ProjectModule::new("test1");
        let mut module2 = ProjectModule::new("test2");
        let module3 = ProjectModule::new("test3");

        module2.modules.push(module3.into());
        module1.modules.push(module2.into());

        let mut context = super::TirContext::default();
        context.modules.push(module1.into());

        let found_module = context.get_signature("test1.test2.test3");
        let found_module = found_module.unwrap();

        let found_module = context.get_signature("test1.test2");
        let found_module = found_module.unwrap();

        let found_module = context.get_signature("test1");
        let found_module = found_module.unwrap();

        let found_module = context.get_signature("");
        assert!(found_module.is_none());
    }

    #[test]
    fn module_not_found() {
        let source_file = Rc::new(SourceFile::new("<memory>", "<memory>".into(), ""));

        let state = State {
            file: source_file.clone(),
        };
        let mut module1 = ProjectModule::new("test1");
        let mut module2 = ProjectModule::new("test2");
        let module3 = ProjectModule::new("test3");

        module2.modules.push(module3.into());
        module1.modules.push(module2.into());

        let mut context = super::TirContext::default();
        context.modules.push(module1.into());

        let found_module = context.get_signature("abc");
        assert!(found_module.is_none());
    }
}
