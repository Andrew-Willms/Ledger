use std::cell::RefCell;
use std::rc::Rc;
use chrono::Utc;
use rust_decimal::Decimal;
use crate::domain::{budget};
use crate::domain::account::{Account};
use crate::domain::budget::{Budget};
use crate::domain::charge::{ChargeData};
use crate::domain::expense::{ExpenseData};
use crate::domain::income::{Income};
use crate::domain::transaction::{Transaction};
use crate::domain::purchase::{MutablePurchase, Purchase};

mod domain;
mod application;
mod data_store;



fn main() {
	
	println!("Hello, world!");
	
	let test_purchase: Rc<RefCell<Purchase>> = domain::purchase::create("name".to_string(), None, Utc::now());

	let account = Account {
		name: "account name".to_string(),
		institution: None
	};
	
	let budget: Budget = budget::create("budget name".to_string(), None, vec![]);

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