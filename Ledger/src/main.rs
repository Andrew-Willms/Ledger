use std::cell::RefCell;
use std::rc::Rc;
use chrono::Utc;
use eframe::{egui, AppCreator, CreationContext, NativeOptions};
use eframe::egui::{ScrollArea, TextStyle};
use rust_decimal::Decimal;
use crate::domain::{budget};
use crate::domain::account::{Account};
use crate::domain::budget::{Budget};
use crate::domain::charge::{ChargeData};
use crate::domain::expense::{ExpenseData};
use crate::domain::income::{Income};
use crate::domain::transaction::{Transaction};
use crate::domain::purchase::{MutablePurchase, Purchase};

mod domain;
mod application;
mod data_store;



fn main() {
	
	println!("Hello, world!");
	
	let app_name: &String = &"My egui App".to_string();
	let native_options: NativeOptions = NativeOptions::default();
	let app_creator: AppCreator = Box::new(|cc: &CreationContext| Ok(Box::new(MyEguiApp::new(cc))));
	let result: eframe::Result = eframe::run_native(app_name, native_options, app_creator);
}

struct MyEguiApp {
	values: Vec<u32>,
}

impl Default for MyEguiApp {
	fn default() -> Self {
		Self {
			values: (1..(1 << 5)).collect(),
		}
	}
}

impl MyEguiApp {
	fn new(creation_context: &CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.
		Self::default()
	}
}

impl eframe::App for MyEguiApp {
	
	fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
		
		egui::CentralPanel::default().show(context, |ui: &mut egui::Ui| {
			
			ui.heading("Hello World!");
			
			let height = TextStyle::Body.resolve(ui.style()).size;
			
			ScrollArea::vertical().show_rows(ui, height, self.values.len(), |ui, row_range| {
				
				ui.allocate_space([ui.available_width(), 0.0].into());
				for i in row_range {
					let Some(value) = self.values.get(i) else {
						continue;
					};
					ui.label(format!("{value:08x}"));
				}
			})
			
		});
	}
}


fn stuff() {
	let test_purchase: Rc<RefCell<Purchase>> = domain::purchase::new("name".to_string(), None, Utc::now());
	
	let account = Account {
		name: "account name".to_string(),
		institution: None
	};
	
	let budget: Budget = budget::new("budget name".to_string(), None, vec![]);
	
	let charge_data = ChargeData {
		name: "charge name".to_string(),
		date_merchant_processed: Utc::now(),
		date_account_processed: Utc::now(),
		account: &account,
		amount: Decimal::new(1, 0),
	};
	
	let expense_data = ExpenseData {
		name: "".to_string(),
		budget: &budget,
		amount: Decimal::new(1, 0),
	};
	
	test_purchase.clone().add_charge(charge_data);
	test_purchase.clone().add_expense(expense_data);
	
	let test_transaction: Transaction = Transaction::Income(Income {
		name: "income name".to_string(),
		account: &account,
		date: Utc::now(),
		amount: Decimal::new(1, 0)
	});
	
	test_purchase.clone().add_associated_transaction(test_transaction)
}