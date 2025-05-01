use chrono::{DateTime, Utc};
use std::rc::Rc;
use std::cell::RefCell;
use rust_decimal::Decimal;
use crate::domain::account::Account;
use crate::domain::purchase::Purchase;



pub struct Charge<'a> {
	pub name: String,
	pub date_merchant_processed: DateTime<Utc>,
	pub date_account_processed: DateTime<Utc>,
	pub account: &'a Account,
	pub purchase: Rc<RefCell<Purchase<'a>>>,
	pub amount: Decimal
}



pub struct ChargeData<'a> {
	pub name: String,
	pub date_merchant_processed: DateTime<Utc>,
	pub date_account_processed: DateTime<Utc>,
	pub account: &'a Account,
	pub amount: Decimal
}