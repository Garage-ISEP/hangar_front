use gloo_net::eventsource::futures::EventSource;
use yew::prelude::*;

use crate::services::sse_service::{SseEvent, connect_to_creation, connect_to_project};

const MAX_EVENTS: usize = 100;

#[derive(Clone, PartialEq)]
pub struct SseState
{
    pub events: Vec<SseEvent>,
    pub connected: bool,
    pub error: Option<String>,
}

impl Default for SseState
{
    fn default() -> Self
    {
        Self 
        {
            events: Vec::new(),
            connected: false,
            error: None,
        }
    }
}

#[hook]
pub fn use_sse_creation() -> SseState
{
    let state = use_state(SseState::default);
    let event_source = use_mut_ref(|| None::<EventSource>);

    {
        let state = state.clone();
        let event_source = event_source.clone();

        use_effect_with((), move |_| 
        {
            let state_clone = state.clone();

            let on_message = Callback::from(move |event: SseEvent| 
            {
                let mut current = (*state_clone).clone();
                current.events.push(event);

                if current.events.len() > MAX_EVENTS
                {
                    current.events.drain(0..(current.events.len() - MAX_EVENTS));
                }

                current.connected = true;
                current.error = None;
                state_clone.set(current);
            });

            let state_error = state.clone();
            let on_error = Callback::from(move |error: String| 
            {
                let mut current = (*state_error).clone();
                current.connected = false;
                current.error = Some(error);
                state_error.set(current);
            });

            match connect_to_creation(on_message, on_error)
            {
                Ok(es) =>
                {
                    *event_source.borrow_mut() = Some(es);
                    let mut current = (*state).clone();
                    current.connected = true;
                    current.error = None;
                    state.set(current);
                }
                Err(e) =>
                {
                    gloo_console::error!("Failed to connect to creation SSE:", &e);
                    let mut current = (*state).clone();
                    current.error = Some(e);
                    current.connected = false;
                    state.set(current);
                }
            }

            move || 
            {
                if let Some(es) = event_source.borrow_mut().take()
                {
                    es.close();
                }
            }
        });
    }

    (*state).clone()
}

#[hook]
pub fn use_sse_project(project_id: i32) -> SseState
{
    let state = use_state(SseState::default);
    let event_source = use_mut_ref(|| None::<EventSource>);

    {
        let state = state.clone();
        let event_source = event_source.clone();

        use_effect_with(project_id, move |&project_id| 
        {
            let state_clone = state.clone();

            let on_message = Callback::from(move |event: SseEvent| 
            {
                let mut current = (*state_clone).clone();

                if let Some(pos) = current
                    .events
                    .iter()
                    .position(|e| std::mem::discriminant(e) == std::mem::discriminant(&event))
                {
                    current.events[pos] = event;
                }
                else
                {
                    current.events.push(event);
                }

                current.connected = true;
                current.error = None;
                state_clone.set(current);
            });

            let state_error = state.clone();
            let on_error = Callback::from(move |error: String| 
            {
                let mut current = (*state_error).clone();
                current.connected = false;
                current.error = Some(error);
                state_error.set(current);
            });

            match connect_to_project(project_id, on_message, on_error)
            {
                Ok(es) =>
                {
                    *event_source.borrow_mut() = Some(es);
                    let mut current = (*state).clone();
                    current.connected = true;
                    current.error = None;
                    state.set(current);
                }
                Err(e) =>
                {
                    gloo_console::error!("Failed to connect to SSE:", &e);
                    let mut current = (*state).clone();
                    current.error = Some(e);
                    current.connected = false;
                    state.set(current);
                }
            }

            move || 
            {
                if let Some(es) = event_source.borrow_mut().take()
                {
                    es.close();
                }
            }
        });
    }

    (*state).clone()
}
