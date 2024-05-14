#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env, String,
    Symbol, Vec, log,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    DeadlineShouldBeFuture = 1,
    CampaignNotExist = 2,
    AmountMustNonZero = 3,
    TargetReached = 4,
    AmountExceedTargetLimit = 5,
    CampaignAlreadyExist = 6,
    IdCampaignMustNonZero = 7,
    LowAmountForSplitter = 8,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Campaign {
    pub id: u32,
    pub owner: Address,
    pub title: String,
    pub description: String,
    // pub short_description: String,
    pub category: String,
    pub main_location: String,
    // pub temp_location: String,
    // pub tag: String,
    pub metadata: String,
    pub target: i128,
    pub deadline: u64,
    pub amount_collected: i128,
    pub status: bool,
    pub image: String,
    pub donators: Vec<Address>,
    pub donations: Vec<i128>,
}

const NO_OF_CAMPAIGNS: Symbol = symbol_short!("CAMPAIGNS");

const ARTIST_PER: i128 = 95;
const LAUNCHPAD_PER: i128 = 2;
const DEV_PER: i128 = 3;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    DevAccount,
    LaunchpadAccount,
    ArtyToken,
    TokenAdmin,
}

#[contract]
pub struct CrowdFund;

#[contractimpl]
impl CrowdFund {
    pub fn initialize(
        env: Env,
        dev_acc: Address,
        launchpad_acc: Address,
        arty_token: Address,
        token_admin: Address,
    ) {
        token_admin.require_auth();
        assert!(
            !env.storage().instance().has(&DataKey::TokenAdmin),
            "already initialized"
        );

        env.storage().instance().set(&DataKey::DevAccount, &dev_acc);
        env.storage()
            .instance()
            .set(&DataKey::LaunchpadAccount, &launchpad_acc);
        env.storage()
            .instance()
            .set(&DataKey::ArtyToken, &arty_token);
        env.storage()
            .instance()
            .set(&DataKey::TokenAdmin, &token_admin);

        env.events().publish(
            (symbol_short!("INIT"), symbol_short!("accounts")),
            token_admin,
        );
    }

    pub fn create_campaign(
        env: Env,
        owner_addr: Address,
        title_cmp: String,
        desc_cmp: String,
        // short_desc_cmp: String,
        category_cmp: String,
        main_location_cmp: String,
        // temp_location_cmp: String,
        // tag_cmp: String,
        metadata_cmp: String,
        image_cmp: String,
        target_cmp: i128,
        deadline_cmp: u64,
    ) -> Result<Campaign, Error> {
        owner_addr.require_auth();

        if deadline_cmp < env.ledger().timestamp() {
            return Err(Error::DeadlineShouldBeFuture);
        }

        let mut count_id: u32 = env.storage().instance().get(&NO_OF_CAMPAIGNS).unwrap_or(0); // If no value set, assume 0.

        count_id += 1;

        let check_campaign = Self::get_campaign(env.clone(), count_id.clone());

        if check_campaign.id == count_id {
            return Err(Error::CampaignAlreadyExist);
        }
        if count_id <= 0 {
            return Err(Error::IdCampaignMustNonZero);
        }

        let campaign = Campaign {
            id: count_id,
            owner: owner_addr,
            title: title_cmp,
            description: desc_cmp,
            target: target_cmp,
            deadline: deadline_cmp,
            category: category_cmp,
            main_location: main_location_cmp,
            // temp_location: temp_location_cmp,
            // tag: tag_cmp,
            metadata: metadata_cmp,
            amount_collected: 0,
            status: true,
            image: image_cmp,
            donators: Vec::new(&env),
            donations: Vec::new(&env),
        };

        env.storage().instance().set(&NO_OF_CAMPAIGNS, &count_id);
        env.storage().instance().set(&count_id, &campaign);

        env.events().publish(
            (symbol_short!("create"), symbol_short!("campaign")),
            count_id,
        );

        return Ok(campaign);
    }

    pub fn get_campaigns(env: Env) -> Vec<Campaign> {
        let mut campaigns = Vec::new(&env);

        let total_campaigns: u32 = env.storage().instance().get(&NO_OF_CAMPAIGNS).unwrap_or(0);

        for campaign_id in 1..=total_campaigns {
            let campaign = Self::get_campaign(env.clone(), campaign_id);

            campaigns.push_back(campaign);
        }

        campaigns
    }

