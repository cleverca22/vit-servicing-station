use std::collections::HashMap;
use vit_servicing_station_lib::db::models::{
    api_tokens::APITokenData, funds::Fund, proposals::Proposal, voteplans::Voteplan,
};

#[derive(Debug, Clone)]
pub struct Snapshot {
    funds: Vec<Fund>,
    proposals: Vec<Proposal>,
    tokens: HashMap<String, APITokenData>,
    voteplans: Vec<Voteplan>,
}

impl Snapshot {
    pub fn new(
        funds: Vec<Fund>,
        proposals: Vec<Proposal>,
        tokens: HashMap<String, APITokenData>,
        voteplans: Vec<Voteplan>,
    ) -> Self {
        Self {
            funds,
            proposals,
            tokens,
            voteplans,
        }
    }

    pub fn funds(&self) -> Vec<Fund> {
        self.funds.clone()
    }

    pub fn proposals(&self) -> Vec<Proposal> {
        self.proposals.clone()
    }

    pub fn tokens(&self) -> HashMap<String, APITokenData> {
        self.tokens.clone()
    }

    pub fn voteplans(&self) -> Vec<Voteplan> {
        self.voteplans.clone()
    }

    pub fn funds_mut(&mut self) -> &mut Vec<Fund> {
        &mut self.funds
    }

    pub fn proposals_mut(&mut self) -> &mut Vec<Proposal> {
        &mut self.proposals
    }

    pub fn voteplans_mut(&mut self) -> &mut Vec<Voteplan> {
        &mut self.voteplans
    }

    pub fn proposal_by_id(&self, id: &str) -> Option<&Proposal> {
        self.proposals.iter().find(|x| x.proposal_id.eq(id))
    }

    pub fn fund_by_id(&self, id: i32) -> Option<&Fund> {
        self.funds.iter().find(|x| x.id == id)
    }

    pub fn any_token(&self) -> (String, APITokenData) {
        let (hash, token) = self.tokens.iter().next().clone().unwrap();
        (hash.to_string(), token.clone())
    }

    pub fn token_hash(&self) -> String {
        self.any_token().0
    }
}
