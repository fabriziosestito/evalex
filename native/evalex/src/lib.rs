mod errors;
mod types;

use evalexpr::{
    build_operator_tree, eval_with_context_mut, ContextWithMutableVariables, HashMapContext,
};
use rustler::{Encoder, Env, MapIterator, ResourceArc, Term};

struct PrecompiledExpression {
    operator_tree: evalexpr::Node,
}

fn load(env: Env, _: Term) -> bool {
    rustler::resource!(PrecompiledExpression, env);
    true
}

#[rustler::nif]
fn eval<'a>(env: Env<'a>, string: &str, context: Term<'a>) -> Result<Term<'a>, Term<'a>> {
    let mut hash_map_context = HashMapContext::new();

    MapIterator::new(context)
        .expect("Should be a map in the argument")
        .for_each(|(k, v)| {
            let key: String = k.decode().expect("Should be a string");

            hash_map_context
                .set_value(key, types::to_value(env, &v))
                .ok();
        });

    match eval_with_context_mut(string, &mut hash_map_context) {
        Ok(value) => Ok(types::from_value(env, &value)),
        Err(err) => Err(errors::to_error_tuple(env, err)),
    }
}

#[rustler::nif]
fn eval_precompiled_expression<'a>(
    env: Env<'a>,
    precompiled_expression: ResourceArc<PrecompiledExpression>,
    context: Term<'a>,
) -> Result<Term<'a>, Term<'a>> {
    let mut hash_map_context = HashMapContext::new();

    MapIterator::new(context)
        .expect("Should be a map in the argument")
        .for_each(|(k, v)| {
            let key: String = k.decode().expect("Should be a string");

            hash_map_context
                .set_value(key, types::to_value(env, &v))
                .ok();
        });
    let ot = &precompiled_expression.operator_tree;

    match ot.eval_with_context_mut(&mut hash_map_context) {
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

rustler::init!(
    "Elixir.EvalEx.Native",
    [eval, eval_precompiled_expression, precompile_expression],
    load = load
);