    pub fn get_campaign(env: Env, campaign_id: u32) -> Campaign {
        let campaign: Campaign = env
            .storage()
            .instance()
            .get(&campaign_id)
            .unwrap_or(Campaign {
                id: 0,
                owner: env.current_contract_address(),
                title: String::from_str(&env, ""),
                description: String::from_str(&env, ""),
                category: String::from_str(&env, ""),
                main_location: String::from_str(&env, ""),
                metadata: String::from_str(&env, ""),
                target: 0,
                deadline: 0,
                amount_collected: 20,
                status: false,
                image: String::from_str(&env, ""),
                donators: Vec::new(&env),
                donations: Vec::new(&env),
            });

        return campaign;
    }

    pub fn donate_to_campaign(
        env: Env,
        id: u32,
        donor_address: Address,
        amount: i128,
        token_id: Address,
    ) -> Result<(i128, i128, i128), Error> {
        donor_address.require_auth();

        assert!(amount > 0, "amount must be positive");

        let mut check_campaign = Self::get_campaign(env.clone(), id.clone());

        if check_campaign.id != id || id == 0 {
            return Err(Error::CampaignNotExist);
        }
        if amount <= 0 {
            return Err(Error::AmountMustNonZero);
        }
        if check_campaign.amount_collected == check_campaign.target {
            return Err(Error::TargetReached);
        }

        log!(&env, "amount: {}", amount);
        let total_percentage = ARTIST_PER + LAUNCHPAD_PER + DEV_PER;
        let real_amount = amount * 10000000;
        // Payment splitting and calculating the percentage
        let artist_amount = (real_amount * ARTIST_PER) / total_percentage;
        let launchpad_amount = (real_amount * LAUNCHPAD_PER) / total_percentage;
        let dev_amount = (real_amount * DEV_PER) / total_percentage;

        if artist_amount == 0 || launchpad_amount == 0 || dev_amount == 0 {
            return Err(Error::LowAmountForSplitter);
        }

        let collected = check_campaign.amount_collected.clone();
        let total = collected + amount.clone();

        if total > check_campaign.target {
            return Err(Error::AmountExceedTargetLimit);
        }

        check_campaign.donators.push_front(donor_address.clone());
        check_campaign.donations.push_front(amount);

        // get accounts
        let dev_acc = Self::get_dev_acc(env.clone());
        let launchpad_acc = Self::get_launchpad_acc(env.clone());

        // transfer splitted Tokens to artist, dev, launchpad accounts
        let client: token::TokenClient = token::Client::new(&env.clone(), &token_id);

        client.transfer(&donor_address, &check_campaign.owner, &artist_amount);
        client.transfer(&donor_address, &dev_acc, &dev_amount);
        client.transfer(&donor_address, &launchpad_acc, &launchpad_amount);

        // Reward ARTY tokens to Donor account from Admin account
        let arty_token = Self::get_arty_token(env.clone());

        let client_admin = token::Client::new(&env.clone(), &arty_token);

        let reward_amount = amount * 100000000;

        client_admin.transfer(
            &env.current_contract_address(),
            &donor_address,
            &reward_amount,
        );

        // Save data
        check_campaign.amount_collected = total;
        env.storage()
            .instance()
            .set(&check_campaign.id, &check_campaign);

        env.events().publish(
            (symbol_short!("donate"), symbol_short!("campaign")),
            check_campaign.id,
        );

        return Ok((artist_amount, launchpad_amount, dev_amount));
    }

    pub fn get_donators(env: Env, id: u32) -> Result<Vec<Address>, Error> {
        let campaign = Self::get_campaign(env.clone(), id.clone());

        if campaign.id != id || id == 0 {
            return Err(Error::CampaignNotExist);
        }

        Ok(campaign.donators)
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

    pub fn get_arty_token(e: Env) -> Address {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::ArtyToken)
            .expect("none")
    }

    pub fn get_token_admin(e: Env) -> Address {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::TokenAdmin)
            .expect("none")
    }
}

#[cfg(test)]
mod test;

mod testutils;
