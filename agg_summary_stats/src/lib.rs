use pgx::*;

pg_module_magic!();

#[derive(Default)] // this gives our struct an implicit default. Other things you could derive, but not needed here
pub struct IntegerAvgState { // this is our state for aggregation, not necessarily what we will return
    sum: f64,
    n: i64,
    min: f64,
    max: f64
}

// define implementation for our aggregate
#[pg_aggregate]
impl Aggregate for IntegerAvgState 
{
    const NAME: &'static str = "summary_stats_pgx";
    type State = Internal;
    type Args = f64; // we just have 1 argument, an double(f64). Typical invocation: SELECT summary_stats_pgx(some_double_column) FROM some_table;
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

        if !val.is_nan() { // check for NaN values, we exclude them from our stats
            inner.sum += val;
            // better if I knew how to set IntegerAvgState's default min and max to NaN
            if val < inner.min || inner.n == 0 {
                inner.min = val;
            }
            if val > inner.max || inner.n == 0 {
                inner.max = val;
            }
            inner.n += 1;
        }
        // return current state and continue
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
        pgx::JsonB(serde_json::json!({
            "sum" : inner.sum,
            "min" : inner.min,
            "max" : inner.max,
            "count": inner.n,
        }))
    }
}