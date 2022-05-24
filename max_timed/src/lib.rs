use pgx::*;

pg_module_magic!();

// define state for our max timed aggregate
pub struct MaxTimedState {
    max: Option<i32>,
    timestamp: Option<pg_sys::TimestampTz>,
}

// define implementation for our aggregate
#[pg_aggregate]
impl Aggregate for MaxTimedState {
    const NAME: &'static str = "max_timed_pgx";
    type State = Internal;
    // typical invocation SELECT max_timed_pgx(instant, temperature) FROM weather_data;
    type Args = (
        pgx::name!(timestamp, Option<pg_sys::TimestampTz>),
        pgx::name!(max, Option<i32>),
    );
    type Finalize = pgx::JsonB;


    #[pgx(immutable)]
    fn state(
        mut current: Self::State,
        (timestamp, max): Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let inner = unsafe { current.get_or_insert(Self {max, timestamp })};
        // implementation starts here...
        if max > inner.max {
            inner.max = max;
            inner.timestamp = timestamp;
        }
        current
    }

    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pgx::pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        let inner = unsafe { current.get::<Self>().unwrap() };
        // convert to jsonb for return
        pgx::JsonB(serde_json::json!({
            "max": inner.max,
            "timestamp": inner.timestamp.map(|t| pgx::TimestampWithTimeZone::from(t))
        }))
    }
}