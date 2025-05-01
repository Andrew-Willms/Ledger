use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::budget::Budget;
use crate::domain::account::Account;



pub struct AccountTransfer<'a> {
	name: String,
	date_initiated: DateTime<Utc>,
	date_sent: DateTime<Utc>,
	date_received: DateTime<Utc>,
	source_account: &'a Account,
	destination_account: &'a Account,
	amount: Decimal
}

pub struct BudgetTransfer {
	name: String,
	date: DateTime<Utc>,
	source_budget: Budget,
	destination_budget: Budget,
	amount: Decimal
}