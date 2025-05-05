use super::error::{AppError, AppResult};

pub struct Calculator {
    display: String,
    memory: f64,
    current_op: Option<Operation>,
    last_value: Option<f64>,
    config: CalculatorConfig,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,
}

impl Calculator {
    pub fn new(config: CalculatorConfig) -> Self {
        Self {
            display: String::from("0"),
            memory: 0.0,
            current_op: None,
            last_value: None,
            config,
        }
    }

    pub fn input_digit(&mut self, digit: char) -> AppResult<()> {
        if digit.is_digit(10) {
            if self.display == "0" {
                self.display = digit.to_string();
            } else {
                self.display.push(digit);
            }
        }
        Ok(())
    }

    pub fn operation(&mut self, op: Operation) -> AppResult<()> {
        if let Some(value) = self.display.parse::<f64>().ok() {
            if let Some(current_op) = self.current_op.take() {
                self.apply_operation(current_op, value)?;
            } else {
                self.last_value = Some(value);
            }
            self.current_op = Some(op);
            self.display = "0".to_string();
        }
        Ok(())
    }

    pub fn equals(&mut self) -> AppResult<()> {
        if let (Some(op), Some(last)) = (self.current_op.take(), self.last_value) {
            if let Ok(value) = self.display.parse::<f64>() {
                self.apply_operation(op, value)?;
            }
        }
        Ok(())
    }

    fn apply_operation(&mut self, op: Operation, value: f64) -> AppResult<()> {
        if let Some(last) = self.last_value {
            let result = match op {
                Operation::Add => last + value,
                Operation::Subtract => last - value,
                Operation::Multiply => last * value,
                Operation::Divide => {
                    if value == 0.0 {
                        return Err(AppError::DivisionByZero);
                    }
                    last / value
                }
                Operation::Power => last.powf(value),
                Operation::Root => value.powf(1.0 / last),
            };
            self.display = result.to_string();
            self.last_value = Some(result);
        }
        Ok(())
    }
} 