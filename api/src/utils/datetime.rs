use chrono::{DateTime, FixedOffset, Utc};

pub fn db_now_datetime() -> DateTime<FixedOffset> {
	Utc::now().with_timezone(&FixedOffset::east_opt(3 * 3600).unwrap())
}
