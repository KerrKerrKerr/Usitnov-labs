use crate::model::{Command, CommandParser, Condition, Fuel};
use crate::{AppState, Message, pick_command_file_async, pick_file_async, save_file_async};
use chrono::Utc;

use crate::model::*;

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
        let cmd = parser
            .parse_single_command("ADD Gasoline,2024.06.01 12:00,3.99,255:0:0")
            .unwrap();
        assert_eq!(
            cmd,
            Command::Add("Gasoline,2024.06.01 12:00,3.99,255:0:0".to_string())
        );
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
        let content =
            "ADD Gasoline,2024.06.01 12:00,3.99,255:0:0\nREM price < 1000\nSAVE output.csv";
        let commands = parser.parse_content(content);
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_command_parser_parse_content_with_comments() {
        let parser = CommandParser::new();
        let content =
            "# This is a comment\nADD Gasoline,2024.06.01 12:00,3.99,255:0:0\n\nREM price < 1000";
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
        storage.push(Fuel::new_param(
            "A".to_string(),
            Utc::now(),
            500.0,
            (0, 0, 0),
        ));
        storage.push(Fuel::new_param(
            "B".to_string(),
            Utc::now(),
            1500.0,
            (0, 0, 0),
        ));
        storage.push(Fuel::new_param(
            "C".to_string(),
            Utc::now(),
            200.0,
            (0, 0, 0),
        ));

        // Apply REM price < 1000
        let cond = Condition::parse("price < 1000").unwrap();
        storage.retain(|f| !cond.evaluate(f));

        assert_eq!(storage.len(), 1);
        assert_eq!(storage.get_all()[0].name, "B");
    }

    #[test]
    fn test_command_remove_by_name() {
        let mut storage = FuelStorage::new();
        storage.push(Fuel::new_param(
            "Gasoline".to_string(),
            Utc::now(),
            3.99,
            (255, 0, 0),
        ));
        storage.push(Fuel::new_param(
            "Diesel".to_string(),
            Utc::now(),
            2.50,
            (0, 255, 0),
        ));
        storage.push(Fuel::new_param(
            "Oil".to_string(),
            Utc::now(),
            1.00,
            (0, 0, 255),
        ));

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
        let content =
            "Gasoline,2024.06.01 12:00,3.99,255:0:0\nDiesel,2024.06.02 13:00,2.50,0:0:255";
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
