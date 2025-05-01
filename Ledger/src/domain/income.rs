use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::domain::account::Account;



pub struct Income<'a> {
	pub name: String,
	pub account: &'a Account,
	pub date: DateTime<Utc>,
	pub amount: Decimal
}