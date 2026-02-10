use std::collections::VecDeque;

#[derive(Debug)]
pub struct Elevator {
    floor: u8,
    state: State,
    queue: VecDeque<u8>,
}

pub struct Status {
    pub floor: u8,
    pub state: State,
    pub queue: VecDeque<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug, PartialEq)]
pub enum ElevatorError {
    InvalidFloor(u8),
    DoorsAlreadyOpen,
    DoorsAlreadyClosed,
    CannotOpenWhileMoving,
    CannotMoveDoorsOpen,
    EmptyQueue,
}

impl Elevator {
    pub fn new(f: u8) -> Result<Self, ElevatorError> {
        if f > 5 {
            return Err(ElevatorError::InvalidFloor(f));
        }

        Ok(Self {
            floor: f,
            state: State::Idle,
            queue: VecDeque::new(),
        })
    }

    pub fn floor(&self) -> u8 {
        self.floor
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn queue(&self) -> VecDeque<u8> {
        self.queue.clone()
    }

    pub fn status(&self) -> Status {
        Status {
            floor: self.floor,
            state: self.state,
            queue: self.queue.clone(),
        }
    }

    fn get_next_move(&self) -> State {
        if self.queue.is_empty() {
            State::Idle
        } else if self.floor < self.queue[0] {
            State::MovingUp
        } else {
            State::MovingDown
        }
    }

    pub fn call(&mut self, new_floor: u8) -> Result<&str, ElevatorError> {
        if self.floor == new_floor {
            Ok("same floor allowed")
        } else if self.queue.contains(&new_floor) {
            Ok("duplicate ignored")
        } else if new_floor > 5 {
            Err(ElevatorError::InvalidFloor(new_floor))
        } else {
            self.queue.push_back(new_floor);

            if self.state == State::Idle {
                self.state = self.get_next_move();
            }

            Ok("call accepted")
        }
    }

    pub fn open_doors(&mut self) -> Result<&str, ElevatorError> {
        match self.state {
            State::DoorsOpen => Err(ElevatorError::DoorsAlreadyOpen),
            State::MovingUp | State::MovingDown => Err(ElevatorError::CannotOpenWhileMoving),
            State::Idle => {
                self.state = State::DoorsOpen;
                Ok("open doors from idle")
            }
        }
    }

    pub fn close_doors(&mut self) -> Result<&str, ElevatorError> {
        match self.state {
            State::DoorsOpen => {
                self.state = self.get_next_move();
                Ok("close doors")
            }
            _ => Err(ElevatorError::DoorsAlreadyClosed),
        }
    }

    pub fn step(&mut self) -> Result<&str, ElevatorError> {
        if self.state == State::DoorsOpen {
            Err(ElevatorError::CannotMoveDoorsOpen)
        } else if self.queue.is_empty() {
            Err(ElevatorError::EmptyQueue)
        } else {
            if self.state == State::MovingUp {
                self.floor += 1;
            } else if self.state == State::MovingDown {
                self.floor -= 1;
            }

            if self.floor == self.queue[0] {
                self.queue.pop_front();
                self.state = State::DoorsOpen;
            }

            Ok("step {self.floor}")
        }
    }
}
