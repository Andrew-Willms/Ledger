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

pub fn create<'a>(name: String, merchant: Option<&'a Merchant<'a>>, date: DateTime<Utc>) -> Purchase<'a> {

    Purchase {
        name,
        merchant,
        date,
        charges: vec![],
        expenses: vec![],
        associated_transactions: vec![],
    }
}

impl<'a> Purchase<'a> {

    pub fn add_charge(&self, charge_data: ChargeData<'a>) {

        let shared_pointer1 = Rc::new(RefCell::new(self));
        let shared_pointer2 = Rc::clone(&shared_pointer1);

        let charge = Charge {
            name: charge_data.name,
            date_merchant_processed: charge_data.date_account_processed,
            date_account_processed: charge_data.date_account_processed,
            account: charge_data.account,
            purchase: shared_pointer1,
            amount: charge_data.amount,
        };

        shared_pointer2.borrow_mut().charges.push(charge);
    }

    pub fn add_expense(&self, expense_data: ExpenseData<'a>) {

        let shared_pointer1 = Rc::new(RefCell::new(self));
        let shared_pointer2 = Rc::clone(&shared_pointer1);
        
        let expense = Expense {
            name: expense_data.name,
            budget: expense_data.budget,
            purchase: shared_pointer1,
            amount: expense_data.amount,
        };

        shared_pointer2.borrow_mut().expenses.push(expense);
    }

    pub fn add_associated_transaction(mut self, transaction: Transaction<'a>) {
        self.associated_transactions.push(transaction);
    }
}