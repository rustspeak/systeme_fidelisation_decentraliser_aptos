use aptos_sdk::types::account_address::AccountAddress;
use aptos_sdk::rest_client::{Client, FaucetClient};
use aptos_sdk::crypto::{ed25519::Ed25519PrivateKey, PrivateKey};
use crate::model::{AccountResponse, CreateProposalRequest, ProposalResponse, SubmitVoteRequest, VoteResponse, GetRewardsRequest, RewardsResponse};
use std::error::Error;

pub async fn create_account() -> Result<AccountResponse, Box<dyn Error>> {
    let client = Client::new("https://fullnode.devnet.aptoslabs.com");
    let faucet_client = FaucetClient::new("https://faucet.devnet.aptoslabs.com", client.clone());

    let private_key = Ed25519PrivateKey::generate_for_testing();
    let public_key = private_key.public_key();
    let account_address = AccountAddress::from_public_key(&public_key);

    faucet_client.create_account(account_address).await.unwrap();

    let response = AccountResponse {
        address: format!("{:?}", account_address),
        private_key: format!("{:?}", private_key),
    };

    Ok(response)
}

pub async fn create_proposal(req: CreateProposalRequest) -> Result<ProposalResponse, Box<dyn Error>> {
    // Simuler la création de proposition
    let proposal_id = 1;
    Ok(ProposalResponse { proposal_id })
}

pub async fn submit_vote(req: SubmitVoteRequest) -> Result<VoteResponse, Box<dyn Error>> {
    // Simuler la soumission de vote
    // Ajouter une récompense à l'utilisateur pour sa participation
    Ok(VoteResponse { status: "success".to_string() })
}

pub async fn get_rewards(query: GetRewardsRequest) -> Result<RewardsResponse, Box<dyn Error>> {
    // Simuler la récupération des récompenses
    let total_rewards = 100; // Valeur simulée
    Ok(RewardsResponse { total_rewards })
}

