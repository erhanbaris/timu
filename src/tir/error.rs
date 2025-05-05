use std::rc::Rc;

use snafu::Snafu;

use crate::nom_tools::Span;

use super::signature::Signature;

#[derive(Debug, Snafu)]
pub enum TirError<'base> {
    #[snafu(visibility(pub), display("Module not found"))]
    ModuleNotFound { module: String },

    #[snafu(visibility(pub), display("Signature already defined"))]
    SignatureAlreadyDefined { old_signature: Rc<Signature<'base>> },

    #[snafu(visibility(pub), display("Module already defined"))]
    ModuleAlreadyDefined { old_signature: Rc<Signature<'base>> },

    #[snafu(visibility(pub), display("Module already defined"))]
    TypeNotFound { name: Span<'base> },
}
