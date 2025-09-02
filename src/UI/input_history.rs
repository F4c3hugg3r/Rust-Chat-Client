// InputHistory manageges the inputHistory

#[derive(Debug, Clone)]
pub struct InputHistory {
    pub current: i32,
    pub inputs: Vec<String>,
    pub first: bool,
}

impl InputHistory {
    pub fn save_input(&mut self, input: String) {
        if !input.is_empty() {
            self.inputs.push(input);
        }
        self.first = true;
        self.set_current_history_index(-1);
    }

    pub fn set_current_history_index(&mut self, pending: i32) -> i32 {
        self.current = match pending {
            p if p >= self.inputs.len() as i32 => 0,
            p if p < 0 => self.inputs.len().saturating_sub(1) as i32,
            p => p,
        };
        self.current
    }

    pub fn check_first(&mut self) -> bool {
        if self.first {
            self.first = false;
            return true;
        }
        false
    }
}
