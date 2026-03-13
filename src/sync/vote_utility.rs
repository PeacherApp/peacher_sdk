use crate::{prelude::ExternalMemberVote, sdk::Vote};

/// A helper trait to determine if a vote succeeds
pub trait VoteSuccess {
    fn succeeds(&self, votes: impl Iterator<Item = Vote>) -> bool;
}

/// Votes succeed by simple majority
pub struct SimpleMajority;
impl VoteSuccess for SimpleMajority {
    fn succeeds(&self, votes: impl Iterator<Item = Vote>) -> bool {
        votes
            .fold(0i32, |prev_total, vote| match vote {
                Vote::Yes => prev_total + 1,
                Vote::No => prev_total - 1,
                _ => prev_total,
            })
            .is_positive()
    }
}

pub trait VoteSuccessExt {
    fn succeeds(&self, strategy: impl VoteSuccess) -> bool;
}
impl VoteSuccessExt for [Vote] {
    fn succeeds(&self, strategy: impl VoteSuccess) -> bool {
        strategy.succeeds(self.iter().copied())
    }
}
impl VoteSuccessExt for Vec<Vote> {
    fn succeeds(&self, strategy: impl VoteSuccess) -> bool {
        strategy.succeeds(self.iter().copied())
    }
}

impl VoteSuccessExt for [ExternalMemberVote] {
    fn succeeds(&self, strategy: impl VoteSuccess) -> bool {
        strategy.succeeds(self.iter().map(|v| v.vote))
    }
}

impl VoteSuccessExt for Vec<ExternalMemberVote> {
    fn succeeds(&self, strategy: impl VoteSuccess) -> bool {
        strategy.succeeds(self.iter().map(|v| v.vote))
    }
}

#[test]
fn test_simple_majority() {
    let votes = [Vote::Yes, Vote::Yes, Vote::No];
    assert!(votes.succeeds(SimpleMajority))
}
