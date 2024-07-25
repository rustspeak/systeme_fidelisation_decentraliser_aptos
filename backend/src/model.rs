use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AccountResponse {
    pub address: String,
    pub private_key: String,
}

#[derive(Deserialize)]
pub struct CreateProposalRequest {
    pub description: String,
}

#[derive(Serialize)]
pub struct ProposalResponse {
    pub proposal_id: u64,
}

#[derive(Deserialize)]
pub struct SubmitVoteRequest {
    pub proposal_id: u64,
    pub vote_for: bool,
}

#[derive(Serialize)]
pub struct VoteResponse {
    pub status: String,
}

#[derive(Deserialize)]
pub struct GetRewardsRequest {
    pub account_address: String,
}

#[derive(Serialize)]
pub struct RewardsResponse {
    pub total_rewards: u64,
}
