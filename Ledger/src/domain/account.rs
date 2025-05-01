use std::cell::RefCell;
use std::rc::Rc;

pub struct Institution {
	pub name: String
}

pub struct Account {
	pub name: String,
	pub institution: Option<Rc<RefCell<Institution>>>
}