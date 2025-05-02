use context::TirContext;
use snafu::Snafu;

use crate::{
    ast::{FileAst, UseAst},
    nom_tools::Span,
};

mod context;

static STANDART_LIBRARY_PREFIX: &str = "std";

#[derive(Debug, Snafu)]
pub enum TirError {
    #[snafu(visibility(pub), display("data store disconnected"))]
    ModuleNotFound {
    }
}

#[derive(Debug)]
struct ProjectModule<'a> {
    name: Span<'a>,
    modules: Vec<ProjectModule<'a>>,
}

impl<'a> ProjectModule<'a> {
    #[cfg(test)]
    fn new(name: Span<'a>) -> Self {
        Self {
            name,
            modules: Vec::new(),
        }
    }

    fn get_module(&self, path: &[Span<'a>]) -> Option<&ProjectModule<'a>> {
        if self.name.fragment() != path[0].fragment() {
            return None;
        }

        if path.len() == 1 && self.modules.len() == path.len() {
            return Some(self);
        }

        let mut found_module = self.modules.iter().find(|module| module.name.fragment() == path[1].fragment())?;

        if path.len() > 1 {
            for path in path[2..].iter() {
                found_module = found_module.modules.iter().find(|module| module.name.fragment() == path.fragment())?;
            }
        }

        Some(found_module)
    }
}

pub fn build(files: Vec<FileAst<'_>>) -> Result<(), TirError> {
    let mut context = TirContext::default();

    for file in files.into_iter() {
        build_file(&mut context, file)?;
    }

    Ok(())
}

fn build_file(context: &mut TirContext, file_ast: FileAst<'_>) -> Result<(), TirError> {
    let uses = file_ast.get_uses();

    for use_item in uses {
        build_use(context, use_item)?;
    }

    Ok(())
}

fn build_use(context: &mut TirContext, use_item: &UseAst<'_>) -> Result<(), TirError> {
    if let Some(module) = context.get_module(&use_item.paths) {
        println!("Module found: {}", module.name);
        if *use_item.paths[0].fragment() == STANDART_LIBRARY_PREFIX {
            println!("Use std library: {}", use_item);
        } else {
            println!("Use custom library: {}", use_item);
        }
    } else {
        println!("Module not found: {}", use_item);

        //let location = LocatedSpan::new_from_raw_offset(use_item.paths[0].location_offset(), use_item.paths[0].location_line(), (), ());
        return Err(TirError::ModuleNotFound {
        });
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
    fn find_module_test_1<'a>() {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), ""));

        let state = State {
            file: source_file.clone(),
        };
        let mut module1 = ProjectModule::new(Span::new_extra("test1", state.clone()));
        let mut module2 = ProjectModule::new(Span::new_extra("test2", state.clone()));
        let module3 = ProjectModule::new(Span::new_extra("test3", state.clone()));

        module2.modules.push(module3);
        module1.modules.push(module2);

        let mut context = super::TirContext::default();
        context.modules.push(module1);

        let found_module =
            context.get_module(&[Span::new_extra("test1", state.clone()), Span::new_extra("test2", state.clone()), Span::new_extra("test3", state.clone())]);
        let found_module = found_module.unwrap();
        assert_eq!(found_module.name.fragment(), &"test3");

        let found_module = context.get_module(&[Span::new_extra("test1", state.clone()), Span::new_extra("test2", state.clone())]);
        let found_module = found_module.unwrap();
        assert_eq!(found_module.name.fragment(), &"test2");

        let found_module = context.get_module(&[Span::new_extra("test1", state.clone())]);
        let found_module = found_module.unwrap();
        assert_eq!(found_module.name.fragment(), &"test1");

        let found_module = context.get_module(&[Span::new_extra("", state.clone())]);
        assert_eq!(found_module.is_none(), true);
    }

    #[test]
    fn module_not_found<'a>() {
        let source_file = Rc::new(SourceFile::new("<memory>".into(), ""));

        let state = State {
            file: source_file.clone(),
        };
        let mut module1 = ProjectModule::new(Span::new_extra("test1", state.clone()));
        let mut module2 = ProjectModule::new(Span::new_extra("test2", state.clone()));
        let module3 = ProjectModule::new(Span::new_extra("test3", state.clone()));

        module2.modules.push(module3);
        module1.modules.push(module2);

        let mut context = super::TirContext::default();
        context.modules.push(module1);

        let found_module = context.get_module(&[Span::new_extra("abc", state.clone())]);
        assert_eq!(found_module.is_none(), true);
    }
}
