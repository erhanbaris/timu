use thiserror::Error;

use crate::{ast::{FileAst, UseAst}, nom_tools::Span};

static STANDART_LIBRARY_PREFIX: &str = "std";

pub struct Tir;

#[derive(Debug, Error)]
pub enum TirError {

}

#[derive(Debug, Default)]
struct TirContext<'a> {
    modules: Vec<ProjectModule<'a>>,
}


impl<'a> TirContext<'a> {
    pub fn get_module(&self, use_item: &UseAst<'a>) -> Option<&ProjectModule<'a>> {
        for module in self.modules.iter() {
            if let Some(module) = module.get_module(use_item) {
                return Some(module);
            }
        } 

        None
    }
}

#[derive(Debug, Default)]
struct File {
}

#[derive(Debug)]
struct ProjectModule<'a> {
    name: Span<'a>,
    modules: Vec<ProjectModule<'a>>,
}

impl<'a> ProjectModule<'a> {
    fn new(name: Span<'a>) -> Self {
        Self {
            name,
            modules: Vec::new(),
        }
    }

    fn get_module(&self, search: &UseAst<'a>) -> Option<&ProjectModule<'a>> {

        let mut found_module = self.modules
            .iter()
            .find(|module| module.name.fragment() == search.paths[0].fragment())?;

        if search.paths.len() > 1 {
            
        for path in search.paths[1..].iter() {
            found_module = found_module.modules
                    .iter()
                    .find(|module| module.name.fragment() == path.fragment())?;
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
    if let Some(module) = context.get_module(use_item) {

    }
    

    if *use_item.paths[0].fragment() == STANDART_LIBRARY_PREFIX {
        println!("Use std library: {}", use_item);
    } else {
        println!("Use custom library: {}", use_item);
    }
    Ok(())
}
