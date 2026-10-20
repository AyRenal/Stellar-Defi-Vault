//! Governance storage accessors.
//!
//! Handles proposals, voting weights, delegates, veto thresholds, and pending actions.

use crate::storage::{AdminProposal, GovernanceProposal, PendingAction, VoteWeightMode};
use soroban_sdk::{symbol_short, Address, Env, Vec};

// ── Vote weight mode ──────────────────────────────────────────────────────────

/// Pool-wide vote weight mode (snapshotted per proposal at creation time).
pub fn get_vote_weight_mode(env: &Env) -> VoteWeightMode {
    env.storage()
        .instance()
        .get(&symbol_short!("vw_mode"))
        .unwrap_or(VoteWeightMode::Linear)
}

pub fn set_vote_weight_mode(env: &Env, mode: VoteWeightMode) {
    env.storage().instance().set(&symbol_short!("vw_mode"), &mode);
}

// ── Proposal Vec store ────────────────────────────────────────────────────────

pub fn get_admin_proposals(env: &Env) -> Vec<AdminProposal> {
    env.storage()
        .instance()
        .get(&symbol_short!("adm_prop"))
        .unwrap_or(Vec::new(env))
}

pub fn set_admin_proposals(env: &Env, proposals: &Vec<AdminProposal>) {
    env.storage()
        .instance()
        .set(&symbol_short!("adm_prop"), proposals);
}

pub fn get_governance_proposals(env: &Env) -> Vec<GovernanceProposal> {
    env.storage()
        .instance()
        .get(&symbol_short!("gov_prop"))
        .unwrap_or(Vec::new(env))
}

pub fn set_governance_proposals(env: &Env, proposals: &Vec<GovernanceProposal>) {
    env.storage()
        .instance()
        .set(&symbol_short!("gov_prop"), proposals);
}

pub fn get_veto_threshold_bps(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("veto_bps"))
        .unwrap_or(0)
}

pub fn set_veto_threshold_bps(env: &Env, bps: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("veto_bps"), &bps);
}

pub fn get_proposal_vetoer(env: &Env, proposal_id: u32) -> Option<Address> {
    let key = (symbol_short!("prop_vtr"), proposal_id);
    env.storage().instance().get(&key)
}

pub fn set_proposal_vetoer(env: &Env, proposal_id: u32, vetoer: &Address) {
    let key = (symbol_short!("prop_vtr"), proposal_id);
    env.storage().instance().set(&key, vetoer);
}

pub fn get_vote_delegate(env: &Env, user: &Address) -> Option<Address> {
    let key = (symbol_short!("v_del"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_vote_delegate(env: &Env, user: &Address, delegate: &Address) {
    let key = (symbol_short!("v_del"), user.clone());
    env.storage().persistent().set(&key, delegate);
}

pub fn remove_vote_delegate(env: &Env, user: &Address) {
    let key = (symbol_short!("v_del"), user.clone());
    env.storage().persistent().remove(&key);
}

pub fn get_delegated_weight_snapshot(env: &Env, user: &Address) -> i128 {
    let key = (symbol_short!("dw_snap"), user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_delegated_weight_snapshot(env: &Env, user: &Address, weight: i128) {
    let key = (symbol_short!("dw_snap"), user.clone());
    env.storage().persistent().set(&key, &weight);
}

pub fn remove_delegated_weight_snapshot(env: &Env, user: &Address) {
    let key = (symbol_short!("dw_snap"), user.clone());
    env.storage().persistent().remove(&key);
}

pub fn get_delegated_vote_weight(env: &Env, delegate: &Address) -> i128 {
    let key = (symbol_short!("d_v_wt"), delegate.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_delegated_vote_weight(env: &Env, delegate: &Address, weight: i128) {
    let key = (symbol_short!("d_v_wt"), delegate.clone());
    env.storage().persistent().set(&key, &weight);
}

pub fn get_pending_actions(env: &Env) -> Vec<PendingAction> {
    env.storage()
        .instance()
        .get(&symbol_short!("p_action"))
        .unwrap_or(Vec::new(env))
}

pub fn set_pending_actions(env: &Env, actions: &Vec<PendingAction>) {
    env.storage()
        .instance()
        .set(&symbol_short!("p_action"), actions);
}

pub fn get_contract_delegate(
    env: &Env,
    user: &Address,
    contract: &Address,
) -> Option<crate::storage::ContractDelegate> {
    let key = (symbol_short!("cdel"), user.clone(), contract.clone());
    env.storage().persistent().get(&key)
}

pub fn set_contract_delegate(
    env: &Env,
    user: &Address,
    contract: &Address,
    delegate: &crate::storage::ContractDelegate,
) {
    let key = (symbol_short!("cdel"), user.clone(), contract.clone());
    env.storage().persistent().set(&key, delegate);
}

pub fn remove_contract_delegate(env: &Env, user: &Address, contract: &Address) {
    let key = (symbol_short!("cdel"), user.clone(), contract.clone());
    env.storage().persistent().remove(&key);
}

pub fn get_emergency_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("emg_adm"))
}

pub fn set_emergency_admin(env: &Env, admin: &Address) {
    env.storage()
        .instance()
        .set(&symbol_short!("emg_adm"), admin);
}

// ── Proposal CRUD helpers ─────────────────────────────────────────────────────

/// Retrieve a single proposal by id, or `None` if it doesn't exist.
pub fn get_proposal(env: &Env, id: u32) -> Option<GovernanceProposal> {
    let proposals = get_governance_proposals(env);
    for i in 0..proposals.len() {
        let p = proposals.get(i).unwrap();
        if p.id == id {
            return Some(p);
        }
    }
    None
}

/// Insert or replace a proposal (matched by `id`) in the stored Vec.
pub fn set_proposal(env: &Env, id: u32, proposal: &GovernanceProposal) {
    let mut proposals = get_governance_proposals(env);
    for i in 0..proposals.len() {
        if proposals.get(i).unwrap().id == id {
            proposals.set(i, proposal.clone());
            set_governance_proposals(env, &proposals);
            return;
        }
    }
    // Not found — append as a new entry.
    proposals.push_back(proposal.clone());
    set_governance_proposals(env, &proposals);
}

// ── Per-user per-proposal voted flag ─────────────────────────────────────────

pub fn has_voted(env: &Env, proposal_id: u32, user: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&(symbol_short!("p_voted"), proposal_id, user.clone()))
        .unwrap_or(false)
}

pub fn set_voted(env: &Env, proposal_id: u32, user: &Address) {
    env.storage()
        .persistent()
        .set(&(symbol_short!("p_voted"), proposal_id, user.clone()), &true);
}

// ── Monotonic proposal id counter ────────────────────────────────────────────

pub fn get_next_proposal_id(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("p_next_id"))
        .unwrap_or(0)
}

pub fn set_next_proposal_id(env: &Env, id: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("p_next_id"), &id);
}

// ── Open proposal count ───────────────────────────────────────────────────────

pub fn get_open_proposal_count(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("p_open"))
        .unwrap_or(0)
}

pub fn set_open_proposal_count(env: &Env, count: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("p_open"), &count);
}
