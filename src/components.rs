use leptos::prelude::*;

use crate::{NavmenuState, NavmenuStateAction, NavmenuStateContext};

#[component]
pub fn NavmenuStateContextProvider(children: Children) -> impl IntoView {
    let navmenu_state_context = ArcRwSignal::<NavmenuState>::default();

    provide_context(navmenu_state_context);

    view! {
        {children()}
    }
}

#[component]
pub fn NavmenuButton(children: Children) -> impl IntoView {
    let navmenu_state_context =
        use_context::<NavmenuStateContext>().expect("no navmenu state context found");

    let on_click = move |_| {
        navmenu_state_context.update(|value| *value = value.reduce(NavmenuStateAction::Toggle))
    };

    view! {
        <button
            on:click=on_click
        >
            {children()}
        </button>
    }
}
