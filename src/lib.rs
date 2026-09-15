//! soroban-contributor-points
//!
//! A minimal on-chain contributor points registry for Soroban.
//!
//! A `maintainer` address registers issues with a point value (mirroring
//! the Trivial/Medium/High complexity tiers used by programs like Drips
//! Wave), and records who resolved each issue. Contributor point totals
//! accumulate on-chain, giving a transparent, verifiable record of
//! contribution history that isn't dependent on any single off-chain
//! platform staying up or keeping accurate records.
//!
//! This is not a replacement for a full contribution-rewards platform —
//! it has no GitHub integration, no automated PR verification, and no
//! payout logic of its own. It's a small primitive that such a platform
//! (or a simpler, single-maintainer project) could build on: an
//! append-only, on-chain ledger of "who resolved what, worth how much."

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Copy, PartialEq)]
pub enum Complexity {
    Trivial,
    Medium,
    High,
}

impl Complexity {
    fn points(&self) -> u32 {
        match self {
            Complexity::Trivial => 100,
            Complexity::Medium => 150,
            Complexity::High => 200,
        }
    }
}

#[contracttype]
pub enum DataKey {
    Maintainer,
    IssuePoints(Symbol),
    IssueResolvedBy(Symbol),
    ContributorPoints(Address),
}

#[contract]
pub struct ContributorPointsContract;

#[contractimpl]
impl ContributorPointsContract {
    /// One-time setup: sets the maintainer address allowed to register
    /// and resolve issues. Should be called immediately after deploy.
    pub fn init(env: Env, maintainer: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Maintainer),
            "already initialized"
        );
        env.storage().instance().set(&DataKey::Maintainer, &maintainer);
    }

    /// Maintainer registers a new issue with a complexity tier.
    pub fn add_issue(env: Env, issue_id: Symbol, complexity: Complexity) {
        Self::require_maintainer(&env);
        assert!(
            !env.storage().persistent().has(&DataKey::IssuePoints(issue_id.clone())),
            "issue already registered"
        );
        env.storage()
            .persistent()
            .set(&DataKey::IssuePoints(issue_id), &complexity.points());
    }

    /// Maintainer records that `contributor` resolved `issue_id`,
    /// crediting them with that issue's points. Each issue can only be
    /// resolved once.
    pub fn resolve_issue(env: Env, issue_id: Symbol, contributor: Address) {
        Self::require_maintainer(&env);

        let points: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::IssuePoints(issue_id.clone()))
            .expect("issue not registered");

        assert!(
            !env.storage().persistent().has(&DataKey::IssueResolvedBy(issue_id.clone())),
            "issue already resolved"
        );
        env.storage()
            .persistent()
            .set(&DataKey::IssueResolvedBy(issue_id), &contributor);

        let current: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::ContributorPoints(contributor.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::ContributorPoints(contributor), &(current + points));
    }

    /// Read-only: total points earned by a contributor.
    pub fn points_of(env: Env, contributor: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ContributorPoints(contributor))
            .unwrap_or(0)
    }

    /// Read-only: who resolved a given issue, if anyone.
    pub fn resolved_by(env: Env, issue_id: Symbol) -> Option<Address> {
        env.storage().persistent().get(&DataKey::IssueResolvedBy(issue_id))
    }

    fn require_maintainer(env: &Env) {
        let maintainer: Address = env
            .storage()
            .instance()
            .get(&DataKey::Maintainer)
            .expect("contract not initialized");
        maintainer.require_auth();
    }
}

mod test;
