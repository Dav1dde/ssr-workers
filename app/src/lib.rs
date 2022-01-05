use sycamore::prelude::*;

#[cfg(feature = "ssr")]
pub fn render_to_string() -> String {
    sycamore::render_to_string(|| view! { App(None) })
}

#[component(App<G>)]
pub fn app(_pathname: Option<String>) -> View<G> {
    let counter = Signal::new(0);

    create_effect(cloned!((counter) => move || {
        log::info!("Counter value: {}", *counter.get());
    }));

    let increment = cloned!((counter) => move |_| counter.set(*counter.get() + 1));

    let reset = cloned!((counter) => move |_| counter.set(0));

    view! {
        div {
            h1 { "Counter demo" }
            p(class="value") {
                "Value: "
                (counter.get())
            }
            button(class="increment", on:click=increment) {
                "Increment"
            }
            button(class="reset", on:click=reset) {
                "Reset"
            }
        }
    }
}
