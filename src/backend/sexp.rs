//! Pretty-printer for Calyx syntax.
//! Outputs s-expressions.

use crate::backend::traits::Backend;
use calyx_ir as ir;
use calyx_utils::{CalyxResult, OutputFile};

#[derive(Default)]
pub struct SexpBackend;

impl Backend for SexpBackend {
    fn name(&self) -> &'static str {
        "sexp"
    }

    /// OK to run this analysis on any Calyx program
    fn validate(_ctx: &ir::Context) -> CalyxResult<()> {
        Ok(())
    }

    /// Don't need to take care of this for this pass
    fn link_externs(
        _ctx: &ir::Context,
        _file: &mut OutputFile,
    ) -> CalyxResult<()> {
        Ok(())
    }

    #[cfg(feature = "serialize")]
    fn emit(ctx: &ir::Context, file: &mut OutputFile) -> CalyxResult<()> {
        let out = &mut file.get_write();
        writeln!(out, "{}", serde_sexpr::to_string(ctx).unwrap())?;

        Ok(())
    }

    #[cfg(not(feature = "serialize"))]
    fn emit(_ctx: &ir::Context, file: &mut OutputFile) -> CalyxResult<()> {
        let out = &mut file.get_write();
        writeln!(out, "The serialize feature is not enabled. To compile Calyx with the serialize feature, run `cargo build --features serialize`")?;
        Ok(())
    }
}
