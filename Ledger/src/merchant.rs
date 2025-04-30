pub struct MerchantFamily {
	name: String
}

pub struct Merchant<'a> {
	name: String,
	merchant_family: Option<&'a MerchantFamily>,
	address: Option<String>
}