mod errors;
mod types;

use std::collections::HashMap;

use evalexpr::{
    build_operator_tree, eval_with_context_mut, ContextWithMutableVariables, HashMapContext,
};
use rustler::{Env, ResourceArc, Term};

struct PrecompiledExpression {
    operator_tree: evalexpr::Node,
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(PrecompiledExpression, env);
    true
}

#[rustler::nif]
fn eval<'a>(
    env: Env<'a>,
    expression: &str,
    context: HashMap<String, Term<'a>>,
) -> Result<Term<'a>, Term<'a>> {
    match eval_with_context_mut(expression, &mut build_hash_map_context(env, context)) {
        Ok(value) => Ok(types::from_value(env, &value)),
        Err(err) => Err(errors::to_error_tuple(env, err)),
    }
}

#[rustler::nif]
fn eval_precompiled_expression<'a>(
    env: Env<'a>,
    precompiled_expression: ResourceArc<PrecompiledExpression>,
    context: HashMap<String, Term<'a>>,
) -> Result<Term<'a>, Term<'a>> {
    match precompiled_expression
        .operator_tree
        .eval_with_context_mut(&mut build_hash_map_context(env, context))
    {
        Ok(value) => Ok(types::from_value(env, &value)),
        Err(err) => Err(errors::to_error_tuple(env, err)),
    }
}

#[rustler::nif]
fn precompile_expression<'a>(
    env: Env<'a>,
    expression: &str,
) -> Result<ResourceArc<PrecompiledExpression>, Term<'a>> {
    match build_operator_tree(expression) {
        Ok(operator_tree) => {
            let data = PrecompiledExpression { operator_tree };

            let arc = ResourceArc::new(data);
            Ok(arc)
        }
        Err(err) => Err(errors::to_error_tuple(env, err)),
    }
}

fn build_hash_map_context<'a>(env: Env<'a>, context: HashMap<String, Term<'a>>) -> HashMapContext {
    let mut hash_map_context = HashMapContext::new();

    for (k, v) in context {
        hash_map_context.set_value(k, types::to_value(env, &v)).ok();
    }

    hash_map_context
}

rustler::init!(
    "Elixir.EvalEx.Native",
    [eval, eval_precompiled_expression, precompile_expression],
    load = load
);
