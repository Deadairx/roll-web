use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use roll_rs::roller::{roll_dice, RollResult};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (roll_total, set_roll_total) = create_signal(cx, 0);
    let (all_dice, set_all_dice) = create_signal(cx, vec![]);

    let on_roll_click = move |_| {
        let roll_result = roll_dice(2, 20, 'L', 0);

        set_all_dice(roll_result.individual_rolls);
        set_roll_total(roll_result.result_total);
    };

    view! { cx,
        <h1>"Roll for Iniative!"</h1>
        <For
            each=all_dice
            key=|_dice| 1
            view=move |cx, dice: u32| {
                view! {
                    cx,
                    <span>{dice} "  "</span>
                }
            }
        />
        <div>{roll_total}</div>
        <button on:click=on_roll_click>"Roll!"</button>
    }
}
