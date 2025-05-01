use rust_decimal::Decimal;
use std::rc::Rc;
use std::cell::RefCell;
use crate::budget::Budget;
use crate::domain::purchase::Purchase;



pub struct Expense<'a> {
	pub name: String,
	pub budget: &'a Budget,
	pub purchase: Rc<RefCell<Purchase<'a>>>,
	pub amount: Decimal
}



pub struct ExpenseData<'a> {
	pub name: String,
	pub budget: &'a Budget,
	pub amount: Decimal
}