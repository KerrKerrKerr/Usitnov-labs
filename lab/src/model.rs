use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info, warn};
use std::fmt;
mod tests;
/// Represents a fuel record with name, date, price, and color
#[derive(Debug, Clone, PartialEq)]
pub struct Fuel {
    pub name: String,
    pub date: DateTime<Utc>,
    pub price: f64,
    pub color: (u8, u8, u8),
}

impl Fuel {
    /// Creates a new Fuel with default values
    pub fn new() -> Self {
        Fuel {
            name: String::from("Not defined"),
            date: Utc::now(),
            price: -1.0,
            color: (0, 0, 0),
        }
    }

    /// Creates a new Fuel with specified parameters
    pub fn new_param(name: String, date: DateTime<Utc>, price: f64, color: (u8, u8, u8)) -> Self {
        Fuel {
            name,
            date,
            price,
            color,
        }
    }

    /// Parses a Fuel from a string
    /// Format: <String>,<Time (yyyy.mm.dd hh.mm)>,<f64>,<u8>:<u8>:<u8>
    pub fn from_string(input: &str) -> Result<Self, String> {
        let input = input.trim();
        if input.is_empty() {
            return Err("Empty input".to_string());
        }

        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 4 {
            return Err(format!(
                "Input must have four parts separated by commas, got {}",
                parts.len()
            ));
        }

        let name = parts[0].trim().to_string();
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let date_str = parts[1].trim();
        let price_str = parts[2].trim();
        let color_string = parts[3].trim();

        let price = price_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse price '{}': {}", price_str, e))?;

        let color = Self::parse_color(color_string)?;

        let date = Self::parse_date(date_str)?;

        Ok(Fuel::new_param(name, date, price, color))
    }

    /// Parses color from string format "r:g:b"
    fn parse_color(color_string: &str) -> Result<(u8, u8, u8), String> {
        let color_parts: Vec<&str> = color_string.split(':').collect();
        if color_parts.len() != 3 {
            return Err(format!(
                "Invalid color format '{}', expected u8:u8:u8",
                color_string
            ));
        }

        let r = color_parts[0]
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse color R '{}': {}", color_parts[0], e))?;
        let g = color_parts[1]
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse color G '{}': {}", color_parts[1], e))?;
        let b = color_parts[2]
            .parse::<u8>()
            .map_err(|e| format!("Failed to parse color B '{}': {}", color_parts[2], e))?;

        Ok((r, g, b))
    }

    /// Parses date from string format "yyyy.mm.dd hh:mm"
    fn parse_date(input: &str) -> Result<DateTime<Utc>, String> {
        match NaiveDateTime::parse_from_str(input, "%Y.%m.%d %H:%M") {
            Ok(naive) => Ok(naive.and_utc()),
            Err(_) => Err(format!(
                "Invalid date format '{}', expected yyyy.mm.dd hh:mm",
                input
            )),
        }
    }
}

impl fmt::Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, Date: {}, Price: {:.2}, Color {}:{}:{}",
            self.name,
            self.date.format("%Y-%m-%d %H:%M:%S"),
            self.price,
            self.color.0,
            self.color.1,
            self.color.2
        )
    }
}

/// Storage for multiple Fuel records
#[derive(Debug, Default, Clone)]
pub struct FuelStorage {
    fuel_storage: Vec<Fuel>,
}

impl FuelStorage {
    /// Creates a new empty FuelStorage
    pub fn new() -> Self {
        FuelStorage {
            fuel_storage: Vec::new(),
        }
    }

    /// Returns all fuels as a slice
    pub fn get_all(&self) -> &[Fuel] {
        &self.fuel_storage
    }

    /// Pushes a fuel record to storage
    pub fn push(&mut self, fuel: Fuel) {
        self.fuel_storage.push(fuel);
    }

    /// Retains only elements satisfying the predicate
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Fuel) -> bool,
    {
        self.fuel_storage.retain(f);
    }

    /// Clears all records
    pub fn clear(&mut self) {
        self.fuel_storage.clear();
    }

    /// Returns the number of records
    pub fn len(&self) -> usize {
        self.fuel_storage.len()
    }

    /// Returns true if storage is empty
    pub fn is_empty(&self) -> bool {
        self.fuel_storage.is_empty()
    }
}

impl fmt::Display for FuelStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for fuel in self.fuel_storage.iter() {
            out.push_str(&format!("{}\n", fuel));
        }
        write!(f, "{}", out)
    }
}

/// Auxiliary struct for parsing fuel data from strings
pub struct FuelParser;

impl FuelParser {
    /// Creates a new parser
    pub fn new() -> Self {
        FuelParser
    }

    /// Parses multiple fuel records from content string
    /// Skips invalid lines and logs errors
    pub fn parse_content(&self, content: &str) -> FuelStorage {
        let mut storage = FuelStorage::new();
        let lines: Vec<&str> = content.lines().collect();

        info!("Starting to parse {} lines", lines.len());

        for (index, line) in lines.iter().enumerate() {
            let line_num = index + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                warn!("Line {}: Skipping empty line", line_num);
                continue;
            }

            match Fuel::from_string(trimmed) {
                Ok(fuel) => {
                    info!("Line {}: Successfully parsed fuel: {}", line_num, fuel.name);
                    storage.push(fuel);
                }
                Err(e) => {
                    error!("Line {}: Failed to parse '{}': {}", line_num, trimmed, e);
                }
            }
        }

