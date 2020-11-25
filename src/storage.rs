pub enum GetStateResult<T> {
    Ok(Option<T>),
    StateNotFound,
    Err(String)
}

pub trait Storage<TState, TEvent> {
    fn get_state(name: &String) -> GetStateResult<TState>;

    fn persist_events_and_state(name: &String, events: Vec<TEvent>, state: Option<TState>) -> Result<(), String>;

    fn get_events(name: &String) -> Result<Vec<TEvent>, String>;
}