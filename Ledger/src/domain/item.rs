use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::domain::purchase::Purchase;



struct Item<'a> {
	name: String,
	notes: String,
	purchase: Purchase<'a>,

	date_order_processed: Option<DateTime<Utc>>,
	date_shipped: Option<DateTime<Utc>>,
	date_received: Option<DateTime<Utc>>,

	regular_price: Decimal,
	sale_price: Decimal,
	effective_price: Decimal
}