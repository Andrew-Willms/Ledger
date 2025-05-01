use crate::domain::expense::Expense;
use crate::domain::income::Income;



pub enum Transaction<'a> {
	Income(Income<'a>),
	Expense(Expense<'a>)
}