#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Storage};

const STORAGE_KEY: &str = "theme";

pub fn get_storage() -> Option<Storage> {
    window()?.local_storage().ok().flatten()
}

pub fn current_pref() -> bool {
    if let Some(storage) = get_storage() {
        if let Ok(Some(val)) = storage.get_item(STORAGE_KEY) {
            return val == "dark";
        }
    }
    window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|mq| mq.matches())
        .unwrap_or(false)
}

pub fn set_pref(dark: bool) {
    if let Some(storage) = get_storage() {
        let _ = storage.set_item(STORAGE_KEY, if dark { "dark" } else { "light" });
    }
    apply_class(dark);
}

pub fn apply_class(dark: bool) {
    if let Some(doc) = window().and_then(|w| w.document()) {
        let elem = doc
            .document_element()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        if dark {
            let _ = elem.class_list().add_1("dark");
        } else {
            let _ = elem.class_list().remove_1("dark");
        }
    }
}
