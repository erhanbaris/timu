use std::rc::Rc;

use snafu::Snafu;

use super::signature::ModuleSignature;

#[derive(Debug, Snafu)]
pub enum TirError<'base> {
    #[snafu(visibility(pub), display("Module not found"))]
    ModuleNotFound { module: String },

    #[snafu(visibility(pub), display("Signature already defined"))]
    SignatureAlreadyDefined { old_signature: Rc<ModuleSignature<'base>> },

    #[snafu(visibility(pub), display("Module already defined"))]
    ModuleAlreadyDefined { old_signature: Rc<ModuleSignature<'base>> },
}
