//! Simple chat component that connects to `/api/game/arena` WebSocket endpoint.

#![cfg(target_arch = "wasm32")]

use gloo::net::websocket::{futures::WebSocket, Message, WebSocketError};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use futures_util::{SinkExt, StreamExt};

#[function_component(Chat)]
pub fn chat() -> Html {
    let messages = use_state(|| Vec::<String>::new());
    let input_value = use_state(|| String::new());
    let ws_ref = use_mut_ref(|| None::<WebSocket>);

    // Establish the websocket connection once on mount.
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                // Build ws://{host}/api/game/arena url, respecting protocol.
                let window = web_sys::window().expect("no window");
                let location = window.location();
                let hostname = location.host().unwrap_or_default();
                let protocol_prefix = if location.protocol().unwrap_or_default() == "https:" {
                    "wss://"
                } else {
                    "ws://"
                };
                let url = format!("{}{}{}", protocol_prefix, hostname, "/api/game/arena");

                if let Ok(ws) = WebSocket::open(&url) {
                    let (mut write, mut read) = ws.split();
                    *ws_ref.borrow_mut() = Some(write.reunite(read).unwrap());

                    // Receive task
                    spawn_local(async move {
                        let mut read = read;
                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(txt)) => messages.set({
                                    let mut vec = (*messages).clone();
                                    vec.push(txt);
                                    vec
                                }),
                                _ => {}
                            }
                        }
                    });
                }

                || {}
            },
            (),
        );
    }

    // Handler for sending message.
    let onsubmit = {
        let input_value = input_value.clone();
        let messages = messages.clone();
        let ws_ref = ws_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let msg = (*input_value).clone();
            if msg.is_empty() {
                return;
            }
            // Push to local list immediately for optimistic UI.
            messages.set({
                let mut vec = (*messages).clone();
                vec.push(format!("You: {}", msg));
                vec
            });

            // Send over websocket.
            if let Some(ws) = &*ws_ref.borrow() {
                let mut ws_clone = ws.clone();
                spawn_local(async move {
                    let _ = ws_clone.send(Message::Text(msg)).await;
                });
            }

            input_value.set(String::new());
        })
    };

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(inp) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_value.set(inp.value());
            }
        })
    };

    html! {
        <div class="max-w-xl mx-auto p-4">
            <h2 class="text-xl font-semibold mb-2">{"Arena Chat"}</h2>
            <div class="border rounded h-64 overflow-y-auto p-2 bg-gray-50">
                {
                    for (*messages).iter().map(|m| html!{ <div class="text-sm mb-1"> {m} </div> })
                }
            </div>

            <form class="mt-2 flex" onsubmit={onsubmit}>
                <input
                    class="flex-grow border rounded-l px-2 py-1"
                    placeholder="Type a message..."
                    value={(*input_value).clone()}
                    {oninput}
                />
                <button type="submit" class="bg-blue-600 text-white px-3 py-1 rounded-r">{"Send"}</button>
            </form>
        </div>
    }
}
