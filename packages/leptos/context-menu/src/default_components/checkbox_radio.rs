//! ContextMenu checkbox and radio components
//! 
//! This module contains the ContextMenuCheckboxItem, ContextMenuRadioGroup,
//! and ContextMenuRadioItem components for interactive menu items.

use leptos::prelude::*;
use leptos_style::Style;
use web_sys::MouseEvent;

#[component]
pub fn ContextMenuCheckboxItem(
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();

    let handle_click = move |e: MouseEvent| {
        if !disabled.get().unwrap_or(false) {
            e.stop_propagation();
            let new_checked = !checked.get();
            checked.set(new_checked);
            if let Some(callback) = &on_checked_change {
                callback.run(new_checked);
            }
            open.set(false);
        }
    };

    let item_class = move || {
        let base_class = "context-menu-checkbox-item";
        let checked_class = if checked.get() { " checked" } else { "" };
        let disabled_class = if disabled.get().unwrap_or(false) { " disabled" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{}{} {}", base_class, checked_class, disabled_class, custom_class)
    };

    view! {
        <div
            class=item_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
            role="menuitemcheckbox"
            aria-checked=checked.get()
            aria-disabled=disabled.get().unwrap_or(false)
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuRadioGroup(
    #[prop(into)] value: RwSignal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(value);
    provide_context(on_value_change);

    view! {
        <div
            class=move || format!("context-menu-radio-group {}", class.get().unwrap_or_default())
            role="radiogroup"
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuRadioItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let group_value = expect_context::<RwSignal<String>>();
    let on_value_change = expect_context::<Option<Callback<String>>>();

    let value_clone = value.clone();
    let is_selected = Signal::derive(move || group_value.get() == value_clone);

    let handle_click = move |e: MouseEvent| {
        if !disabled.get().unwrap_or(false) {
            e.stop_propagation();
            group_value.set(value.clone());
            if let Some(callback) = &on_value_change {
                callback.run(value.clone());
            }
            open.set(false);
        }
    };

    let item_class = move || {
        let base_class = "context-menu-radio-item";
        let selected_class = if is_selected.get() { " selected" } else { "" };
        let disabled_class = if disabled.get().unwrap_or(false) { " disabled" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{}{} {}", base_class, selected_class, disabled_class, custom_class)
    };

    view! {
        <div
            class=item_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:click=handle_click
            role="menuitemradio"
            aria-checked=is_selected.get()
            aria-disabled=disabled.get().unwrap_or(false)
        >
            {children.map(|c| c())}
        </div>
    }
}
