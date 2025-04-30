use std::cell::RefCell;
use std::rc::Rc;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use crate::purchase::{MutablePurchase, Purchase};

mod purchase;
mod application;
mod merchant;
mod transfers;

fn main() {
	
	println!("Hello, world!");
	
	let test_purchase: Rc<RefCell<Purchase>> = purchase::create("name".to_string(), None, Utc::now());

	let account = Account { name: "account name".to_string(), institution: None };
	let budget = Budget { name: "budget name".to_string() };

	let charge_data = ChargeData {
		name: "charge name".to_string(),
		date_merchant_processed: Utc::now(),
		date_account_processed: Utc::now(),
		account: &account,
		amount: Decimal::new(1, 0),
	};

	let expense_data = ExpenseData {
		name: "".to_string(),
		budget: &budget,
		amount: Decimal::new(1, 0),
	};
	
	test_purchase.clone().add_charge(charge_data);
	test_purchase.clone().add_expense(expense_data);
	
	let test_transaction: Transaction = Transaction::Income(Income {
		name: "income name".to_string(),
		account: &account,
		date: Utc::now(),
		amount: Decimal::new(1, 0)
	});
	
	test_purchase.clone().add_associated_transaction(test_transaction)
}

struct Institution {
	name: String
}

struct Account<'a> {
	name: String,
	institution: Option<&'a Institution>
}

struct Budget {
	name: String,
}


struct Charge<'a> {
	name: String,
	date_merchant_processed: DateTime<Utc>,
	date_account_processed: DateTime<Utc>,
	account: &'a Account<'a>,
	purchase: Rc<RefCell<Purchase<'a>>>,
	amount: Decimal
}

struct ChargeData<'a> {
	name: String,
	date_merchant_processed: DateTime<Utc>,
	date_account_processed: DateTime<Utc>,
	account: &'a Account<'a>,
	amount: Decimal
}

struct Expense<'a> {
	name: String,
	budget: &'a Budget,
	purchase: Rc<RefCell<Purchase<'a>>>,
	amount: Decimal
}

struct ExpenseData<'a> {
	name: String,
	budget: &'a Budget,
	amount: Decimal
}

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

struct Income<'a> {
	name: String,
	account: &'a Account<'a>,
	date: DateTime<Utc>,
	amount: Decimal
}

enum Transaction<'a> {
	Income(Income<'a>),
	Expense(Expense<'a>)
}