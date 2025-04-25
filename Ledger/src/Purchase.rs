use std::cell::{RefCell};
use std::rc::Rc;
use chrono::{DateTime, Utc};
use crate::{Charge, ChargeData, Expense, ExpenseData, Merchant, Transaction};

pub(crate) struct Purchase<'a> {
	pub name: String,
	merchant: Option<&'a Merchant<'a>>,
	date: DateTime<Utc>,
	pub charges: Vec<Charge<'a>>,
	expenses: Vec<Expense<'a>>,
	associated_transactions: Vec<Transaction<'a>>
}

pub fn create<'a>(name: String, merchant: Option<&'a Merchant<'a>>, date: DateTime<Utc>) -> Rc<RefCell<Purchase<'a>>> {
	
	let purchase = Purchase {
		name,
		merchant,
		date,
		charges: vec![],
		expenses: vec![],
		associated_transactions: vec![],
	};
	
	Rc::new(RefCell::new(purchase))
}

pub trait PurchaseStuff {
	fn add_charge(self, charge_data: ChargeData);
	fn add_expense(self, expense_data: ExpenseData);
	fn add_associated_transaction(self, transaction: Transaction);
}

impl PurchaseStuff for Rc<RefCell<Purchase<'_>>> {
	
	fn add_charge(self, charge_data: ChargeData) {

		let ref_cell_clone = Rc::clone(&self);

		self.borrow_mut().charges.push(Charge {
			name: charge_data.name,
			date_merchant_processed: charge_data.date_account_processed,
			date_account_processed: charge_data.date_account_processed,
			account: charge_data.account,
			purchase: ref_cell_clone,
			amount: charge_data.amount,
		});
	}

	fn add_expense<'a>(self, expense_data: ExpenseData<'a>) {
		
		let ref_cell_clone = Rc::clone(&self);

		self.borrow_mut().expenses.push(Expense {
			name: expense_data.name,
			budget: expense_data.budget,
			purchase: ref_cell_clone,
			amount: expense_data.amount,
		});
	}

	fn add_associated_transaction<'a>(self, transaction: Transaction<'a>) {
		self.borrow_mut().associated_transactions.push(transaction);
	}

}