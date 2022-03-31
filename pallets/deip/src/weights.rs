//! Autogenerated weights for pallet_deip
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-01-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// appchain-deip
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_deip
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages
// 4096
// --output
// weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn create_project(d: u32) -> Weight;
    fn create_investment_opportunity(s: u32) -> Weight;
    fn activate_crowdfunding() -> Weight;
    fn expire_crowdfunding_already_expired() -> Weight;
    fn expire_crowdfunding() -> Weight;
    fn finish_crowdfunding() -> Weight;
    fn invest() -> Weight;
    fn invest_hard_cap_reached() -> Weight;
    fn update_project() -> Weight;
    fn create_project_content(a: u32, r: u32) -> Weight;
    fn create_project_nda(p: u32) -> Weight;
    fn create_nda_content_access_request() -> Weight;
    fn fulfill_nda_content_access_request() -> Weight;
    fn reject_nda_content_access_request() -> Weight;
    fn create_review(d: u32) -> Weight;
    fn upvote_review() -> Weight;
    fn add_domain() -> Weight;
    fn create_contract_agreement_project_license() -> Weight;
    fn create_contract_agreement_general_contract() -> Weight;
    fn accept_contract_agreement_project_license_unsigned() -> Weight;
    fn accept_contract_agreement_project_license_signed_by_licenser() -> Weight;
    fn accept_contract_agreement_general_contract_partially_accepted() -> Weight;
    fn accept_contract_agreement_general_contract_finalized() -> Weight;
}

