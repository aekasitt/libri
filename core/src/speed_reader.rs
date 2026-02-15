/* ~~/core/src/speed_reader.rs */

// third-party crates
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

// local crates
use crate::components::ui::card::{Card, CardContent, CardFooter};
use crate::components::ui::kbd::Kbd;
use crate::components::ui::shimmer::Shimmer;
use crate::settings::Settings;

fn text_to_words(text: &str) -> Vec<String> {
  text.split_whitespace().map(|s| s.to_string()).collect()
}

#[component]
pub fn SpeedReader(text: RwSignal<Option<String>>, is_visible: RwSignal<bool>) -> impl IntoView {
  let settings = create_rw_signal(Settings::default());
  let words =
    create_memo(move |_| text.with(|t| t.as_ref().map(|t| text_to_words(t)).unwrap_or_default()));

  let current_index = create_rw_signal(0_usize);
  let is_paused = create_rw_signal(false);
  let wpm = create_rw_signal(settings.get_untracked().initial_speed);

  let current_word = create_memo(move |_| {
    let idx = current_index.get();
    words.with(|w| w.get(idx).cloned().unwrap_or_default())
  });

  // Loading signal for shimmer (true when no word is displayed)
  let is_loading = create_memo(move |_| current_word.get().is_empty());

  let remaining_time = create_memo(move |_| {
    let idx = current_index.get();
    let total = words.with(|w| w.len());
    if total == 0 {
      return "0:00".to_string();
    }
    let remaining = total.saturating_sub(idx);
    let interval = 60.0 * 1000.0 / wpm.get() as f64;
    let seconds = (remaining as f64 * interval) / 1000.0;
    let minutes = (seconds / 60.0).floor() as i32;
    let secs = (seconds % 60.0).ceil() as i32;
    format!("{}:{:02}", minutes, secs)
  });

  let split_word = create_memo(move |_| {
    let word = current_word.get();
    if word.is_empty() {
      return ("".to_string(), "".to_string(), "".to_string());
    }

    let mut middle_index = word.len() / 2;

    // Adjust for punctuation at end
    if word.chars().last().map_or(false, |c| !c.is_alphanumeric()) {
      middle_index = middle_index.saturating_sub(1);
    }

    if word.chars().nth(middle_index) == Some(' ') {
      middle_index = middle_index.saturating_sub(1);
    }

    let start = word.chars().take(middle_index).collect::<String>();
    let middle = word.chars().nth(middle_index).unwrap_or(' ').to_string();
    let end = word.chars().skip(middle_index + 1).collect::<String>();

    (start, middle, end)
  });

  // Animation loop using set_interval
  let interval_handle = create_rw_signal::<Option<IntervalHandle>>(None);

  // Effect to manage the interval based on pause/visibility state
  create_effect(move |_| {
    let paused = is_paused.get();
    let visible = is_visible.get();
    let current_wpm = wpm.get();

    // Clear existing interval if any
    if let Some(handle) = interval_handle.get_untracked() {
      handle.clear();
      interval_handle.set(None);
    }

    // Start new interval if not paused and visible
    if !paused && visible {
      let interval_ms = (60.0 * 1000.0 / current_wpm as f64) as i32;

      let handle = set_interval_with_handle(
        move || {
          let idx = current_index.get_untracked();
          let total = words.with_untracked(|w| w.len());

          if idx < total - 1 {
            current_index.update(|i| *i += 1);
          } else {
            is_paused.set(true);
            current_index.set(0);
            is_visible.set(false);
          }
        },
        std::time::Duration::from_millis(interval_ms as u64),
      )
      .expect("Failed to create interval");

      interval_handle.set(Some(handle));
    }
  });

  let speed_up = move |_: MouseEvent| {
    let increment = settings.get_untracked().speed_increment;
    wpm.update(|w| *w += increment);
  };

  let speed_down = move |_: MouseEvent| {
    let increment = settings.get_untracked().speed_increment;
    wpm.update(|w| *w = (*w - increment).max(50));
  };

  // Keyboard event handler
  let on_keydown = move |keyboard_event: KeyboardEvent| match keyboard_event.key().as_str() {
    " " => {
      keyboard_event.prevent_default();
      is_paused.update(|p| *p = !*p);
    }
    "Escape" => {
      keyboard_event.prevent_default();
      is_paused.set(true);
      current_index.set(0);
      is_visible.set(false);
    }
    "ArrowUp" => {
      keyboard_event.prevent_default();
      let increment = settings.get_untracked().speed_increment;
      wpm.update(|w| *w += increment);
    }
    "ArrowDown" => {
      keyboard_event.prevent_default();
      let increment = settings.get_untracked().speed_increment;
      wpm.update(|w| *w = (*w - increment).max(50));
    }
    "ArrowLeft" => {
      keyboard_event.prevent_default();
      current_index.update(|i| *i = i.saturating_sub(1));
    }
    "ArrowRight" => {
      keyboard_event.prevent_default();
      let total = words.with(|w| w.len());
      current_index.update(|i| *i = (*i + 1).min(total.saturating_sub(1)));
    }
    _ => {}
  };

  let on_background_click = move |_mouse_event: MouseEvent| {
    is_paused.set(true);
    current_index.set(0);
    is_visible.set(false);
  };

  // Effect to steal focus when speed reader becomes visible so keyboard events work
  create_effect(move |_| {
    if is_visible.get() {
      let _ = set_timeout(
        move || {
          if let Some(window) = web_sys::window() {
            if let Ok(shadow_root) = js_sys::Reflect::get(
              &window,
              &wasm_bindgen::JsValue::from_str("__LIBRI_SHADOW_ROOT__"),
            ) {
              if let Some(shadow_root) = shadow_root.dyn_into::<web_sys::ShadowRoot>().ok() {
                if let Ok(Some(element)) = shadow_root.query_selector("#libri-container") {
                  let _ = element
                    .dyn_into::<web_sys::HtmlElement>()
                    .map(|html_el| html_el.focus());
                }
              }
            }
          }
        },
        std::time::Duration::from_millis(10),
      );
    }
  });

  view! {
    <div
      id="libri-container"
      class="fixed inset-0 z-[999999] flex items-center justify-center bg-black/50"
      on:click=on_background_click
      on:keydown=on_keydown
      tabindex="0"
    >
      <Card
        class="w-[90%] max-w-2xl bg-stone-500"
        on:click=move |mouse_event: MouseEvent| mouse_event.stop_propagation()
        >
        <CardContent class="p-8">
          <Shimmer loading=Signal::derive(move || is_loading.get())>
            <div class="flex items-center justify-center text-4xl font-mono min-h-20">
              <span class="flex-1 text-right">{move || split_word.get().0}</span>
              <span class="text-orange-500 font-bold">{move || split_word.get().1}</span>
              <span class="flex-1 text-left">{move || split_word.get().2}</span>
            </div>
          </Shimmer>
        </CardContent>

        <CardFooter class="flex items-center justify-between border-t p-4">
          <div class="flex items-center gap-2">
            <button on:click=speed_down class="px-3 py-1 hover:bg-gray-100 rounded">"-"</button>
            <span class="font-semibold">{move || wpm.get()}</span>
            <span class="text-sm text-gray-500">"WPM"</span>
            <button on:click=speed_up class="px-3 py-1 hover:bg-gray-100 rounded">"+"</button>
          </div>

          <div class="flex items-center gap-4">
            <span class="text-sm text-gray-600">{move || remaining_time.get()}</span>
            <div class="flex gap-1">
              <Kbd>"Space"</Kbd>
              <span class="text-xs text-gray-500">"Pause"</span>
            </div>
          </div>
        </CardFooter>
      </Card>
    </div>
  }
}
