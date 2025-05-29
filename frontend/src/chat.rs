#![cfg(target_arch = "wasm32")]

use gloo_net::websocket::{futures::WebSocket, Message};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use futures_util::{StreamExt, SinkExt};

#[function_component(Chat)]
pub fn chat() -> Html {
    let messages = use_state(|| Vec::<String>::new());
    let input = use_node_ref();

    {
        let messages = messages.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().expect("no window");
            let location = window.location();
            let protocol = if location.protocol().unwrap_or_default().starts_with("https") {
                "wss"
            } else {
                "ws"
            };
            let host = location.host().unwrap_or_default();
            let url = format!("{protocol}://{host}/api/game/arena");
            let ws = WebSocket::open(&url).expect("failed to open ws");

            let (write, read) = ws.split();

            wasm_bindgen_futures::spawn_local(read.for_each(move |event| {
                let messages = messages.clone();
                async move {
                    if let Ok(Message::Text(text)) = event {
                        let mut v = (*messages).clone();
                        v.push(text);
                        messages.set(v);
                    }
                }
            }));

            move || drop(write)
        });
    }

    let on_submit = {
        let input = input.clone();
        let messages = messages.clone();
        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            let input_elem = input.cast::<HtmlInputElement>().unwrap();
            let text = input_elem.value();
            if text.is_empty() {
                return;
            }
            input_elem.set_value("");

            let window = web_sys::window().unwrap();
            let location = window.location();
            let protocol = if location.protocol().unwrap_or_default().starts_with("https") {
                "wss"
            } else {
                "ws"
            };
            let host = location.host().unwrap_or_default();
            let url = format!("{protocol}://{host}/api/game/arena");
            let mut v = (*messages).clone();
            v.push(text.clone());
            messages.set(v);

            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(mut ws) = WebSocket::open(&url) {
                    let _ = ws.send(Message::Text(text)).await;
                }
            });
        })
    };

    html! {
        <div class="flex flex-col gap-4">
            <ul class="border border-gray-300 dark:border-gray-700 rounded p-4 h-64 overflow-y-auto bg-white/10">
                { for messages.iter().map(|msg| html! { <li>{msg}</li> }) }
            </ul>
            <form onsubmit={on_submit} class="flex gap-2">
                <input ref={input} type="text" class="flex-grow border rounded px-3 py-2 bg-gray-100 dark:bg-gray-800" />
                <button type="submit" class="px-4 py-2 bg-primary text-white rounded">{"Send"}</button>
            </form>
        </div>
    }
}
