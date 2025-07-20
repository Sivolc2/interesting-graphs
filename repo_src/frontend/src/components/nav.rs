use leptos::*;
use leptos_router::A;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <header class="header">
            <div class="container nav-container">
                <A href="/" class="nav-logo">
                    <span role="img" aria-label="galaxy">"🌌"</span>
                    " Sci-Fi Tech-Verse"
                </A>
                <nav class="nav-links">
                    <A href="/" exact=true> "Item Manager (Home)" </A>
                    <A href="/tech-graph">"Tech Graph"</A>
                </nav>
            </div>
        </header>
    }
} 