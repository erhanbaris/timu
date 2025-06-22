use std::{borrow::Cow, fmt::Debug};

use libtimu_macros::TimuError;
use simplelog::debug;
use strum_macros::{EnumDiscriminants, EnumProperty};

use crate::{map::{TimuHashMap, ValueTrait}, nom_tools::{Span, SpanInfo}, tir::resolver::{AstSignatureLocation, BuildFullNameLocater, ResolveAst}};

use super::{module::ModuleRef, resolver::{ResolverError, TypeLocation}, signature::LocationTrait, TirContext, TirError};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ScopeLocation(#[allow(dead_code)]pub usize);

impl ScopeLocation {
    pub const UNDEFINED: Self = ScopeLocation(usize::MAX);
}

impl From<usize> for ScopeLocation {
    fn from(signature_location: usize) -> Self {
        ScopeLocation(signature_location)
    }
}

impl LocationTrait for ScopeLocation {
    fn get(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableInformation<'base, L: Debug + Clone + PartialEq> {
    pub span: Span<'base>,
    pub location: L,
    pub nullable: bool,
    pub reference: bool,
    pub readonly: bool,
}

impl<'base, L> ValueTrait<'base> for VariableInformation<'base, L>
where L: Debug + Clone + PartialEq
{
    fn get_span(&self) -> Span<'base> {
        self.span.clone()
    }
}

impl<'base, L> VariableInformation<'base, L> 
where L: Debug + Clone + PartialEq
{
    pub fn new(span: Span<'base>, location: L, nullable: bool, reference: bool, readonly: bool) -> Self {
        Self {
            span,
            location,
            nullable,
            reference,
            readonly,
        }
    }

    pub fn basic(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, false, false, false)
    }

    pub fn readonly(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, false, false, true)
    }

    pub fn reference(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, false, true, false)
    }

    pub fn nullable(span: Span<'base>, location: L) -> Self {
        Self::new(span, location, true, false, false)
    }
}

pub type TypeVariableInformation<'base> = VariableInformation<'base, TypeLocation>;
pub type AstVariableInformation<'base> = VariableInformation<'base, AstSignatureLocation>;

#[derive(Debug, Clone)]
pub struct Scope<'base> {
    pub module_ref: ModuleRef<'base>,
    variables: TimuHashMap<'base, Cow<'base, str>, TypeVariableInformation<'base>>,
    pub parent_scope: Option<ScopeLocation>,
    pub parent_type: Option<TypeLocation>,
    pub current_type: TypeLocation,
    pub location: ScopeLocation,
}

impl<'base> Scope<'base> {
    pub fn new(module_ref: ModuleRef<'base>, parent_scope: Option<ScopeLocation>,  parent_type: Option<TypeLocation>, location: ScopeLocation) -> Self {
        Self {
            module_ref,
            variables: TimuHashMap::new(),
            parent_scope,
            parent_type,
            location,
            current_type: TypeLocation::UNDEFINED
        }
    }

    pub fn get_variable<T: AsRef<str>>(&self, context: &TirContext<'base>, name: T) -> Option<TypeVariableInformation<'base>> {
        debug!("get_variable: name: {}, scope: {}", name.as_ref(), self.location.0);

        /* Search in current scope */
        if let Some(variable) = self.variables.get(name.as_ref()) {
            return Some(variable.clone());
        }

        /* Search in parent scope */
        if let Some(type_location) = self.parent_scope.and_then(|parent_location| context.get_scope(parent_location)).and_then(|parent_scope| parent_scope.get_variable(context, name.as_ref())) {
            return Some(type_location);
        }

        /* Search in module level */
        let module = self.module_ref.upgrade(context).unwrap();
        if let Some(type_location) = module.types.get(name.as_ref()) {
            println!("0 Found type location in module: {:?}", type_location);
            //return Some(*type_location);
        }

        let module_scope = context.get_scope(module.scope_location).expect(&format!("Module scope not found for module: {}", module.path));

        if module_scope.location != self.location {
            if let Some(variable_info) = module_scope.get_variable(context, name.as_ref()) {
                return Some(variable_info);
            }
        }

        /* Search as a module name  */
        if let Some(module_ref) = context.modules.get(name.as_ref()) {
            if let Some(type_location) = module.types.get(module_ref.path.as_ref()) {
                println!("1 Found type location in module: {:?}", type_location);
                //return Some(*type_location);
                return None;
            }
        }

        if let Some(type_location) = context.types.location(name.as_ref()) {
            println!("2 Found type location in module: {:?}", type_location);
            //return Some(type_location);
        }

        if let Some(ast_location) = module.ast_imported_modules.get(name.as_ref()) {
            if let Some(signature) = context.ast_signatures.get_from_location(*ast_location) {
                let full_name = signature.value.build_full_name(context, BuildFullNameLocater::Module(signature.extra.as_ref().unwrap()), None);

                if let Some(type_location) = context.types.location(full_name.as_str()) {

                    println!("3 Found type location: {:?}", type_location);
                    //return Some(type_location)
                }
            }
        }

        if let Some(module_ref) = context.modules.get(name.as_ref()) {
            if let Some(type_location) = module.types.get(module_ref.path.as_ref()) {
                println!("4 Found type location in module: {:?}", type_location);
                //return Some(*type_location);
            }
        }

        None
    }

    pub fn add_variable(&mut self, variable: TypeVariableInformation<'base>) -> Result<(), TirError> {
        simplelog::debug!("Adding variable: <u><b><on-green>{}</></b></u>, location <u><b>{:?}</b></u>, scope: {}", variable.span.text, variable.location, self.location.0);
        self.variables.validate_insert((*variable.span.text).into(), variable)?;
        Ok(())
    }

    pub fn set_current_type(&mut self, type_location: TypeLocation) {
        self.current_type = type_location;
    }
}

#[derive(Clone, Debug, TimuError, thiserror::Error, EnumDiscriminants, EnumProperty)]
pub enum ScopeError {
    #[error("Variable already defined")]
    VariableAlreadyDefined(SpanInfo),
}

impl From<ScopeError> for TirError {
    fn from(value: ScopeError) -> Self {
        ResolverError::Scope(Box::new(value)).into()
    }
}
