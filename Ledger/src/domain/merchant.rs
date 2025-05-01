use std::cell::RefCell;
use std::rc::Rc;

pub struct Merchant<> {
	name: String,
	address: Option<String>,
	parent: Option<Rc<RefCell<Merchant>>>,
	children: Vec<Merchant>,
}