# agg_summary_stats
Trying to create my own aggregate pgx function to obtain the Sum,Min,Max,Count for an aggregate as an array. Project is based on the max_timed example, and is usable but incomplete.

So far, it creates a summary stats aggregate that handles int32s, and returns Jsonb containing min, max, sum, count.

Usage:
Build and run the project with:

`cargo pgx run pg13 --release`

Then within the psql prompt setup some data:

```
CREATE TABLE public.some_timeseries
(
    event_time timestamp with time zone NOT NULL,
    timeseries_id integer NOT NULL,
    event_data integer NOT NULL,
    CONSTRAINT some_timeseries_pkey PRIMARY KEY (timeseries_id, event_time)
)
TABLESPACE pg_default;

INSERT INTO public.some_timeseries (timeseries_id, event_time, event_data) 
SELECT 123,x,(random()*100)::int
FROM generate_series('2020-01-01'::timestamptz,'2020-01-01'::timestamptz + ((1000000 -1) * interval '1 second'), '1 second'::interval) AS x; 
```

A simple query would be:

`SELECT summary_stats_pgx(event_data) FROM some_timeseries;`

Or to show the 1 minute aggregates on a given timeseries:

```
SELECT ts.timeseries_id, date_trunc('minute', ts.event_time) AS agg_interval, summary_stats_pgx(ts.event_data)
FROM some_timeseries ts
WHERE timeseries_id = 123
GROUP BY timeseries_id, agg_interval;
```