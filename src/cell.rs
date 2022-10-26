use office::{CellErrorType, DataType};
use tui::style::Color;

// NOTE: workaround for E0210
struct CET(CellErrorType);

pub struct Cell {
    pub value: String,
    pub col: char,
    pub color: Color,
}

impl From<(&DataType, char)> for Cell {
    fn from(other: (&DataType, char)) -> Self {
        let (value, color) = match other.0 {
            DataType::Int(_) => ("I".into(), Color::Blue),
            DataType::Float(_) => ("F".into(), Color::Magenta),
            DataType::String(_) => ("S".into(), Color::Green),
            DataType::Bool(_) => ("B".into(), Color::Yellow),
            DataType::Error(e) => (CET(e.to_owned()).into(), Color::Red),
            DataType::Empty => ("E".into(), Color::Black),
        };
        Self {
            value,
            col: other.1,
            color,
        }
    }
}

impl From<CET> for String {
    fn from(e: CET) -> Self {
        let msg = match e.0 {
            CellErrorType::Div0 => "Div by 0",
            CellErrorType::NA => "Not available",
            CellErrorType::Name => "Invalid name",
            CellErrorType::Null => "Null value",
            CellErrorType::Num => "Invalid number",
            CellErrorType::Ref => "Cell ref error",
            CellErrorType::Value => "Value error",
            CellErrorType::GettingData => "Get data error",
        };
        String::from(msg)
    }
}
