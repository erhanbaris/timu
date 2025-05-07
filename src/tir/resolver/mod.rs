use std::{cell::{RefCell, RefMut}, fmt::Debug, rc::Rc};

use crate::{ast::TypeNameAst, nom_tools::Span};

use super::{context::TirContext, error::TirError, module::Module};

pub mod function_definition;
mod module_use;

pub trait ResolveSignature<'base> {
    type Result;
    fn resolve(&self, context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>) -> Result<Self::Result, TirError<'base>>;
}

fn build_type_name(ast: &TypeNameAst) -> String {
    ast.names.iter().map(|path| *path.fragment()).collect::<Vec<&str>>().join(".")
}

pub fn build_file<'base>(context: &mut TirContext<'base>, module: Rc<RefCell<Module<'base>>>) -> Result<(), TirError<'base>> {
    let uses = module.borrow().ast.get_uses().collect::<Vec<_>>();
    let functions = module.borrow().ast.get_functions().collect::<Vec<_>>();
    
    let mut module = module.borrow_mut();

    for use_item in uses {
        use_item.resolve(context, &mut module)?;
        //try_resolve_signature(context, &mut module, use_item)?;
    }

    for function in functions {
        function.resolve(context, &mut module)?;
    }

    Ok(())
}

pub fn try_resolve_signature<'base, T: ResolveSignature<'base>>(context: &TirContext<'base>, module: &mut RefMut<'_, Module<'base>>, signature: Rc<T>) -> Result<T::Result, TirError<'base>> {
    signature.resolve(context, module)
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

#[cfg(test)]
mod tests {
    use crate::{process_ast, process_code};

    #[test]
    fn found_type() -> Result<(), ()> {
        let ast = process_code(vec!["source".to_string()], "class a {} func test(a: a): a {} ")?;
        crate::tir::build(vec![ast.into()]).unwrap();
        Ok(())
    }

    #[test]
    fn cross_reference() -> Result<(), ()> {
        let ast_1 = process_code(vec!["source1".to_string()], " class testclass1 {} ")?;
        let ast_9 = process_code(
            vec!["sub".to_string(), "source9".to_string()],
            r#"use source1.testclass1;
    func testfunction1(): testclass1 {}"#,
        )?;

        process_ast(vec![ast_1.into(), ast_9.into()])?;

        Ok(())
    }

}
