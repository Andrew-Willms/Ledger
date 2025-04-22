use chrono::{DateTime, Utc};
use rust_decimal::{Decimal};

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

struct Purchase<'a> {
    name: String,
    merchant: Option<&'a Merchant<'a>>,
    date: DateTime<Utc>,
}


struct MerchantFamily {
    name: String,
}

struct Merchant<'a> {
    name: String,
    merchant_family: Option<&'a MerchantFamily>,
    address: Option<String>,
}

struct Expense<'a> {
    name: String,
    merchant: Option<&'a Merchant<'a>>,
    date_initiated: DateTime<Utc>,
    date_posted: DateTime<Utc>,
    account: &'a Account<'a>
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

struct Income<'a> {
    name: String,
    account: &'a Account<'a>,
    date:DateTime<Utc>,
    amount: Decimal
}