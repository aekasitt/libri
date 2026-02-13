/* ~~/core/src/speed_reader.rs */

// third-party crates
use leptos::leptos_dom::helpers::IntervalHandle;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

// local crates
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
  let on_keydown = move |ev: KeyboardEvent| match ev.key().as_str() {
    " " => {
      ev.prevent_default();
      is_paused.update(|p| *p = !*p);
    }
    "Escape" => {
      ev.prevent_default();
      is_paused.set(true);
      current_index.set(0);
      is_visible.set(false);
    }
    "ArrowUp" => {
      ev.prevent_default();
      let increment = settings.get_untracked().speed_increment;
      wpm.update(|w| *w += increment);
    }
    "ArrowDown" => {
      ev.prevent_default();
      let increment = settings.get_untracked().speed_increment;
      wpm.update(|w| *w = (*w - increment).max(50));
    }
    "ArrowLeft" => {
      ev.prevent_default();
      current_index.update(|i| *i = i.saturating_sub(1));
    }
    "ArrowRight" => {
      ev.prevent_default();
      let total = words.with(|w| w.len());
      current_index.update(|i| *i = (*i + 1).min(total.saturating_sub(1)));
    }
    _ => {}
  };

  let on_background_click = move |ev: MouseEvent| {
    if let Some(target) = ev.target() {
      if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
        if element.id() == "libri-container" {
          is_paused.set(true);
          current_index.set(0);
          is_visible.set(false);
        }
      }
    }
  };

  let container_style = move || {
    let s = settings.get();
    format!(
      "--bg-color: {}; --text-color: {}; --middle-letter-color: {}; --font-family: {}; --font-size: {};",
      s.background_color, s.text_color, s.middle_letter_color, s.font_family, s.font_size
    )
  };

  let wrapper_style = move || {
    let s = settings.get();
    format!(
      "width: {}; height: {};",
      if s.full_screen { "100%" } else { &s.width },
      if s.full_screen { "100%" } else { &s.height }
    )
  };

  view! {
    <div
      id="libri-container"
      style=container_style
      on:click=on_background_click
      on:keydown=on_keydown
      tabindex="0"
    >
      <div class="libri-wrapper" style=wrapper_style>
        <div class="libri-word-container">
          <div class="libri-word-start">{move || split_word.get().0}</div>
          <div class="libri-word-middle">{move || split_word.get().1}</div>
          <div class="libri-word-end">{move || split_word.get().2}</div>
        </div>
        <div class="libri-controls">
          <div class="libri-speed">
            <span class="libri-speed-minus" on:click=speed_down>"-"</span>
            <span>{move || wpm.get()}</span>
            <span class="libri-speed-plus" on:click=speed_up>"+"</span>
          </div>
          <div class="lectio-time">{move || remaining_time.get()}</div>
        </div>
      </div>
      <style>
        {r#"
#libri-container {
  position: fixed;
  z-index: 999999;
  top: 0;
  left: 0;
  height: 100%;
  width: 100%;
  display: flex;
  flex-flow: column nowrap;
  justify-content: center;
  align-items: center;
  background: rgba(128, 128, 128, 0.5);
  color: var(--text-color);
  font-family: var(--font-family);
  font-size: var(--font-size);
}

.libri-wrapper {
  padding: 10px;
  position: relative;
  background: var(--bg-color);
}

.libri-word-container {
  display: flex;
  align-items: center;
  margin: 20px 0;
}

.libri-word-start {
  flex: 1;
  text-align: right;
}

.libri-word-end {
  flex: 1;
  text-align: left;
}

.libri-word-middle {
  flex: 0;
  color: var(--middle-letter-color);
}

.libri-controls {
  font-size: 12px;
  display: flex;
}

.libri-speed {
  flex: 1;
}

.libri-speed-plus,
.libri-speed-minus {
  cursor: pointer;
  user-select: none;
  display: inline-block;
  width: 15px;
  text-align: center;
}

.libri-time {
  text-align: right;
}
        "#}
      </style>
    </div>
  }
}
