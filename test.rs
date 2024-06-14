#![cfg(test)]

// use crate::Campaign;

// use super::testutils::{register_test_contract as register_contract, CrowdFund};

// use soroban_sdk::token::Client;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token, Address, Env,
};
extern crate std;

#[test]
fn test_intialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    // Address::generate(&env);

    let initialized = client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));
    // let re_initialized: Result<String, Error> = core::prelude::v1::Ok(client.initialize(&admin.clone(), &admin.clone(), &admin.clone(),  &admin.clone()));

    // assert_eq!(re_initialized, Err(Error::AlreadyInitialized));
}

#[test]
fn test_create_product() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    // Address::generate(&env);

    let initialized = client.initialize(
        &admin.clone(),
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
    );
    // std::println!("{:?}",initialized);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));

    std::println!("{}", env.ledger().timestamp());
    let title = String::from_str(&env, "Product 1");
    let description = String::from_str(&env, "Description 1");
    let category = String::from_str(&env, "Category 1");
    let expiry = env.ledger().timestamp() + 10000;
    let image = String::from_str(&env, "image.png");
    let price = 1000;
    let target = 10;

    // initialize(env.clone(), admin.clone(), admin.clone(), admin.clone(), admin.clone());
    let product = client.create_product(
        // env.clone(),
        &title.clone(),
        &description.clone(),
        &category.clone(),
        &expiry,
        &image.clone(),
        &price,
        &target,
    );

    assert_eq!(product.id, 1);
    assert_eq!(product.title, title);
    assert_eq!(product.description, description);
    assert_eq!(product.category, category);
    assert_eq!(product.expiry, expiry);
    assert_eq!(product.image, image);
    assert_eq!(product.price, price);
    assert_eq!(product.remaining, target);
}

#[test]
fn test_get_product() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    // Address::generate(&env);

    let initialized = client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));

    let title = String::from_str(&env, "Product 1");
    let description = String::from_str(&env, "Description 1");
    let category = String::from_str(&env, "Category 1");
    let expiry = env.ledger().timestamp() + 10000;
    let image = String::from_str(&env, "image.png");
    let price = 1000;
    let target = 10;

    // initialize(env.clone(), admin.clone(), admin.clone(), admin.clone(), admin.clone());
    client.create_product(
        // env.clone(),
        &title.clone(),
        &description.clone(),
        &category.clone(),
        &expiry,
        &image.clone(),
        &price,
        &target,
    );

    let product = client.get_product(&1);
    assert_eq!(product.id, 1);
    assert_eq!(product.title, title);
    assert_eq!(product.description, description);
    assert_eq!(product.category, category);
    assert_eq!(product.expiry, expiry);
    assert_eq!(product.image, image);
    assert_eq!(product.price, price);
    assert_eq!(product.remaining, target);
}

#[test]
fn test_get_products() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    // Address::generate(&env);

    let initialized = client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));

    let title = String::from_str(&env, "Product 1");
    let description = String::from_str(&env, "Description 1");
    let category = String::from_str(&env, "Category 1");
    let expiry = env.ledger().timestamp() + 10000;
    let image = String::from_str(&env, "image.png");
    let price = 1000;
    let target = 10;

    // initialize(env.clone(), admin.clone(), admin.clone(), admin.clone(), admin.clone());
    client.create_product(
        // env.clone(),
        &title.clone(),
        &description.clone(),
        &category.clone(),
        &expiry,
        &image.clone(),
        &price,
        &target,
    );

    let title2 = String::from_str(&env, "Product 2");
    let description2 = String::from_str(&env, "Description 2");
    let category2 = String::from_str(&env, "Category 2");
    let expiry2 = env.ledger().timestamp() + 20000;
    let image2 = String::from_str(&env, "image2.png");
    let price2 = 2000;
    let target2 = 20;

    client.create_product(
        // env.clone(),
        &title2.clone(),
        &description2.clone(),
        &category2.clone(),
        &expiry2,
        &image2.clone(),
        &price2,
        &target2,
    );

    let products = client.get_products();
    assert_eq!(products.len(), 2);
    assert_eq!(products.get(0).unwrap().title, title);
    assert_eq!(products.get(1).unwrap().title, title2);
}