        info!(
            "Parsing complete: {} valid records out of {} lines",
            storage.len(),
            lines.len()
        );
        storage
    }

    /// Validates a single line without adding to storage
    #[cfg(test)]
    pub fn validate_line(&self, line: &str) -> Result<Fuel, String> {
        Fuel::from_string(line)
    }
}

// ==================== Command Definition ====================

/// Represents a command from an external file
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// ADD <csv> - Add a fuel record in CSV format
    Add(String),
    /// REM <condition> - Remove records matching condition
    /// Condition format: "field <operator> value" e.g., "price < 1000"
    Remove(String),
    /// SAVE <filename> - Save data to file
    Save(String),
}

/// Parses a condition expression like "price < 1000" or "name == Diesel"
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub field: ConditionField,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionField {
    Price,
    Name,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionOperator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
}

impl Condition {
    /// Parses a condition string like "price < 1000"
    pub fn parse(condition_str: &str) -> Result<Self, String> {
        let trimmed = condition_str.trim();

        // Parse field and operator
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(format!(
                "Invalid condition format '{}', expected: field operator value",
                trimmed
            ));
        }

        let field_str = parts[0].to_lowercase();
        let operator_str = parts[1];
        let value = parts[2..].join(" ");

        let field = match field_str.as_str() {
            "price" | "sum" | "cost" => ConditionField::Price,
            "name" | "type" | "fuel" => ConditionField::Name,
            _ => return Err(format!("Unknown field '{}'", field_str)),
        };

        let operator = match operator_str {
            "<" => ConditionOperator::Less,
            "<=" => ConditionOperator::LessEqual,
            ">" => ConditionOperator::Greater,
            ">=" => ConditionOperator::GreaterEqual,
            "==" | "=" => ConditionOperator::Equal,
            "!=" | "<>" => ConditionOperator::NotEqual,
            _ => return Err(format!("Unknown operator '{}'", operator_str)),
        };

        Ok(Condition {
            field,
            operator,
            value,
        })
    }

    /// Evaluates the condition against a Fuel record
    pub fn evaluate(&self, fuel: &Fuel) -> bool {
        match self.field {
            ConditionField::Price => {
                if let Ok(value) = self.value.parse::<f64>() {
                    match self.operator {
                        ConditionOperator::Less => fuel.price < value,
                        ConditionOperator::LessEqual => fuel.price <= value,
                        ConditionOperator::Greater => fuel.price > value,
                        ConditionOperator::GreaterEqual => fuel.price >= value,
                        ConditionOperator::Equal => (fuel.price - value).abs() < 0.001,
                        ConditionOperator::NotEqual => (fuel.price - value).abs() >= 0.001,
                    }
                } else {
                    false
                }
            }
            ConditionField::Name => {
                let match_value = self.value.to_lowercase();
                let fuel_name = fuel.name.to_lowercase();
                match self.operator {
                    ConditionOperator::Less
                    | ConditionOperator::LessEqual
                    | ConditionOperator::Greater
                    | ConditionOperator::GreaterEqual => {
                        // Lexicographic comparison for strings
                        match self.operator {
                            ConditionOperator::Less => fuel_name < match_value,
                            ConditionOperator::LessEqual => fuel_name <= match_value,
                            ConditionOperator::Greater => fuel_name > match_value,
                            ConditionOperator::GreaterEqual => fuel_name >= match_value,
                            _ => false,
                        }
                    }
                    ConditionOperator::Equal => fuel_name == match_value,
                    ConditionOperator::NotEqual => fuel_name != match_value,
                }
            }
        }
    }
}

/// Parses commands from a file
pub struct CommandParser;

impl CommandParser {
    /// Creates a new command parser
    pub fn new() -> Self {
        CommandParser
    }

    /// Parses multiple commands from content string
    /// Each line should be a command: ADD <csv>, REM <condition>, or SAVE <filename>
    pub fn parse_content(&self, content: &str) -> Vec<Command> {
        let mut commands = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        info!("Starting to parse {} command lines", lines.len());

        for (index, line) in lines.iter().enumerate() {
            let line_num = index + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                warn!("Line {}: Skipping empty or comment line", line_num);
                continue;
            }

            match self.parse_single_command(trimmed) {
                Ok(cmd) => {
                    info!("Line {}: Parsed command: {:?}", line_num, cmd);
                    commands.push(cmd);
                }
                Err(e) => {
                    error!("Line {}: Failed to parse '{}': {}", line_num, trimmed, e);
                }
            }
        }

        info!("Parsing complete: {} commands", commands.len());
        commands
    }

    /// Parses a single command line
    fn parse_single_command(&self, line: &str) -> Result<Command, String> {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        let command_type = parts[0].to_uppercase();
        let args = parts.get(1).map(|s| s.trim()).unwrap_or("");

        match command_type.as_str() {
            "ADD" => {
                if args.is_empty() {
                    return Err("ADD requires CSV data".to_string());
                }
                Ok(Command::Add(args.to_string()))
            }
            "REM" | "REMOVE" => {
                if args.is_empty() {
                    return Err("REM requires condition".to_string());
                }
                // Validate condition
                Condition::parse(args)?;
                Ok(Command::Remove(args.to_string()))
            }
            "SAVE" => {
                if args.is_empty() {
                    return Err("SAVE requires filename".to_string());
                }
                Ok(Command::Save(args.to_string()))
            }
            _ => Err(format!("Unknown command '{}'", parts[0])),
        }
    }

}