/// Weight functions for pallet_deip.
pub struct Weights<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for Weights<T> {
    // Storage: Deip Domains (r:1 w:0)
    // Storage: Deip ProjectMapV1 (r:1 w:1)
    // Storage: Deip ProjectIdByTeamIdV1 (r:0 w:1)
    fn create_project(d: u32) -> Weight {
        (36_645_000 as Weight)
            // Standard Error: 29_000
            .saturating_add((6_632_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    // Storage: Assets InvestmentMap (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Assets AssetIdByDeipAssetId (r:2 w:0)
    // Storage: ParityTechAssets Asset (r:1 w:1)
    // Storage: ParityTechAssets Account (r:2 w:2)
    // Storage: Assets InvestmentByAssetId (r:2 w:2)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn create_investment_opportunity(s: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 43_194_000
            .saturating_add((362_308_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().reads((6 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
            .saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn activate_crowdfunding() -> Weight {
        (45_068_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    fn expire_crowdfunding_already_expired() -> Weight {
        (19_449_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip InvestmentMap (r:1 w:0)
    // Storage: Assets InvestmentMap (r:1 w:1)
    // Storage: Assets InvestmentByAssetId (r:11 w:11)
    // Storage: Assets AssetIdByDeipAssetId (r:22 w:0)
    // Storage: ParityTechAssets Account (r:21 w:20)
    // Storage: ParityTechAssets Asset (r:10 w:10)
    // Storage: System Account (r:1 w:1)
    // Storage: Assets ProjectIdByAssetId (r:10 w:0)
    fn expire_crowdfunding() -> Weight {
        (1_271_753_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(79 as Weight))
            .saturating_add(T::DbWeight::get().writes(44 as Weight))
    }
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    // Storage: Deip InvestmentMap (r:1 w:1)
    // Storage: Assets InvestmentMap (r:1 w:1)
    // Storage: Assets AssetIdByDeipAssetId (r:22 w:0)
    // Storage: ParityTechAssets Asset (r:11 w:11)
    // Storage: ParityTechAssets Account (r:22 w:22)
    // Storage: System Account (r:1 w:1)
    // Storage: Assets ProjectIdByAssetId (r:11 w:0)
    // Storage: Assets InvestmentByAssetId (r:11 w:11)
    fn finish_crowdfunding() -> Weight {
        (1_429_724_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(81 as Weight))
            .saturating_add(T::DbWeight::get().writes(48 as Weight))
    }
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    // Storage: Assets InvestmentMap (r:1 w:0)
    // Storage: Assets AssetIdByDeipAssetId (r:2 w:0)
    // Storage: ParityTechAssets Asset (r:1 w:1)
    // Storage: ParityTechAssets Account (r:2 w:2)
    // Storage: System Account (r:1 w:1)
    // Storage: Deip InvestmentMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn invest() -> Weight {
        (164_877_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: Deip SimpleCrowdfundingMap (r:1 w:1)
    // Storage: Assets InvestmentMap (r:1 w:1)
    // Storage: Assets AssetIdByDeipAssetId (r:22 w:0)
    // Storage: ParityTechAssets Asset (r:11 w:11)
    // Storage: ParityTechAssets Account (r:22 w:22)
    // Storage: System Account (r:1 w:1)
    // Storage: Deip InvestmentMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Assets ProjectIdByAssetId (r:11 w:0)
    // Storage: Assets InvestmentByAssetId (r:11 w:11)
    fn invest_hard_cap_reached() -> Weight {
        (1_528_920_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(82 as Weight))
            .saturating_add(T::DbWeight::get().writes(48 as Weight))
    }
    // Storage: Deip ProjectMapV1 (r:1 w:1)
    fn update_project() -> Weight {
        (28_574_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip ProjectContentMapV1 (r:51 w:1)
    // Storage: Deip ProjectMapV1 (r:1 w:0)
    // Storage: Deip ContentIdByProjectIdV1 (r:1 w:1)
    fn create_project_content(a: u32, r: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 652_000
            .saturating_add((798_000 as Weight).saturating_mul(a as Weight))
            // Standard Error: 652_000
            .saturating_add((9_357_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip Ndas (r:1 w:1)
    // Storage: Deip NdaMapV1 (r:0 w:1)
    // Storage: Deip ProjectMapV1 (r:1 w:0)
    fn create_project_nda(p: u32) -> Weight {
        (71_768_000 as Weight)
            // Standard Error: 60_000
            .saturating_add((8_374_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip NdaMapV1 (r:1 w:0)
    // Storage: Deip NdaAccessRequests (r:1 w:1)
    // Storage: Deip NdaAccessRequestMapV1 (r:0 w:1)
    fn create_nda_content_access_request() -> Weight {
        (60_406_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Deip NdaAccessRequestMapV1 (r:1 w:1)
    // Storage: Deip NdaMapV1 (r:1 w:0)
    fn fulfill_nda_content_access_request() -> Weight {
        (51_652_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip NdaAccessRequestMapV1 (r:1 w:1)
    // Storage: Deip NdaMapV1 (r:1 w:0)
    fn reject_nda_content_access_request() -> Weight {
        (50_083_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip Domains (r:1 w:0)
    // Storage: Deip ReviewMapV1 (r:1 w:1)
    // Storage: Deip ProjectContentMapV1 (r:1 w:0)
    // Storage: Deip ReviewIdByAccountIdV1 (r:0 w:1)
    // Storage: Deip ReviewIdByProjectIdV1 (r:0 w:1)
    // Storage: Deip ReviewIdByContentIdV1 (r:0 w:1)
    fn create_review(d: u32) -> Weight {
        (44_425_000 as Weight)
            // Standard Error: 22_000
            .saturating_add((6_612_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Deip Domains (r:1 w:0)
    // Storage: Deip ReviewMapV1 (r:1 w:0)
    // Storage: Deip ReviewVoteMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip VoteIdByReviewIdV1 (r:0 w:1)
    // Storage: Deip VoteIdByAccountId (r:0 w:1)
    fn upvote_review() -> Weight {
        (65_252_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Deip DomainCount (r:1 w:1)
    // Storage: Deip Domains (r:1 w:1)
    fn add_domain() -> Weight {
        (46_126_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Deip ProjectMapV1 (r:1 w:0)
    // Storage: Deip ContractAgreementIdByType (r:0 w:1)
    fn create_contract_agreement_project_license() -> Weight {
        (49_030_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Deip ContractAgreementIdByType (r:0 w:1)
    fn create_contract_agreement_general_contract() -> Weight {
        (42_595_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn accept_contract_agreement_project_license_unsigned() -> Weight {
        (40_842_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Assets AssetIdByDeipAssetId (r:4 w:0)
    // Storage: ParityTechAssets Account (r:2 w:2)
    // Storage: Assets AssetIdByProjectId (r:1 w:0)
    // Storage: ParityTechAssets Asset (r:2 w:1)
    // Storage: Assets FtBalanceMap (r:1 w:0)
    // Storage: Deip ProjectMapV1 (r:1 w:0)
    // Storage: Assets ProjectIdByAssetId (r:1 w:0)
    fn accept_contract_agreement_project_license_signed_by_licenser() -> Weight {
        (197_721_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(14 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    fn accept_contract_agreement_general_contract_partially_accepted() -> Weight {
        (36_490_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Deip ContractAgreementMap (r:1 w:1)
    fn accept_contract_agreement_general_contract_finalized() -> Weight {
        (47_737_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
