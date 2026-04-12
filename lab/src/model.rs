use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info, warn};
use std::fmt;

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
            "<=" | "<>=" => ConditionOperator::LessEqual,
            ">" => ConditionOperator::Greater,
            ">=" | ">= " => ConditionOperator::GreaterEqual,
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
                    ConditionOperator::Less |
                    ConditionOperator::LessEqual |
                    ConditionOperator::Greater |
                    ConditionOperator::GreaterEqual => {
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

    /// Validates a single command without parsing
    pub fn validate_command(&self, line: &str) -> Result<Command, String> {
        self.parse_single_command(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Command Tests ====================

    #[test]
    fn test_condition_parse_price_less() {
        let cond = Condition::parse("price < 1000").unwrap();
        assert_eq!(cond.field, ConditionField::Price);
        assert_eq!(cond.operator, ConditionOperator::Less);
        assert_eq!(cond.value, "1000");
    }

    #[test]
    fn test_condition_parse_name_equal() {
        let cond = Condition::parse("name == Diesel").unwrap();
        assert_eq!(cond.field, ConditionField::Name);
        assert_eq!(cond.operator, ConditionOperator::Equal);
        assert_eq!(cond.value, "Diesel");
    }

    #[test]
    fn test_condition_parse_with_sum_keyword() {
        let cond = Condition::parse("sum < 500").unwrap();
        assert_eq!(cond.field, ConditionField::Price);
    }

    #[test]
    fn test_condition_parse_invalid_field() {
        let result = Condition::parse("unknown < 100");
        assert!(result.is_err());
    }

    #[test]
    fn test_condition_parse_invalid_operator() {
        let result = Condition::parse("price ^ 100");
        assert!(result.is_err());
    }

    #[test]
    fn test_condition_evaluate_price_less() {
        let cond = Condition::parse("price < 5.0").unwrap();
        let fuel = Fuel::new_param("Test".to_string(), Utc::now(), 3.0, (255, 0, 0));
        assert!(cond.evaluate(&fuel));

        let fuel2 = Fuel::new_param("Test2".to_string(), Utc::now(), 7.0, (0, 255, 0));
        assert!(!cond.evaluate(&fuel2));
    }

    #[test]
    fn test_condition_evaluate_price_greater_equal() {
        let cond = Condition::parse("price >= 10.0").unwrap();
        let fuel = Fuel::new_param("Test".to_string(), Utc::now(), 10.0, (255, 0, 0));
        assert!(cond.evaluate(&fuel));

        let fuel2 = Fuel::new_param("Test2".to_string(), Utc::now(), 9.0, (0, 255, 0));
        assert!(!cond.evaluate(&fuel2));
    }

    #[test]
    fn test_condition_evaluate_name_equal() {
        let cond = Condition::parse("name == Gasoline").unwrap();
        let fuel = Fuel::new_param("Gasoline".to_string(), Utc::now(), 5.0, (255, 0, 0));
        assert!(cond.evaluate(&fuel));

        let fuel2 = Fuel::new_param("Diesel".to_string(), Utc::now(), 3.0, (0, 255, 0));
        assert!(!cond.evaluate(&fuel2));
    }

    #[test]
    fn test_condition_evaluate_name_not_equal() {
        let cond = Condition::parse("name != Diesel").unwrap();
        let fuel = Fuel::new_param("Gasoline".to_string(), Utc::now(), 5.0, (255, 0, 0));
        assert!(cond.evaluate(&fuel));

        let fuel2 = Fuel::new_param("Diesel".to_string(), Utc::now(), 3.0, (0, 255, 0));
        assert!(!cond.evaluate(&fuel2));
    }

    #[test]
    fn test_command_parser_add() {
        let parser = CommandParser::new();
        let cmd = parser.parse_single_command("ADD Gasoline,2024.06.01 12:00,3.99,255:0:0").unwrap();
        assert_eq!(cmd, Command::Add("Gasoline,2024.06.01 12:00,3.99,255:0:0".to_string()));
    }

    #[test]
    fn test_command_parser_rem() {
        let parser = CommandParser::new();
        let cmd = parser.parse_single_command("REM price < 1000").unwrap();
        assert_eq!(cmd, Command::Remove("price < 1000".to_string()));
    }

    #[test]
    fn test_command_parser_rem_remove_alias() {
        let parser = CommandParser::new();
        let cmd = parser.parse_single_command("REMOVE sum < 500").unwrap();
        assert_eq!(cmd, Command::Remove("sum < 500".to_string()));
    }

    #[test]
    fn test_command_parser_save() {
        let parser = CommandParser::new();
        let cmd = parser.parse_single_command("SAVE output.csv").unwrap();
        assert_eq!(cmd, Command::Save("output.csv".to_string()));
    }

    #[test]
    fn test_command_parser_add_empty() {
        let parser = CommandParser::new();
        let result = parser.parse_single_command("ADD");
        assert!(result.is_err());
    }

    #[test]
    fn test_command_parser_unknown_command() {
        let parser = CommandParser::new();
        let result = parser.parse_single_command("FOO bar");
        assert!(result.is_err());
    }

    #[test]
    fn test_command_parser_parse_content() {
        let parser = CommandParser::new();
        let content = "ADD Gasoline,2024.06.01 12:00,3.99,255:0:0\nREM price < 1000\nSAVE output.csv";
        let commands = parser.parse_content(content);
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_command_parser_parse_content_with_comments() {
        let parser = CommandParser::new();
        let content = "# This is a comment\nADD Gasoline,2024.06.01 12:00,3.99,255:0:0\n\nREM price < 1000";
        let commands = parser.parse_content(content);
        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn test_command_parser_parse_content_empty() {
        let parser = CommandParser::new();
        let content = "";
        let commands = parser.parse_content(content);
        assert!(commands.is_empty());
    }

    // ==================== Fuel Tests ====================

    #[test]
    fn test_fuel_new_default_values() {
        let fuel = Fuel::new();
        assert_eq!(fuel.name, "Not defined");
        assert_eq!(fuel.price, -1.0);
        assert_eq!(fuel.color, (0, 0, 0));
    }

    #[test]
    fn test_fuel_new_param() {
        let date = Utc::now();
        let fuel = Fuel::new_param("Diesel".to_string(), date, 2.5, (255, 0, 0));
        assert_eq!(fuel.name, "Diesel");
        assert_eq!(fuel.price, 2.5);
        assert_eq!(fuel.color, (255, 0, 0));
    }

    #[test]
    fn test_fuel_from_string_valid() {
        let input = "Gasoline,2024.06.01 12:00,3.99,255:0:0";
        let result = Fuel::from_string(input);
        assert!(result.is_ok());

        let fuel = result.unwrap();
        assert_eq!(fuel.name, "Gasoline");
        assert_eq!(fuel.price, 3.99);
        assert_eq!(fuel.color, (255, 0, 0));
    }

    #[test]
    fn test_fuel_from_string_with_spaces() {
        let input = "  Gasoline  ,  2024.06.01 12:00  ,  3.99  ,  255:0:0  ";
        let result = Fuel::from_string(input);
        assert!(result.is_ok());

        let fuel = result.unwrap();
        assert_eq!(fuel.name, "Gasoline");
        assert_eq!(fuel.price, 3.99);
    }

    #[test]
    fn test_fuel_from_string_empty_name() {
        let input = ",2024.06.01 12:00,3.99,255:0:0";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Name cannot be empty"));
    }

    #[test]
    fn test_fuel_from_string_invalid_date_format() {
        let input = "Gasoline,01-06-2024 12:00,3.99,255:0:0";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid date format"));
    }

    #[test]
    fn test_fuel_from_string_invalid_price() {
        let input = "Gasoline,2024.06.01 12:00,abc,255:0:0";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse price"));
    }

    #[test]
    fn test_fuel_from_string_missing_parts() {
        let input = "Gasoline,2024.06.01 12:00,3.99";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("four parts"));
    }

    #[test]
    fn test_fuel_from_string_empty_input() {
        let input = "";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Empty input"));
    }

    #[test]
    fn test_fuel_from_string_whitespace_only() {
        let input = "   ";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_fuel_from_string_invalid_color_format() {
        let input = "Gasoline,2024.06.01 12:00,3.99,255-0-0";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid color format"));
    }

    #[test]
    fn test_fuel_from_string_invalid_color_value() {
        let input = "Gasoline,2024.06.01 12:00,3.99,abc:0:0";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse color R"));
    }

    #[test]
    fn test_fuel_from_string_color_out_of_range() {
        // u8::parse will fail on values > 255
        let input = "Gasoline,2024.06.01 12:00,3.99,256:0:0";
        let result = Fuel::from_string(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_fuel_display() {
        let date = Utc::now();
        let fuel = Fuel::new_param("Test".to_string(), date, 1.5, (100, 150, 200));
        let display = format!("{}", fuel);
        assert!(display.contains("Test"));
        assert!(display.contains("1.50"));
        assert!(display.contains("100:150:200"));
    }

    // ==================== FuelStorage Tests ====================

    #[test]
    fn test_fuel_storage_new_empty() {
        let storage = FuelStorage::new();
        assert!(storage.is_empty());
        assert_eq!(storage.len(), 0);
    }

    #[test]
    fn test_fuel_storage_push_and_get() {
        let mut storage = FuelStorage::new();
        let fuel = Fuel::new_param("Diesel".to_string(), Utc::now(), 2.0, (0, 0, 255));

        storage.push(fuel.clone());

        assert_eq!(storage.len(), 1);
        let all = storage.get_all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0], fuel);
    }

    #[test]
    fn test_fuel_storage_retain() {
        let mut storage = FuelStorage::new();
        storage.push(Fuel::new_param("A".to_string(), Utc::now(), 1.0, (0, 0, 0)));
        storage.push(Fuel::new_param("B".to_string(), Utc::now(), 2.0, (0, 0, 0)));
        storage.push(Fuel::new_param("C".to_string(), Utc::now(), 3.0, (0, 0, 0)));

        storage.retain(|f| f.price > 1.0);

        assert_eq!(storage.len(), 2);
        let names: Vec<String> = storage.get_all().iter().map(|f| f.name.clone()).collect();
        assert!(!names.contains(&"A".to_string()));
        assert!(names.contains(&"B".to_string()));
        assert!(names.contains(&"C".to_string()));
    }

    #[test]
    fn test_fuel_storage_clear() {
        let mut storage = FuelStorage::new();
        storage.push(Fuel::new());
        storage.push(Fuel::new());

        storage.clear();

        assert!(storage.is_empty());
    }

    #[test]
    fn test_fuel_storage_display() {
        let mut storage = FuelStorage::new();
        storage.push(Fuel::new_param(
            "Test".to_string(),
            Utc::now(),
            1.0,
            (0, 0, 0),
        ));
        let display = format!("{}", storage);
        assert!(display.contains("Test"));
    }

    // ==================== FuelParser Tests ====================

    #[test]
    fn test_fuel_parser_new() {
        let parser = FuelParser::new();
        // Just verify it creates successfully
        let _ = parser;
    }

    #[test]
    fn test_fuel_parser_parse_content_valid() {
        let parser = FuelParser::new();
        let content =
            "Gasoline,2024.06.01 12:00,3.99,255:0:0\nDiesel,2024.06.02 13:00,2.50,0:0:255";

        let storage = parser.parse_content(content);

        assert_eq!(storage.len(), 2);
        assert_eq!(storage.get_all()[0].name, "Gasoline");
        assert_eq!(storage.get_all()[1].name, "Diesel");
    }

    #[test]
    fn test_fuel_parser_parse_content_with_invalid_lines() {
        let parser = FuelParser::new();
        let content = "Gasoline,2024.06.01 12:00,3.99,255:0:0\ninvalid line\nDiesel,2024.06.02 13:00,2.50,0:0:255";

        let storage = parser.parse_content(content);

        // Should have 2 valid entries, skipping the invalid line
        assert_eq!(storage.len(), 2);
    }

    #[test]
    fn test_fuel_parser_parse_content_empty_lines() {
        let parser = FuelParser::new();
        let content =
            "Gasoline,2024.06.01 12:00,3.99,255:0:0\n\n\nDiesel,2024.06.02 13:00,2.50,0:0:255";

        let storage = parser.parse_content(content);

        assert_eq!(storage.len(), 2);
    }

    #[test]
    fn test_fuel_parser_parse_content_only_invalid() {
        let parser = FuelParser::new();
        let content = "invalid line\nanother bad line\n";

        let storage = parser.parse_content(content);

        assert!(storage.is_empty());
    }

    #[test]
    fn test_fuel_parser_parse_content_empty() {
        let parser = FuelParser::new();
        let content = "";

        let storage = parser.parse_content(content);

        assert!(storage.is_empty());
    }

    #[test]
    fn test_fuel_parser_validate_line_valid() {
        let parser = FuelParser::new();
        let result = parser.validate_line("Gasoline,2024.06.01 12:00,3.99,255:0:0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_fuel_parser_validate_line_invalid() {
        let parser = FuelParser::new();
        let result = parser.validate_line("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_fuel_parser_parse_content_whitespace_handling() {
        let parser = FuelParser::new();
        let content = "  Gasoline  ,  2024.06.01 12:00  ,  3.99  ,  255:0:0  \n\n   \nDiesel,2024.06.02 13:00,2.50,0:0:255";

        let storage = parser.parse_content(content);

        assert_eq!(storage.len(), 2);
    }

    // ==================== Command Execution Tests ====================

    #[test]
    fn test_command_remove_by_price() {
        let mut storage = FuelStorage::new();
        storage.push(Fuel::new_param("A".to_string(), Utc::now(), 500.0, (0, 0, 0)));
        storage.push(Fuel::new_param("B".to_string(), Utc::now(), 1500.0, (0, 0, 0)));
        storage.push(Fuel::new_param("C".to_string(), Utc::now(), 200.0, (0, 0, 0)));

        // Apply REM price < 1000
        let cond = Condition::parse("price < 1000").unwrap();
        storage.retain(|f| !cond.evaluate(f));

        assert_eq!(storage.len(), 1);
        assert_eq!(storage.get_all()[0].name, "B");
    }

    #[test]
    fn test_command_remove_by_name() {
        let mut storage = FuelStorage::new();
        storage.push(Fuel::new_param("Gasoline".to_string(), Utc::now(), 3.99, (255, 0, 0)));
        storage.push(Fuel::new_param("Diesel".to_string(), Utc::now(), 2.50, (0, 255, 0)));
        storage.push(Fuel::new_param("Oil".to_string(), Utc::now(), 1.00, (0, 0, 255)));

        // Apply REM name == Diesel
        let cond = Condition::parse("name == Diesel").unwrap();
        storage.retain(|f| !cond.evaluate(f));

        assert_eq!(storage.len(), 2);
        let names: Vec<String> = storage.get_all().iter().map(|f| f.name.clone()).collect();
        assert!(!names.contains(&"Diesel".to_string()));
        assert!(names.contains(&"Gasoline".to_string()));
        assert!(names.contains(&"Oil".to_string()));
    }

    #[test]
    fn test_command_execution_workflow() {
        let mut storage = FuelStorage::new();
        let parser = FuelParser::new();

        // Initial data
        let content = "Gasoline,2024.06.01 12:00,3.99,255:0:0\nDiesel,2024.06.02 13:00,2.50,0:0:255";
        let initial = parser.parse_content(content);
        for f in initial.get_all() {
            storage.push(f.clone());
        }

        assert_eq!(storage.len(), 2);

        // Apply commands
        let cmd_parser = CommandParser::new();
        let commands = cmd_parser.parse_content("REM price < 3.0\nSAVE output.csv");

        for cmd in commands {
            match cmd {
                Command::Add(csv) => {
                    if let Ok(fuel) = Fuel::from_string(&csv) {
                        storage.push(fuel);
                    }
                }
                Command::Remove(condition) => {
                    let cond = Condition::parse(&condition).unwrap();
                    storage.retain(|f| !cond.evaluate(f));
                }
                Command::Save(_) => {
                    // In real implementation, save to file
                }
            }
        }

        // After REM price < 3.0, only Gasoline (3.99) should remain
        assert_eq!(storage.len(), 1);
        assert_eq!(storage.get_all()[0].name, "Gasoline");
    }
}
