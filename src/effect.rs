pub struct Effect<TEvent, TResponse> {
    pub events: Vec<TEvent>,
    pub response: Option<TResponse>,
}

impl<TEvent, TResponse> Effect<TEvent, TResponse> {
    pub fn publish(event: TEvent) -> Self {
        Effect {
            events: vec!(event),
            response: None
        }
    }

    pub fn then(self, side_effect: fn() -> ()) -> Self {
        side_effect();

        self
    }

    pub fn then_reply(self, response: TResponse) -> Self {
        Effect {
            events: self.events,
            response: Some(response)
        }
    }
}
