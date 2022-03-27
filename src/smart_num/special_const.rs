use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum SpecialConst {
    Zero,
    One,
    Pi,
    E,
}

impl Display for SpecialConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match &self {
            SpecialConst::Zero => "0",
            SpecialConst::One => "1",
            SpecialConst::Pi => "pi",
            SpecialConst::E => "e",
        };
        write!(f, "{}", symbol)
    }
}
