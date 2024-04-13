use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn utc_time_to_naive_moscow(dt: DateTime<Utc>) -> NaiveDateTime {
    let moscow_offset = moscow_offset();

    dt.with_timezone(&moscow_offset).naive_local()
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn naive_moscow_time_to_utc(dt: NaiveDateTime) -> DateTime<Utc> {
    let moscow_offset = moscow_offset();

    // Можно unwrap, так как смещение верное
    dt.and_local_timezone(moscow_offset)
        .single()
        .unwrap()
        .to_utc()
}

///////////////////////////////////////////////////////////////////////////////////////////////

// Фиксированный оффсет, можно unwrap()
fn moscow_offset() -> FixedOffset {
    FixedOffset::west_opt(60 * 60 * 3).unwrap()
}
