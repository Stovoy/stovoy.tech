#![cfg(target_arch = "wasm32")]

use gloo_timers::callback::{Interval, Timeout};
use yew::prelude::*;

const COMMAND: &str = "stovoy@devbox ~ $ whoami";

struct CommandSegment<'a> {
    text: &'a str,
    class: &'a str,
}

fn command_segments() -> [CommandSegment<'static>; 5] {
    [
        CommandSegment { text: "stovoy", class: "cmd-user" },
        CommandSegment { text: "@devbox", class: "cmd-host" },
        CommandSegment { text: " ~ ", class: "cmd-path" },
        CommandSegment { text: "$ ", class: "cmd-prompt" },
        CommandSegment { text: "whoami", class: "cmd-cmd" },
    ]
}

#[function_component(Home)]
pub fn home() -> Html {
    let typed = use_state(|| String::new());
    let cursor_visible = use_state(|| true);
    let show_rest = use_state(|| false);

    {
        let typed = typed.clone();
        let show_rest = show_rest.clone();
        use_effect_with((), move |_| {
            for (i, _) in COMMAND.chars().enumerate() {
                let typed = typed.clone();
                let show_rest = show_rest.clone();
                Timeout::new((i as u32) * 80, move || {
                    typed.set(COMMAND.chars().take(i + 1).collect::<String>());
                    if i + 1 == COMMAND.len() {
                        Timeout::new(200, move || show_rest.set(true)).forget();
                    }
                })
                .forget();
            }
            || ()
        });
    }

    {
        let cursor_visible = cursor_visible.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(400, move || {
                cursor_visible.set(!*cursor_visible);
            });
            move || drop(interval)
        });
    }

    let rest_content = html! {
        <>
            <p>{"Hi. I'm Steve, but you can call me Stovoy - 20+ years code wizard, creator of Evades.io, and Safety Engineer @ OpenAI"}</p>

            <div class="fake-command-line">
                <span class="cmd-user">{"stovoy"}</span><span class="cmd-host">{"@devbox"}</span><span class="cmd-path">{" ~ "}</span><span class="cmd-prompt">{"$ "}</span><span class="cmd-cmd">{"cat interests.md"}</span>
            </div>
            <ul>
                <li>{"Rust | Godot | Optimizations"}</li>
                <li>{"Finnley (my dog) | Video Games | Building Cool Things | Sci-Fi"}</li>
                <li>{"Automate. Everything."}</li>
            </ul>

            <div class="fake-command-line">
                <span class="cmd-user">{"stovoy"}</span><span class="cmd-host">{"@devbox"}</span><span class="cmd-path">{" ~ "}</span><span class="cmd-prompt">{"$ "}</span><span class="cmd-cmd">{"ls projects"}</span>
            </div>
            <ul class="links">
                <li><a href="https://evades.io">{"Evades.io"}</a></li>
                <li><a href="/game/snake" target="_blank">{"Snake"}</a></li>
            </ul>

            <div class="fake-command-line">
                <span class="cmd-user">{"stovoy"}</span><span class="cmd-host">{"@devbox"}</span><span class="cmd-path">{" ~ "}</span><span class="cmd-prompt">{"$ "}</span><span class="cmd-cmd">{"contact"}</span>
            </div>
            <ul class="links">
                <li><a class="github" href="https://github.com/stovoy" target="_blank">{"GitHub"}</a></li>
                <li><a class="twitch" href="https://twitch.tv/stovoy" target="_blank">{"Twitch"}</a></li>
            </ul>

            <p>
                {"This terminal styled with the "}
                <a href="https://github.com/catppuccin" target="_blank">{"Catppuccin theme"}</a>
            </p>
        </>
    };

    let mut chars_typed = 0;
    let mut html_segments = vec![];
    let typed_str = &*typed;
    for seg in command_segments() {
        let seg_len = seg.text.len();
        let end = (chars_typed + seg_len).min(typed_str.len());
        if chars_typed < typed_str.len() {
            let seg_typed = &typed_str[chars_typed..end];
            if !seg_typed.is_empty() {
                html_segments.push(html! { <span class={seg.class}>{seg_typed}</span> });
            }
        }
        chars_typed += seg_len;
    }

    html! {
        <div class="terminal">
            { html! { for html_segments } }
            { if !*show_rest && *cursor_visible { html! { <span class="cursor">{"|"}</span> } } else { html! {} } }
            { if *show_rest { rest_content } else { html! {} } }
        </div>
    }
}
