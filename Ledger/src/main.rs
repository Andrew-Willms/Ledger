use std::vec;
use chrono::{DateTime, Utc};
use rust_decimal::{Decimal};
use vec::Vec;

fn main() {
    println!("Hello, world!");
}

struct Institution {
    name: String
}

struct Account<'a> {
    name: String,
    institution: Option<&'a Institution>,
}

struct Budget {
    name: String,
}

struct MerchantFamily {
    name: String,
}

struct Merchant<'a> {
    name: String,
    merchant_family: Option<&'a MerchantFamily>,
    address: Option<String>,
}

struct Charge<'a> {
    name: String,
    date_merchant_processed: DateTime<Utc>,
    date_account_processed: DateTime<Utc>,
    account: &'a Account<'a>,
    amount: Decimal,
}

struct Expense<'a> {
    name: String,
    budget: Budget,
    purchase: Purchase<'a>,
    amount: Decimal,
}

struct Purchase<'a> {
    name: String,
    merchant: Option<&'a Merchant<'a>>,
    date: DateTime<Utc>,
    charges: Vec<Charge<'a>>,
    expenses: Vec<Expense<'a>>
}

struct Item<'a> {
    name: String,
    notes: String,
    purchase: Purchase<'a>,

    date_order_processed: Option<DateTime<Utc>>,
    date_shipped: Option<DateTime<Utc>>,
    date_received: Option<DateTime<Utc>>,

    regular_price: Decimal,
    sale_price: Decimal,
    effective_price: Decimal,
}

struct Transfer<'a> {
    name: String,
    date_initiated: DateTime<Utc>,
    date_sent: DateTime<Utc>,
    date_received: DateTime<Utc>,
    source_account: &'a Account<'a>,
    destination_account: &'a Account<'a>,
    amount: Decimal
}

struct TransferBudget {
    name: String,
    date: DateTime<Utc>,
    source_budget: Budget,
    destination_budget: Budget,
    amount: Decimal,
}

struct Income<'a> {
    name: String,
    account: &'a Account<'a>,
    date: DateTime<Utc>,
    amount: Decimal
}