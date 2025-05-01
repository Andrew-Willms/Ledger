use std::rc::Rc;
use std::cell::RefCell;

pub struct Budget {
	pub name: String,
	parent: Option<Rc<RefCell<Budget>>>,
	children: Vec<Budget>
}

pub fn new(name: String, parent: Option<Rc<RefCell<Budget>>>, children: Vec<Budget>) -> Budget {
	return Budget {
		name,
		parent,
		children
	}
}

impl Budget {

	pub fn add_child(&mut self, child: Budget) {
		self.children.push(child)
	}
	
	// pub fn remove_child(&mut self, child: Budget) -> Budget {
	//
	// 	self.children.iter().position(|x| x == child).unwrap();
	//
	// 	self.children.find
	// }
	
}