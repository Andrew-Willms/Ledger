use rust_decimal::Decimal;
use crate::budget::Budget;
use crate::domain::account::{Account, Institution};
use crate::domain::merchant::Merchant;
use crate::domain::purchase::Purchase;

pub(crate) struct Application<'a> {
	
	state: State<'a>
}

pub(crate) struct State<'a> {
	
	institutions: Vec<Institution>,
	archived_institutions: Vec<Institution>,
	
	merchants: Vec<Merchant>,
	archived_merchants: Vec<Merchant>,
	
	budgets: Vec<(Budget, Decimal)>,
	archived_budgets: Vec<Budget>,
	
	account: Vec<(Account, Decimal)>,
	archived_accounts: Vec<Account>,
	
	history: Vec<Purchase<'a>>
}

pub fn create(state: State) -> Application {
	Application {
		state
	}
}

impl<'a> Application<'a> {
	
	fn add_institution(mut self, institution: Institution) {
		self.state.institutions.push(institution);
	}
	fn archive_institution() {}
	fn delete_institution() {}
	
	fn add_account() {}
	fn archive_account() {}
	fn delete_account() {}
	fn rename_account() {}
	fn change_account_institution() {}
	
	fn add_budget() {}
	fn archive_budget() {}
	fn delete_budget() {}
	fn rename_budget() {}
	fn change_budget_parent() {}
}