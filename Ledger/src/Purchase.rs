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

pub fn add_charge<'a>(purchase: Rc<RefCell<Purchase<'a>>>, charge_data: ChargeData<'a>) {

    let shared_pointer = Rc::clone(&purchase);

    purchase.borrow_mut().charges.push(Charge {
        name: charge_data.name,
        date_merchant_processed: charge_data.date_account_processed,
        date_account_processed: charge_data.date_account_processed,
        account: charge_data.account,
        purchase: shared_pointer,
        amount: charge_data.amount,
    });
}

pub fn add_expense<'a>(purchase: Rc<RefCell<Purchase<'a>>>, expense_data: ExpenseData<'a>) {

    let shared_pointer = Rc::clone(&purchase);

    purchase.borrow_mut().expenses.push(Expense {
        name: expense_data.name,
        budget: expense_data.budget,
        purchase: shared_pointer,
        amount: expense_data.amount,
    });
}

impl<'a> Purchase<'a> {

    pub fn add_associated_transaction(mut self, transaction: Transaction<'a>) {
        self.associated_transactions.push(transaction);
    }
}