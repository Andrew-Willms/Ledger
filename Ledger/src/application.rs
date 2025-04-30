use rust_decimal::Decimal;
use crate::{Account, Budget, Institution};
use crate::merchant::{Merchant, MerchantFamily};
use crate::purchase::Purchase;

pub(crate) struct Application<'a> {
	
	state: State<'a>
}

pub(crate) struct State<'a> {
	
	institutions: Vec<Institution>,
	merchant_families: Vec<MerchantFamily>,
	merchants: Vec<Merchant<'a>>,
	
	budgets: Vec<(Budget, Decimal)>,
	account: Vec<(Account<'a>, Decimal)>,
	
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
	
	fn archive_institution() {
	
	}
	
	fn delete_institution() {
	
	}
	
	
	
	fn add_budget() {
	
	}
	
	fn add_account() {
	
	}
	
	fn

	
	
}

trait DataStore {
	
	fn get_accounts<'a>() -> Vec<Account<'a>>;
	
	
	
}