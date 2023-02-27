use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use roll_rs::roller::roll_dice;

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
    let (roll, set_roll) = create_signal(cx, 0);

    let on_roll_click = move |_| set_roll(roll_dice(1, 20, '_', 0).result_total);

    view! { cx,
        <h1>"Roll for Iniative!"</h1>
        <div>{roll}</div>
        <button on:click=on_roll_click>"Roll!"</button>
    }
}
