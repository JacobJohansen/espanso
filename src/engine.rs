use crate::matcher::{Match, MatchReceiver};
use crate::keyboard::KeyboardSender;

pub struct Engine<'a>{
    sender: &'a dyn KeyboardSender
}

impl <'a> Engine<'a> {
    pub fn new(sender: &'a dyn KeyboardSender) -> Engine<'a> {
        Engine{sender}
    }
}

impl <'a> MatchReceiver for Engine<'a>{
    fn on_match(&self, m: &Match) {
        self.sender.delete_string(m.trigger.len() as i32);
        self.sender.send_string(m.replace.as_str());
    }
}