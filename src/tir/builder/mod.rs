use std::{cell::{RefCell, RefMut}, fmt::Debug, rc::Rc};

use crate::{ast::{BodyAst, FunctionDefinitionAst, TypeNameAst, UseAst}, nom_tools::Span};

use super::{context::TirContext, error::TirError, module::Module, AstSignature};

pub trait Builder<'base> {
    type Result;
    fn build(&self, context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Result, TirError<'base>>;
}

fn build_type_name(ast: &TypeNameAst) -> String {
    ast.names.iter().map(|path| *path.fragment()).collect::<Vec<&str>>().join(".")
}

pub fn build_file<'base>(context: &mut TirContext<'base>, module: Rc<RefCell<Module<'base>>>) -> Result<(), TirError<'base>> {
    let uses = module.borrow().ast.get_uses().collect::<Vec<_>>();
    let functions = module.borrow().ast.get_functions().collect::<Vec<_>>();
    
    let mut module = module.borrow_mut();

    for use_item in uses {
        use_item.build(context, &mut module)?;
    }

    for function in functions {
        function.build(context, &mut module)?;
    }

    Ok(())
}

impl<'base> Builder<'base> for UseAst<'base> {
    type Result = ();
    fn build(&self, context: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Result, TirError<'base>> {
        if let Some(signature) = context.get_ast_signature(&self.import.text) {
            println!("Module found: {}", module.name);
            
            let name = match &self.alias {
                Some(alias) => std::borrow::Cow::Borrowed(*alias.fragment()),
                None => std::borrow::Cow::Borrowed(*self.name().fragment()),
            };
    
            if let Some(old_signature) = module.imported_modules.insert(name, signature.clone()) {
                return Err(TirError::AstModuleAlreadyDefined {
                    old_signature,
                    source: self.name().extra.file.clone(),
                });
            }
        } else {
            println!("Module not found: {}", self);
            return Err(TirError::ModuleNotFound {
                module: self.import.text.clone(),
                position: self.import.to_range(),
                source: self.name().extra.file.clone(),
            });
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SpannedValue<'a, T: Debug> {
    #[allow(dead_code)] pub span: Span<'a>,
    pub value: T,
}

impl<T> std::ops::Deref for SpannedValue<'_, T> where T: Debug {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: SpannedValue<'base, Rc<AstSignature<'base>>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionDefinition<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: TypeNameAst<'base>,
    pub body: BodyAst<'base>,
}

impl<'base> Builder<'base> for FunctionDefinitionAst<'base> {
    type Result = ();
    fn build(&self, _: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Result, TirError<'base>> {
        let mut arguments = vec![];

        let return_type_name = build_type_name(&self.return_type);
        match module.signatures.get(return_type_name.as_str()) {
            Some(signature) => signature,
            None => {
                return Err(TirError::TypeNotFound {
                    source: self.return_type.names.last().unwrap().extra.file.clone(),
                });
            }
        };

        for arg in self.arguments.iter() {
            let type_name = build_type_name(&arg.field_type);
            let field_type = match module.signatures.get(type_name.as_str()) {
                Some(signature) => signature,
                None => {
                    return Err(TirError::TypeNotFound {
                        source: arg.field_type.names.last().unwrap().extra.file.clone(),
                });
                }
            };

            arguments.push(field_type);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::process_code;

    #[test]
    fn found_type() -> Result<(), Box<dyn Error>> {
        let ast = process_code(vec!["source".to_string()], "class a {} func test(a: a): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn missing_type() -> Result<(), Box<dyn Error>> {
        let ast = process_code(vec!["source".to_string()], "func test(a: a): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
}
