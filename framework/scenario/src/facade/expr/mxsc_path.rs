use multiversx_chain_scenario_format::{
    interpret_trait::InterpreterContext, value_interpreter::interpret_string,
};
use multiversx_sc::types::{AnnotatedValue, ManagedBuffer, TxCodeValue};

use crate::{ScenarioTxEnv, ScenarioTxEnvData};

use super::RegisterCodeSource;

const MXSC_PREFIX: &str = "mxsc:";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MxscPath<'a> {
    path: &'a str,
}

impl<'a> MxscPath<'a> {
    pub const fn new(path: &'a str) -> Self {
        MxscPath { path }
    }
}

impl MxscPath<'_> {
    pub fn eval_to_expr(&self) -> String {
        format!("{MXSC_PREFIX}{}", self.path_without_prefix())
    }

    pub fn resolve_contents(&self, context: &InterpreterContext) -> Vec<u8> {
        interpret_string(&self.eval_to_expr(), context)
    }

    fn path_without_prefix(&self) -> &str {
        self.path.strip_prefix(MXSC_PREFIX).unwrap_or(self.path)
    }
}

impl<Env> AnnotatedValue<Env, ManagedBuffer<Env::Api>> for MxscPath<'_>
where
    Env: ScenarioTxEnv,
{
    fn annotation(&self, _env: &Env) -> ManagedBuffer<Env::Api> {
        self.eval_to_expr().into()
    }

    fn to_value(&self, env: &Env) -> ManagedBuffer<Env::Api> {
        self.resolve_contents(&env.env_data().interpreter_context())
            .into()
    }
}

impl<Env> TxCodeValue<Env> for MxscPath<'_> where Env: ScenarioTxEnv {}

impl RegisterCodeSource for MxscPath<'_> {
    fn into_code(self, env_data: ScenarioTxEnvData) -> Vec<u8> {
        self.resolve_contents(&env_data.interpreter_context())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::imports::MxscPath;

    fn assert_eq_eval(expr: &'static str, expected: &str) {
        assert_eq!(&MxscPath::new(expr).eval_to_expr(), expected);
    }

    #[test]
    fn test_address_value() {
        assert_eq_eval("output/adder.mxsc.json", "mxsc:output/adder.mxsc.json");
    }

    #[test]
    fn test_already_prefixed_path_is_not_prefixed_twice() {
        assert_eq_eval("mxsc:output/adder.mxsc.json", "mxsc:output/adder.mxsc.json");
    }
}
