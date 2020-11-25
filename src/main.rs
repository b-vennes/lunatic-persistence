use lunatic_persistence::actor::{PersistenceActor, send_command};
use lunatic_persistence::effect::CommandEffect;
use lunatic_persistence::storage::{Storage, GetStateResult};

use lunatic::Channel;
use serde::{Serialize, Deserialize};

fn main() {
    let channel = Channel::new(0);
    send_command_to_sum_actor("test".to_string(), Command::Add(5), channel.clone());

    let result = channel.receive();

    println!("Result: {:?}", result);
}

struct SumPersistenceActor {}

impl PersistenceActor<Command, Event, State, String> for SumPersistenceActor {

    fn handle_command(_state: &Option<State>, command: Command) -> Result<CommandEffect<Event, String>, String> {
        let effect = match command {
            Command::Add(v) =>
                CommandEffect::publish(Event::Added(v))
                    .then_reply(format!("added {}", v)),
            Command::Subtract(v) =>
                CommandEffect::publish(Event::Subtracted(v))
                    .then_reply(format!("subtracted {}", v))
        };

        Ok(effect)
    }

    fn handle_event(state: Option<State>, event: &Event) -> Option<State> {
        match event {
            Event::Added(v) => match state {
                Some(state) => Some(State { value: state.value + v }),
                None => Some(State { value: *v })
            },
            Event::Subtracted(v) => match state {
                Some(state) => Some(State { value: state.value - v }),
                None => Some(State { value: -*v })
            },
        }
    }
}

fn send_command_to_sum_actor(name: String, command: Command, channel: Channel<Result<Option<String>, String>>) {
    send_command::<'static, SumPersistenceActor, BasicStorage, Command, Event, State, String>
        (name, command, channel);
}

struct BasicStorage {}

impl Storage<State, Event> for BasicStorage {
    fn get_state(name: &String) -> GetStateResult<State> {
        GetStateResult::StateNotFound
    }

    fn persist_events_and_state(name: &String, events: Vec<Event>, state: Option<State>) -> Result<(), String> {
        Ok(())
    }

    fn get_events(name: &String) -> Result<Vec<Event>, String> {
        Ok(vec!(Event::Added(5), Event::Subtracted(3)))
    }
}

#[derive(Serialize, Deserialize)]
enum Command {
    Add(i32),
    Subtract(i32),
}

#[derive(Serialize, Deserialize)]
enum Event {
    Added(i32),
    Subtracted(i32)
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    pub value: i32
}
