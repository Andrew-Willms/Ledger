use chrono::{DateTime, Utc};
use crate::{Charge, ChargeData, Expense, ExpenseData, Merchant, Transaction};
use crate::Transaction::Expense;

pub(crate) struct Purchase<'a> {
    name: String,
    merchant: Option<&'a Merchant<'a>>,
    date: DateTime<Utc>,
    charges: Vec<Charge<'a>>,
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

impl Purchase<'_> {

    pub fn add_charge(&mut self, charge_data: ChargeData) {

        let charge = Charge {
            name: charge_data.name,
            date_merchant_processed: charge_data.date_account_processed,
            date_account_processed: charge_data.date_account_processed,
            account: charge_data.account,
            purchase: self,
            amount: charge_data.amount,
        };

        self.charges.push(charge);
    }

    pub fn add_expense(&mut self, expense_data: ExpenseData) {

        let expense = Expense {
            name: expense_data.name,
            budget: expense_data.budget,
            purchase: self,
            amount: expense_data.amount,
        };

        self.expenses.push(expense);
    }

    pub fn add_associated_transaction(&mut self) {

    }
}