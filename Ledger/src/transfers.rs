use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::{Account, Budget};

struct AccountTransfer<'a> {
	name: String,
	date_initiated: DateTime<Utc>,
	date_sent: DateTime<Utc>,
	date_received: DateTime<Utc>,
	source_account: &'a Account<'a>,
	destination_account: &'a Account<'a>,
	amount: Decimal
}

struct BudgetTransfer {
	name: String,
	date: DateTime<Utc>,
	source_budget: Budget,
	destination_budget: Budget,
	amount: Decimal
}