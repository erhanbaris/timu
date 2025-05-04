use std::{collections::HashMap, rc::Rc};

use context::TirContext;
use snafu::Snafu;

use crate::{
    ast::{ClassDefinitionAst, ClassDefinitionFieldAst, FileAst, FunctionDefinitionAst, UseAst},
    nom_tools::Span,
};

mod context;

static STANDART_LIBRARY_PREFIX: &str = "std";

#[derive(Debug, Snafu)]
pub enum TirError {
    #[snafu(visibility(pub), display("Module not found"))]
    ModuleNotFound {},
}

#[derive(Debug)]
struct ProjectModule<'base> {
    name: &'base str,
    modules: Vec<Rc<ProjectModule<'base>>>,
    imported_modules: HashMap<&'base str, Rc<ProjectModule<'base>>>,
    signatures: Vec<ModuleSignature<'base>>,
}

impl<'base> ProjectModule<'base> {
    #[cfg(test)]
    fn new(name: &'base str) -> Self {
        Self {
            name,
            modules: Vec::new(),
            imported_modules: HashMap::new(),
            signatures: Vec::new(),
        }
    }

    fn get_signature(self: &Rc<Self>, path: &[Span<'base>]) -> Option<Rc<ProjectModule<'base>>> {
        if self.name != *path[0].fragment() {
            return None;
        }

        if path.len() == 1 {
            return Some(self.clone());
        }

        let mut found_module = self.modules.iter().find(|module| module.name == *path[1].fragment())?;

        if path.len() > 1 {
            for path in path[2..].iter() {
                found_module = found_module.modules.iter().find(|module| module.name == *path.fragment())?;
            }
        }

        Some(found_module.clone())
    }
}

pub fn build<'base>(files: Vec<Rc<FileAst<'base>>>) -> Result<(), TirError> {
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

#[derive(Debug)]
pub enum ModuleSignature<'base> {
    Module(Rc<ProjectModule<'base>>),
    Class(Rc<ClassDefinitionAst<'base>>),
    Function(Rc<FunctionDefinitionAst<'base>>),
    Interface(Span<'base>),
}

fn build_module_signature<'base>(context: &mut TirContext<'base>, file_ast: Rc<FileAst<'base>>) -> Result<(), TirError> {
    let mut signatures = vec![];

    // Class signatures
    for klass in file_ast.get_classes() {
        let mut fields = vec![];
        let mut functions = vec![];

        klass.fields.iter().for_each(|field| {
            match field {
                ClassDefinitionFieldAst::ClassField(var) => fields.push(var.name.clone()),
                ClassDefinitionFieldAst::ClassFunction(func) => functions.push(func.name.clone()),
            }
        });
        
        context.signatures.insert(format!("{}.{}", file_ast.file.name(), klass.name.fragment()), ModuleSignature::Class(klass));
    }

    // Function signatures
    for func in file_ast.get_functions() {
        context.signatures.insert(format!("{}.{}", file_ast.file.name(), func.name.fragment()), ModuleSignature::Function(func));
    }

    // Imterface signatures
    for interface in file_ast.get_interfaces() {
        signatures.push(ModuleSignature::Interface(interface.name.clone()));
    }

    let module: Rc<ProjectModule> = ProjectModule {
        name: file_ast.file.name(),
        modules: Vec::new(),
        imported_modules: HashMap::new(),
        signatures,
    }.into();
    context.modules.push(module.clone());
    context.signatures.insert(file_ast.file.name().to_string(), ModuleSignature::Module(module));

    Ok(())
}

fn build_file<'base>(context: &mut TirContext<'base>, file_ast: Rc<FileAst<'base>>) -> Result<(), TirError> {
    let uses = file_ast.get_uses();
    let mut module = ProjectModule {
        name: file_ast.file.name(),
        modules: Vec::new(),
        imported_modules: HashMap::new(),
        signatures: Vec::new(),
    };

    for use_item in uses {
        build_use(context, &mut module, use_item)?;
    }

    context.modules.push(module.into());

    Ok(())
}

fn build_use<'base>(context: &TirContext<'base>, module: &mut ProjectModule<'base>, use_item: &UseAst<'base>) -> Result<(), TirError> {
    if let Some(found_module) = context.get_signature(use_item.import.as_ref()) {
        println!("Module found: {}", module.name);
        //module.imported_modules.insert(use_item.import.as_ref(), found_module);

        if *use_item.splited_import[0].fragment() == STANDART_LIBRARY_PREFIX {
            println!("Use std library: {}", use_item);
        } else {
            println!("Use custom library: {}", use_item);
        }
    } else {
        println!("Module not found: {}", use_item);
        return Err(TirError::ModuleNotFound {});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        file::SourceFile,
        nom_tools::{Span, State},
    };

    use super::ProjectModule;

    #[test]
    fn find_module_test_1() {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), "<memory>".into(), ""));

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

        let found_module =
            context.get_signature("test1.test2.test3");
        let found_module = found_module.unwrap();

        let found_module = context.get_signature("test1.test2");
        let found_module = found_module.unwrap();

        let found_module = context.get_signature("test1");
        let found_module = found_module.unwrap();

        let found_module = context.get_signature("");
        assert_eq!(found_module.is_none(), true);
    }

    #[test]
    fn module_not_found() {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), "<memory>".into(), ""));

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
        assert_eq!(found_module.is_none(), true);
    }
}
