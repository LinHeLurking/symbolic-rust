use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstType {
    Zero,
    One,
    Pi,
    E,
    Nothing,
}

impl Display for ConstType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            ConstType::Zero => "0",
            ConstType::One => "1",
            ConstType::Pi => "pi",
            ConstType::E => "e",
            ConstType::Nothing => panic!("This is not a special number!!!"),
        };
        write!(f, "{}", symbol)
    }
}
