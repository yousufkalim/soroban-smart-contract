#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, symbol_short, token, Address, Env,
    String, Symbol, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    DiscountExpired = 1,
    ProductNotExist = 2,
    AmountMustNonZero = 3,
    TargetReached = 4,
    AmountExceedTargetLimit = 5,
    ProductAlreadyExist = 6,
    IdProductMustNonZero = 7,
    LowAmountForSplitter = 8,
    ExpiryShouldBeFuture = 9,
    AlreadyInitialized = 10,
    AmountMustBeGreaterThanZero = 11,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub category: String,
    pub expiry: u64,
    pub image: String,
    pub price: i128,
    pub remaining: i128,
}

const NO_OF_PRODUCTS: Symbol = symbol_short!("PRODUCTS");

const RESERVE_PER: i128 = 60;
const LAUNCHPAD_PER: i128 = 10;
const DEV_PER: i128 = 30;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    ReserveAccount,
    DevAccount,
    LaunchpadAccount,
    Admin,
}

#[contract]
pub struct Marketplace;

#[contractimpl]
impl Marketplace {
    pub fn initialize(
        env: Env,
        reserve_acc: Address,
        dev_acc: Address,
        launchpad_acc: Address,
        admin: Address,
    ) -> Result<String, Error> {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        env.storage()
            .instance()
            .set(&DataKey::ReserveAccount, &reserve_acc);
        env.storage().instance().set(&DataKey::DevAccount, &dev_acc);
        env.storage()
            .instance()
            .set(&DataKey::LaunchpadAccount, &launchpad_acc);
        env.storage().instance().set(&DataKey::Admin, &admin);

        env.events()
            .publish((symbol_short!("INIT"), symbol_short!("accounts")), admin);

        Ok(String::from_str(&env, "Initialized"))
    }

    pub fn create_product(
        env: Env,
        product_title: String,
        product_description: String,
        product_category: String,
        product_expiry: u64,
        product_image: String,
        product_price: i128,
        product_target: i128,
    ) -> Result<Product, Error> {
        if product_expiry < env.ledger().timestamp() {
            return Err(Error::ExpiryShouldBeFuture);
        }

        let mut count_id: u32 = env.storage().instance().get(&NO_OF_PRODUCTS).unwrap_or(0); // If no value set, assume 0.

        count_id += 1;

        let check_product = Self::get_product(env.clone(), count_id.clone());

        if check_product.id == count_id {
            return Err(Error::ProductAlreadyExist);
        }
        if count_id == 0 {
            return Err(Error::IdProductMustNonZero);
        }

        let product = Product {
            id: count_id,
            title: product_title,
            description: product_description,
            remaining: product_target,
            expiry: product_expiry,
            category: product_category,
            image: product_image,
            price: product_price,
        };

        env.storage().instance().set(&NO_OF_PRODUCTS, &count_id);
        env.storage().instance().set(&count_id, &product);

        env.events().publish(
            (symbol_short!("create"), symbol_short!("product")),
            count_id,
        );

        return Ok(product);
    }

    pub fn get_products(env: Env) -> Vec<Product> {
        let mut products = Vec::new(&env);

        let total_products: u32 = env.storage().instance().get(&NO_OF_PRODUCTS).unwrap_or(0);

        for product_id in 1..=total_products {
            let product = Self::get_product(env.clone(), product_id);

            products.push_back(product);
        }

        return products;
    }

    pub fn get_product(env: Env, product_id: u32) -> Product {
        let product: Product = env
            .storage()
            .instance()
            .get(&product_id)
            .unwrap_or(Product {
                id: 0,
                title: String::from_str(&env, ""),
                description: String::from_str(&env, ""),
                category: String::from_str(&env, ""),
                remaining: 0,
                image: String::from_str(&env, ""),
                price: 0,
                expiry: 0,
            });

        return product;
    }

    pub fn get_discount(
        env: Env,
        id: u32,
        customer_address: Address,
        amount: i128,
        token_id: Address,
    ) -> Result<(i128, i128, i128), Error> {
        customer_address.require_auth();

        if amount <= 0 {
            return Err(Error::AmountMustBeGreaterThanZero);
        }

        // log!(&env, "amount: {}", amount);
        let mut check_product = Self::get_product(env.clone(), id.clone());

        if check_product.id != id || id == 0 {
            return Err(Error::ProductNotExist);
        }
        if amount <= 0 {
            return Err(Error::AmountMustNonZero);
        }
        if check_product.remaining <= 0 {
            return Err(Error::TargetReached);
        }

        log!(&env, "amount: {}", amount);
        let total_percentage = RESERVE_PER + LAUNCHPAD_PER + DEV_PER;
        let real_amount = amount * 10000000;
        // Payment splitting and calculating the percentage
        let reserve_amount = (real_amount * RESERVE_PER) / total_percentage;
        let launchpad_amount = (real_amount * LAUNCHPAD_PER) / total_percentage;
        let dev_amount = (real_amount * DEV_PER) / total_percentage;

        if reserve_amount == 0 || launchpad_amount == 0 || dev_amount == 0 {
            return Err(Error::LowAmountForSplitter);
        }

        // get accounts
        let reserve_acc = Self::get_reserve_acc(env.clone());
        let dev_acc = Self::get_dev_acc(env.clone());
        let launchpad_acc = Self::get_launchpad_acc(env.clone());

        // transfer splitted Tokens to artist, dev, launchpad accounts
        let client: token::TokenClient = token::Client::new(&env.clone(), &token_id);
        log!(&env, "reserve_acc: ", reserve_acc);
        log!(&env, "reserve_amount: ", reserve_amount);
        client.transfer(&customer_address, &reserve_acc, &reserve_amount);
        client.transfer(&customer_address, &dev_acc, &dev_amount);
        client.transfer(&customer_address, &launchpad_acc, &launchpad_amount);
        log!(&env, "Haris: ", amount);
        // Save data
        check_product.remaining -= 1;
        env.storage()
            .instance()
            .set(&check_product.id, &check_product);

        env.events().publish(
            (symbol_short!("buy"), symbol_short!("discount")),
            check_product.id,
        );

        return Ok((reserve_amount, launchpad_amount, dev_amount));
    }

    pub fn get_reserve_acc(e: Env) -> Address {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::ReserveAccount)
            .expect("none")
    }

    pub fn get_dev_acc(e: Env) -> Address {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::DevAccount)
            .expect("none")
    }

    pub fn get_launchpad_acc(e: Env) -> Address {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::LaunchpadAccount)
            .expect("none")
    }

    pub fn get_admin(e: Env) -> Address {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .expect("none")
    }
}

#[cfg(test)]
mod test;