#[test]
fn test_get_discount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    let token_id = Address::generate(&env);
    // Address::generate(&env);
    let customer = Address::generate(&env);
    let initialized = client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);
    let token = token::StellarAssetClient::new(
        &env,
        &env.register_stellar_asset_contract(token_id.clone()),
    );

    // Mint some ARTY tokens to work with
    token.mint(&customer.clone(), &10000000000);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));

    let title = String::from_str(&env, "Product 1");
    let description = String::from_str(&env, "Description 1");
    let category = String::from_str(&env, "Category 1");
    let expiry = env.ledger().timestamp() + 10000;
    let image = String::from_str(&env, "image.png");
    let price = 1000;
    let target = 10;
    let amount = 1;
    // initialize(env.clone(), admin.clone(), admin.clone(), admin.clone(), admin.clone());
    client.create_product(
        // env.clone(),
        &title.clone(),
        &description.clone(),
        &category.clone(),
        &expiry,
        &image.clone(),
        &price,
        &target,
    );

    let (reserve_amount, launchpad_amount, dev_amount) =
        client.get_discount(&1, &customer.clone(), &amount, &token.address.clone());

    let total_amount = amount * 10000000;
    let expected_reserve_amount =
        (total_amount * RESERVE_PER) / (RESERVE_PER + LAUNCHPAD_PER + DEV_PER);
    let expected_launchpad_amount =
        (total_amount * LAUNCHPAD_PER) / (RESERVE_PER + LAUNCHPAD_PER + DEV_PER);
    let expected_dev_amount = (total_amount * DEV_PER) / (RESERVE_PER + LAUNCHPAD_PER + DEV_PER);

    assert_eq!(reserve_amount, expected_reserve_amount);
    assert_eq!(launchpad_amount, expected_launchpad_amount);
    assert_eq!(dev_amount, expected_dev_amount);

    let product = client.get_product(&1);
    assert_eq!(product.remaining, target - 1);
}

#[test]
#[should_panic]
fn test_initialize_twice() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    // let token_id = Address::generate(&env);
    // Address::generate(&env);
    // let customer = Address::generate(&env);
    let initialized = client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));

    assert_eq!(
        Ok(client.initialize(
            &reserve_acc.clone(),
            &dev_acc.clone(),
            &launchpad_acc.clone(),
            &admin.clone(),
        )),
        Err(Error::AlreadyInitialized)
    );
}
#[test]
#[should_panic]
fn test_create_product_with_past_expiry() {
    let env = Env::default();

    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    // let token_id = Address::generate(&env);
    // Address::generate(&env);
    // let customer = Address::generate(&env);
    let initialized = client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);

    assert_eq!(initialized, String::from_str(&env, "Initialized"));
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
        max_entry_ttl: 1,
        min_persistent_entry_ttl: 2,
        min_temp_entry_ttl: 2,
    });
    std::println!("{}", env.ledger().timestamp());
    let title = String::from_str(&env, "Product 1");
    let description = String::from_str(&env, "Description 1");
    let category = String::from_str(&env, "Category 1");
    let expiry = env.ledger().timestamp() - 100;
    let image = String::from_str(&env, "image.png");
    let price = 1000;
    let target = 10;

    // initialize(env.clone(), admin.clone(), admin.clone(), admin.clone(), admin.clone());
    let result = client.create_product(
        // env.clone(),
        &title.clone(),
        &description.clone(),
        &category.clone(),
        &expiry,
        &image.clone(),
        &price,
        &target,
    );

    assert_eq!(Ok(result), Err(Error::ExpiryShouldBeFuture));
}

#[test]
#[should_panic]
fn test_get_discount_with_zero_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let reserve_acc = Address::generate(&env);
    let dev_acc = Address::generate(&env);
    let launchpad_acc = Address::generate(&env);
    let customer = Address::generate(&env);

    let title = String::from_str(&env, "Product 1");
    let description = String::from_str(&env, "Description 1");
    let category = String::from_str(&env, "Category 1");
    let expiry = env.ledger().timestamp() + 10000;
    let image = String::from_str(&env, "image.png");
    let price = 1000;
    let target = 10;
    let amount = 0;
    let token_id = Address::generate(&env);

    client.initialize(
        &reserve_acc.clone(),
        &dev_acc.clone(),
        &launchpad_acc.clone(),
        &admin.clone(),
    );
    // std::println!("{:?}",initialized);
    let token = token::StellarAssetClient::new(
        &env,
        &env.register_stellar_asset_contract(token_id.clone()),
    );

    // Mint some ARTY tokens to work with
    token.mint(&customer.clone(), &10000000000);
    client.create_product(
        // env.clone(),
        &title.clone(),
        &description.clone(),
        &category.clone(),
        &expiry,
        &image.clone(),
        &price,
        &target,
    );

    // let (reserve_amount, launchpad_amount, dev_amount) =

    assert_eq!(
        Ok(client.get_discount(&1, &customer.clone(), &amount, &token.address.clone())),
        Err(Error::AmountMustNonZero)
    );
}
