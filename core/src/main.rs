/* ~~/libri_wasm/src/main.rs */

// third-party packages
use leptos::*;
use wasm_bindgen::prelude::*;

// local modules
mod settings;
mod speed_reader;
use speed_reader::SpeedReader;

fn main() {
  use wasm_bindgen::JsCast;
  
  // Try to get shadow root first, fallback to document
  let root_element = if let Some(shadow_root) = get_shadow_root() {
    shadow_root
      .query_selector("#libri-speed-reader-root")
      .ok()
      .flatten()
      .expect("libri-speed-reader-root not found in shadow DOM")
      .dyn_into::<web_sys::HtmlElement>()
      .expect("Failed to cast to HtmlElement")
  } else {
    document()
      .get_element_by_id("libri-speed-reader-root")
      .expect("libri-speed-reader-root element not found")
      .dyn_into::<web_sys::HtmlElement>()
      .expect("Failed to cast to HtmlElement")
  };
  
  // Mount the Leptos app
  mount_to(root_element, App);
}

// Helper function to get shadow root from global window object
fn get_shadow_root() -> Option<web_sys::ShadowRoot> {
  use wasm_bindgen::JsCast;
  
  let window = web_sys::window()?;
  let shadow_root = js_sys::Reflect::get(
    &window,
    &wasm_bindgen::JsValue::from_str("__LIBRI_SHADOW_ROOT__")
  ).ok()?;
  
  shadow_root.dyn_into::<web_sys::ShadowRoot>().ok()
}

#[component]
fn App() -> impl IntoView {
  let text = create_rw_signal::<Option<String>>(None);
  let is_visible = create_rw_signal(false);

  // Listen for messages from content script
  let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
    if let Ok(data) = serde_wasm_bindgen::from_value::<serde_json::Value>(event.data()) {
      if let Some(msg_type) = data.get("type").and_then(|v| v.as_str()) {
        match msg_type {
          "LIBRI_TEXT" => {
            if let Some(text_content) = data.get("text").and_then(|v| v.as_str()) {
              text.set(Some(text_content.to_string()));
              is_visible.set(true);
            }
          }
          _ => {}
        }
      }
    }
  }) as Box<dyn FnMut(_)>);

  window()
    .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
    .unwrap();
  closure.forget();

  view! {
    <Show
      when=move || is_visible.get()
      fallback=|| view! { <div style="display: none;"></div> }
    >
      <SpeedReader text=text is_visible=is_visible />
    </Show>
  }
}
