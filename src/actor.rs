use super::storage;
use super::effect::Effect;

use lunatic::{Channel, Process};
use serde::{Serialize, Deserialize};

pub trait PersistenceActor<TCommand, TEvent, TState, TResponse> {
    fn handle_command(state: &Option<TState>, command: TCommand) -> Result<Effect<TEvent, TResponse>, String>;

    fn handle_event(state: Option<TState>, event: &TEvent) -> Option<TState>;
}

pub fn send_command<'de, TActor, TStorage, TCommand, TEvent, TState, TResponse>
    (name: String, command: TCommand, channel: Channel<Result<Option<TResponse>, String>>) -> () where
    TCommand: Serialize + Deserialize<'de>,
    TResponse: Serialize + Deserialize<'de>,
    TActor: PersistenceActor<TCommand, TEvent, TState, TResponse>,
    TStorage: storage::Storage<TState, TEvent>,
{
    let _ = Process::spawn((name, command, channel), process::<TActor, TStorage, TCommand, TEvent, TState, TResponse>);
}

fn process<'de, TActor, TStorage, TCommand, TEvent, TState, TResponse>
    (context: (String, TCommand, Channel<Result<Option<TResponse>, String>>)) -> () where
    TActor: PersistenceActor<TCommand, TEvent, TState, TResponse>,
    TStorage: storage::Storage<TState, TEvent>,
    TResponse: Serialize + Deserialize<'de>,
{
    let result = handle_command::<TActor, TStorage, TCommand, TEvent, TState, TResponse>(&context.0, context.1);

    context.2.send(result);
}

fn handle_command<TActor, TStorage, TCommand, TEvent, TState, TResponse>(name: &String, command: TCommand) -> Result<Option<TResponse>, String> where
    TActor: PersistenceActor<TCommand, TEvent, TState, TResponse>,
    TStorage: storage::Storage<TState, TEvent>,
{
    let state = match TStorage::get_state(name) {
        storage::GetStateResult::Ok(s) => s,
        storage::GetStateResult::StateNotFound => {
            let events = TStorage::get_events(name)?;

            events.iter().fold(None, |acc, e| {
                (TActor::handle_event)(acc, e)
            })
        },
        storage::GetStateResult::Err(e) => return Err(e)
    };

    let result = (TActor::handle_command)(&state, command)?;

    let next_state = result.events.iter().fold(state, |acc, e| {
        (TActor::handle_event)(acc, e)
    });

    TStorage::persist_events_and_state(name, result.events, next_state)?;

    Ok(result.response)
}
