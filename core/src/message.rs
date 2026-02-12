/* ~~/libri_wasm/src/message.rs */

// standard crates
use std::{collections::HashSet, num::NonZeroU64};

// third-party crates
use chrome_wasm_bindgen::*;
use leptos::*;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};

// local crates
use crate::{
  SelectedComponentId,
  utils::{merge_component, remove_all, remove_component_children},
};
use libri::api::{ComponentChildrenRemove, Event, Message};

pub(crate) fn chrome() -> Option<Chrome> {
  let obj = window().get("chrome")?;
  Some(obj.unchecked_into())
}

const LIBRI_DICTATION_PANEL: &str = "LIBRI_DICTATION_PANEL";

pub(crate) fn on_message(message_component_update: RwSignal<bool>) {
  let selected_component_id = expect_context::<RwSignal<Option<SelectedComponentId>>>();
  let expand_component = expect_context::<RwSignal<HashSet<NonZeroU64>>>();
  let port = chrome().unwrap().runtime().connect_with_connect_info(
    ConnectInfo {
      name: Some(LIBRI_DICTATION_PANEL),
    }
    .into(),
  );

  let on_message = Closure::<dyn FnMut(JsValue)>::new(move |message: JsValue| {
    let Message { payload, .. } = Message::from(message);
    let mut component_update = false;
    for event in payload {
      match event {
        Event::Component(comp) => {
          merge_component(comp);
          component_update = true;
        }
        Event::ComponentChildrenRemove(ComponentChildrenRemove { id, deep }) => {
          remove_component_children(&id, deep);
          component_update = true;
        }
        Event::OpenDevtoolsPanel => {}
        Event::PageUnload => {
          remove_all();
          selected_component_id.set(None);
          expand_component.set(HashSet::new());
          component_update = true;
        }
      }
    }
    if component_update {
      message_component_update.set(true);
    }
  })
  .into_js_value();
  port.on_message().add_listener(&on_message.unchecked_ref());
}
