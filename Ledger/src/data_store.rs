use crate::domain::account::{Account, Institution};
use crate::domain::budget::Budget;



trait DataStore {
	
	fn get_institutions() -> Result<Vec<Institution>, String>;
	fn archive_institution() -> Result<(), String>;
	fn delete_institution() -> Result<(), String>;
	
	fn get_accounts() -> Result<Vec<Account>, String>;
	fn archive_account() -> Result<(), String>;
	fn delete_account() -> Result<(), String>;
	
	fn get_budgets() -> Result<Vec<Budget>, String>;
	fn archive_budget() -> Result<(), String>;
	fn delete_budget() -> Result<(), String>;
}