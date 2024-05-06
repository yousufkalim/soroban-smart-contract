#![cfg(test)]

use crate::Campaign;

use super::testutils::{register_test_contract as register_contract, CrowdFund};

use soroban_sdk::token::Client;

use super::CrowdFundClient;
use soroban_sdk::{testutils::Address as _, token::Client as Token, Address, Env, String, Vec};
mod token_arty {
    soroban_sdk::contractimport!(file = "token/soroban_token_contract.wasm");
}

fn create_contract() -> (
    CrowdFundClient<'static>,
    Env,
    token_arty::Client<'static>,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let id = register_contract(&env);
    let crowdfund = CrowdFund::new(&env, id.clone());

    // ARTY token creation
    let token_admin = Address::random(&env);
    let contract_arty = env.register_stellar_asset_contract(token_admin.clone());
    let arty_token = token_arty::Client::new(&env, &contract_arty);

    // Mint some ARTY tokens to work with
    arty_token.mint(&token_admin, &50000);
    // arty_token

    // Accounts creation and initialization
    let dev_acc = Address::random(&env);
    let launchpad_acc = Address::random(&env);

    let client = crowdfund.client();

    // initialize the accounts, ARTY token and Admin Account
    client.initialize(&dev_acc, &launchpad_acc, &contract_arty, &token_admin);

    (client, env.clone(), arty_token, token_admin)
}

struct Setup<'a> {
    env: Env,
    client: CrowdFundClient<'static>,
    contract_token: Address,
    token: Token<'a>,
    arty_token: Client<'static>,
    artist1: Address,
    artist2: Address,
    donor1: Address,
    donor2: Address,
    title: String,
    description: String,
    target: i128,
    target1: i128,
    deadline: u64,
    image: String,
    campaign1: Campaign,
    campaign2: Campaign,
    category: String,
}

impl Setup<'_> {
    fn new() -> Self {
        let contract_client = create_contract();
        let client = contract_client.0;
        let env = contract_client.1;
        let arty_token = contract_client.2;
        let token_admin = contract_client.3;

        // Create the token contract
        let contract_token = env.register_stellar_asset_contract(token_admin);
        let token = Token::new(&env, &contract_token);

        // Artists Addresses
        let artist1 = Address::random(&env);
        let artist2 = Address::random(&env);

        // Donors Addresses
        let donor1 = Address::random(&env);
        let donor2 = Address::random(&env);

        // Mint some tokens to work with
        token.mint(&donor1, &1000);
        token.mint(&donor2, &500);

        // Campaign Details
        let title = String::from_slice(&env, "New Title");
        let description = String::from_slice(&env, "New Desc");
        let category = String::from_slice(&env, "Music"); // New category
        let target: i128 = 2000;
        let target1 = 300;
        let deadline = 1692574408;
        let image = String::from_slice(&env, "https://avatars.githubusercontent.com/u/43227117?s=400&u=f9fa09edf5ea7a342c28f54fd0da5a7ac53b3eed&v=4");

        // create campaigns
        let campaign1 = client.create_campaign(
            &artist1,
            &title,
            &description,
            &image,
            &target,
            &deadline,
            &category,
        );
        let campaign2 = client.create_campaign(
            &artist2,
            &title,
            &description,
            &image,
            &target1,
            &deadline,
            &category,
        );

        Self {
            env: env,
            client: client,
            contract_token,
            token,
            arty_token,
            artist1,
            artist2,
            donor1,
            donor2,
            title,
            description,
            target,
            target1,
            deadline,
            image,
            campaign1,
            campaign2,
            category,
        }
    }
}

#[test]
fn test_all_created_campaigns() {
    let setup = Setup::new();

    let exp_campaign1 = Campaign {
        id: 1,
        owner: setup.artist1.clone(),
        title: setup.title.clone(),
        description: setup.description.clone(),
        target: setup.target,
        deadline: setup.deadline,
        amount_collected: 0,
        status: true,
        image: setup.image.clone(),
        category: setup.category.clone(), // New category field
        donators: Vec::new(&setup.env.clone()),
        donations: Vec::new(&setup.env.clone()),
    };

    let exp_campaign2 = Campaign {
        id: 2,
        owner: setup.artist2.clone(),
        title: setup.title,
        description: setup.description,
        target: setup.target1,
        deadline: setup.deadline,
        amount_collected: 0,
        status: true,
        category: setup.category.clone(), // New category field
        image: setup.image,
        donators: Vec::new(&setup.env.clone()),
        donations: Vec::new(&setup.env.clone()),
    };

    assert_eq!(setup.campaign1, exp_campaign1);
    assert_eq!(setup.campaign2, exp_campaign2);

    // Get all campaigns before donation
    let all_campaings = setup.client.get_campaigns();

    let mut all_camp_exp = Vec::new(&setup.env.clone()); // Initialize with a new address
    all_camp_exp.push_back(exp_campaign1.clone());
    all_camp_exp.push_back(exp_campaign2.clone());

    assert_eq!(all_campaings, all_camp_exp);
}

#[test]
fn test_donate_to_campaigns_and_payment_splitter() {
    let setup = Setup::new();

    let amount1: i128 = 100;
    let splitted_amount1 =
        setup
            .client
            .donate_to_campaign(&1, &setup.donor1, &amount1, &setup.contract_token.clone());

    assert_eq!(splitted_amount1, (95, 2, 3));
    assert_eq!(setup.arty_token.balance(&setup.donor1), 100);

    let amount2: i128 = 150;
    let splitted_amount2 =
        setup
            .client
            .donate_to_campaign(&2, &setup.donor2, &amount2, &setup.contract_token.clone());

    assert_eq!(splitted_amount2, (142, 3, 4));
    assert_eq!(setup.arty_token.balance(&setup.donor2), 150);

    let amount3: i128 = 149;
    let splitted_amount3 =
        setup
            .client
            .donate_to_campaign(&2, &setup.donor1, &amount3, &setup.contract_token.clone());

    assert_eq!(splitted_amount3, (141, 2, 4));
    assert_eq!(setup.arty_token.balance(&setup.donor1), 249);

    // Get Donators
    let donators = setup.client.get_donators(&2);

    let mut exp_donators = Vec::new(&setup.env);
    exp_donators.push_front(setup.donor2.clone());
    exp_donators.push_front(setup.donor1.clone());

    assert_eq!(donators, exp_donators);

    // Balance Test
    let dev_acc = setup.client.get_dev_acc();
    let launchpad_acc = setup.client.get_launchpad_acc();

    assert_eq!(setup.token.balance(&dev_acc), 11);
    assert_eq!(setup.token.balance(&launchpad_acc), 7);
    assert_eq!(setup.token.balance(&setup.artist1), 95);
    assert_eq!(setup.token.balance(&setup.artist2), 283);

    assert_eq!(setup.arty_token.balance(&setup.donor2), 150);
}
