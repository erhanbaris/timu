use std::{cell::RefMut, rc::Rc};

use crate::{ast::FunctionDefinitionAst, nom_tools::{Span, ToRange}, tir::{context::TirContext, module::Module, AstSignature, ObjectSignature, TirError}};

use super::{build_type_name, ResolveSignature};


#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionArgument<'base> {
    pub name: Span<'base>,
    pub field_type: Rc<ObjectSignature<'base>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FunctionDefinition<'base> {
    pub is_public: bool,
    pub name: Span<'base>,
    pub arguments: Vec<FunctionArgument<'base>>,
    pub return_type: Rc<AstSignature<'base>>,
    // pub body: BodyAst<'base>,
}

impl<'base> ResolveSignature<'base> for FunctionDefinitionAst<'base> {
    type Result = FunctionDefinition<'base>;

    fn resolve(&self, _: &'_ TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Result, TirError<'base>> {
        let mut arguments = vec![];

        let return_type_name = build_type_name(&self.return_type);
        let return_type = match module.imported_modules.get(return_type_name.as_str()) {
            Some(return_type) => return_type.clone(),
            None => {
                return Err(TirError::TypeNotFound {
                    source: self.return_type.names.last().unwrap().extra.file.clone(),
                    position: self.return_type.to_range(),
                });
            }
        };

        for arg in self.arguments.iter() {
            let type_name = build_type_name(&arg.field_type);
            let field_type = match module.imported_modules.get(type_name.as_str()) {
                Some(field_type) => field_type.clone(),
                None => {
                    return Err(TirError::TypeNotFound {
                        source: arg.field_type.names.last().unwrap().extra.file.clone(),
                        position: arg.field_type.to_range(),
                });
                }
            };

            if arguments.iter().any(|item: &FunctionArgument| item.name.fragment() == arg.name.fragment()) {
                return Err(TirError::AlreadyDefined {
                    position: arg.name.to_range(),
                    source: arg.name.extra.file.clone(),
                });
            }

            arguments.push(FunctionArgument {
                name: arg.name.clone(),
                field_type,
            });
        }

        Ok(FunctionDefinition {
            is_public: self.is_public.is_some(),
            name: self.name.clone(),
            arguments,
            return_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{process_code, tir::TirError};

    #[test]
    fn missing_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".to_string()], "func test(a: a): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap_err();
        Ok(())
    }
    
    #[test]
    fn dublicated_function_argument() -> Result<(), ()> {
        let ast = process_code(vec!["source".to_string()], "class a {} func test(a: a, a: a): a {} ")?;
        let error = crate::tir::build(vec![ast.into()]).unwrap_err();

        if let TirError::AlreadyDefined { position, source } = error {
            assert_eq!(position, 27..28);
            assert_eq!(source.path().join("/"), "source");
        } else {
            panic!("Expected TirError::AlreadyDefined but got {:?}", error);
        }
        Ok(())
    }
}
