
pub enum RoundType {
    RoundOf16,
    QuarterFinal,
    SemiFinal,
    Final
}

pub struct SubRound {
    first_round: String,
    second_round: String,
}

pub struct Round {
    round_type: RoundType,
    sub_round: SubRound
}

impl RoundType {
    pub fn to_string(&self) -> String {
        match self{
            Self::RoundOf16 => "Round of 16".to_string(),
            Self::QuarterFinal => "Quarter Final".to_string(),
            Self::SemiFinal => "Semi Final".to_string(),
            Self::Final => "Final".to_string()
        }
    }
}

impl SubRound {
    pub fn default() -> Self {
        Self {
            first_round: "round 1".to_string(),
            second_round: "round 2".to_string(),
        }
    }
    pub fn first_round_to_string(&self) -> String {
        self.first_round.clone()
    }
    pub fn first_round_as_str(&self) -> &str {
        &self.first_round
    }

    pub fn second_round_to_string(&self) -> String {
        self.second_round.clone()
    }
    pub fn second_round_as_str(&self) -> &str {
        &self.second_round
    }

}

impl Round {
    pub fn new(round_type: RoundType) -> Self {
        let sub_round = SubRound::default();
        Self {
            round_type,
            sub_round
        }
    }
    pub fn round_type(&self) -> &RoundType {
        &self.round_type
    }
    pub fn sub_round(&self) -> &SubRound {
        &self.sub_round
    }
}
