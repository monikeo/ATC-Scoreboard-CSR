use serde::{Deserialize, Serialize};

use crate::data::performance::{
    Final,
    SemiFinal,
    QuarterFinal,
    PreliminaryRound
};

pub struct Competitor{
    id: usize,
    name: String,
    preliminary_round: PreliminaryRound,
    quarter_final: QuarterFinal,
    semi_final: SemiFinal,
    final_round: Final
}
