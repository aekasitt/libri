/* ~~/core/src/components/ui/shimmer.rs */

// third-party crates
use leptos::*;
use tw_merge::*;

// local crates
use crate::components::hooks::use_random::use_random_id_for;

#[component]
pub fn Shimmer(
  #[prop(into)] loading: Signal<bool>,
  #[prop(into, optional)] shimmer_color: Option<String>,
  #[prop(into, optional)] background_color: Option<String>,
  #[prop(optional)] duration: Option<f64>,
  #[prop(optional)] fallback_border_radius: Option<f64>,
  #[prop(into, optional)] class: String,
  children: Children,
) -> impl IntoView {
  let shimmer_id = use_random_id_for("Shimmer");
  let merged_class = tw_merge!("relative", class);

  view! {
    <div
      id=shimmer_id
      class=merged_class
      data-name="Shimmer"
      data-shimmer-loading=move || loading.get().to_string()
      data-shimmer-color=shimmer_color
      data-shimmer-bg-color=background_color
      data-shimmer-duration=duration.map(|d| d.to_string())
      data-shimmer-fallback-radius=fallback_border_radius.map(|r| r.to_string())
    >
      {children()}
    </div>
  }
}
