use pgx::*;
use std::collections::HashSet;

pg_module_magic!();

#[derive(Copy, Clone, Default, Debug)]
pub struct DemoUnique;

#[pg_aggregate]
impl Aggregate for DemoUnique {
    const NAME: &'static str = "unique_string_pgx";
    type Args = &'static str;
    type State = Internal;
    type Finalize = i32;

    fn state(
        mut current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let inner = unsafe { current.get_or_insert_default::<HashSet<String>>() };

        inner.insert(arg.to_string());
        current
    }

    fn combine(
        mut first: Self::State,
        mut second: Self::State,
        _fcinfo: pg_sys::FunctionCallInfo
    ) -> Self::State {
        let first_inner = unsafe { first.get_or_insert_default::<HashSet<String>>() };
        let second_inner = unsafe { second.get_or_insert_default::<HashSet<String>>() };

        let unioned: HashSet<_> = first_inner.union(second_inner).collect();
        Internal::new(unioned)
    }

    fn finalize(
        mut current: Self::State,
        _direct_arg: Self::OrderedSetArgs,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        let inner = unsafe { current.get_or_insert_default::<HashSet<String>>() };

        inner.len() as i32
    }
}