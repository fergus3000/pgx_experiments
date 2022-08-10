use pgx::*;

pg_module_magic!();

#[derive(Default)] // this gives our struct an implicit default. Other things you could derive, but not needed here
pub struct IntegerAvgState { // this is our state for aggregation, not necessarily what we will return
    sum: i32,
    n: i32,
    min: i32,
    max: i32
}

// define implementation for our aggregate
#[pg_aggregate]
impl Aggregate for IntegerAvgState 
{
    const NAME: &'static str = "summary_stats_pgx";
    type State = Internal;
    type Args = i32; // we just have 1 argument, an int32. Typical invocation: SELECT summary_stats_pgx(some_integer_column) FROM some_table;
    type Finalize = pgx::JsonB; // the Finalize declares what we will return from our aggregate function - we're going to return a jsonb


    #[pgx(immutable)]
    fn state(
        mut current: Self::State,
        val: Self::Args, // here's that arg as declared above
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State 
    {
        let inner = unsafe { current.get_or_insert_default::<IntegerAvgState>() }; // if its our first row, we get a default state. Default impl is delivered courtesy of #[derive(Default)]
        // implementation starts here...
        inner.sum += val;
        inner.n += 1;
        if (val < inner.min) {
            inner.min = val;
        }
        if (val > inner.max) {
            inner.max = val;
        }
        current
    }

    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pgx::pg_sys::FunctionCallInfo,
    ) -> Self::Finalize 
    {
        let inner = unsafe { current.get::<Self>().unwrap() };
        // convert to jsonb for return
        // todo: return sum,min,max,count as an array. Just want to test if this works first :)
        pgx::JsonB(serde_json::json!({
            "sum" : inner.sum,
            "min" : inner.min,
            "max" : inner.max,
            "count": inner.n,
        }))
    }
}