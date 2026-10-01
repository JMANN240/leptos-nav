use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::{NavmenuStateAction, NavmenuStateContext};

pub fn use_hide_navmenu_on_navigation() {
    let navmenu_state_context =
        use_context::<NavmenuStateContext>().expect("no navmenu state found");

    let location = use_location();

    Effect::new(move |_| {
        let _ = location.pathname.get();

        navmenu_state_context.update(|value| *value = value.reduce(NavmenuStateAction::Close));
    });
}
