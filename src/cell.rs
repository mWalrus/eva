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
            DataType::Empty => ("U".into(), Color::Black),
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
            CellErrorType::Div0 => "EA",
            CellErrorType::NA => "EB",
            CellErrorType::Name => "EC",
            CellErrorType::Null => "ED",
            CellErrorType::Num => "EE",
            CellErrorType::Ref => "EF",
            CellErrorType::Value => "EG",
            CellErrorType::GettingData => "EH",
        };
        String::from(msg)
    }
}

pub static LEGEND_ITEMS: &[&str] = &[
    "Regular Types:",
    "S: String",
    "I: Int",
    "F: Float",
    "B: Bool",
    "U: Undefined",
    "",
    "Errors:",
    "EA: Div by 0",
    "EB: Not available",
    "EC: Invalid name",
    "ED: Null value",
    "EE: Invalid number",
    "EF: Cell ref error",
    "EG: Value error",
    "EH: Get data error",
];
