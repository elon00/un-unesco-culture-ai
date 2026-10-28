use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 3,
        agency_name: "UN Educational, Scientific and Cultural Organization (UNESCO)".to_string(),
        project_name: "UNESCO HeritageAI".to_string(),
        portfolio: "World Heritage & Scientific Ethics".to_string(),
        strategy: "Soulbound Verifiable Credential -- Global Academic Equivalency and Endangered Cultural Asset Registry".to_string(),
        target_chain: "Internet Computer (ICP) + Polygon".to_string(),
        is_active: true,
    }
}
