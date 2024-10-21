use serde::{Deserialize, Serialize};

pub trait Result {
    fn result(&self) -> bool;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreliminaryRound {
    first_round :bool,
    second_round: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuarterFinal {
    first_round: bool,
    second_round: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SemiFinal {
    first_round: bool,
    second_round: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Final {
    first_round: bool,
    second_round: bool,
}

impl Result for PreliminaryRound {
    fn result(&self) -> bool {
        self.first_round && self.second_round
    }
}
impl Result for QuarterFinal {
    fn result(&self) -> bool {
        self.first_round && self.second_round
    }
}

impl Result for SemiFinal {
    fn result(&self) -> bool {
        self.first_round && self.second_round
    }
}

impl Result for Final {
    fn result(&self) -> bool {
        self.first_round && self.second_round
    }
}
