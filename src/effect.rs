pub struct CommandEffect<TEvent, TResponse> {
    pub events: Vec<TEvent>,
    pub response: Option<TResponse>,
}

impl<TEvent, TResponse> CommandEffect<TEvent, TResponse> {
    pub fn publish(event: TEvent) -> Self {
        CommandEffect {
            events: vec!(event),
            response: None
        }
    }

    pub fn then(self, side_effect: fn() -> ()) -> Self {
        side_effect();

        self
    }

    pub fn then_reply(self, response: TResponse) -> Self {
        CommandEffect {
            events: self.events,
            response: Some(response)
        }
    }
}
