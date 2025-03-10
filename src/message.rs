#[derive(PartialEq)]
pub enum MessageState {
    Solved,
    Error,
    None
}

pub struct Message{
    pub title: String,
    pub message: String,
    pub state: MessageState
}

impl Message {
    pub fn new() -> Self {
        Message {
            title: String::from(""),
            message: String::from(""),
            state: MessageState::None
        }
    }

    pub fn set(&mut self, state: MessageState) {
        match state {
            MessageState::Solved => {
                self.title = String::from("Solved!");
                self.message = String::from("The sudoku has been solved successfully.");
            }
            MessageState::Error => {
                self.title = String::from("Error!");
                self.message = String::from("The sudoku, in the current state, has no solution. Try to remove some of the entered values or press clear to bring back the initial state.");
            }
            MessageState::None => {
                self.title = String::from("");
                self.message = String::from("")
            }
        }
        self.state = state;
    }
}