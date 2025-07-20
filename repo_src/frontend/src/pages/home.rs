use leptos::*;
use leptos_meta::*;
use crate::components::item_form::ItemForm;
use crate::components::item_list::ItemList;
// Ensure server function structs are directly usable.
// The `#[server]` macro makes them available.
use crate::server_fns::{AddItem, DeleteItem};

#[component]
pub fn HomePage() -> impl IntoView {
    // Action to add an item
    let add_item_action = create_server_action::<AddItem>();

    // Action to delete an item
    let delete_item_action = create_server_action::<DeleteItem>();

    // Resource to fetch items. It refetches whenever an item is added or deleted.
    let items_resource = create_resource(
        move || (add_item_action.version().get(), delete_item_action.version().get()),
        |_| async move {
            // GetItems server function call
            match crate::server_fns::get_items().await {
                Ok(items) => items,
                Err(e) => {
                    leptos::logging::error!("Failed to fetch items: {:?}", e);
                    Vec::new() // Return empty vec on error to avoid breaking UI
                }
            }
        }
    );
    
    view! {
        <Title text="Item Management App"/>
        
        <h1>"Item Management"</h1>
        
        <div class="card">
            <h2>"Add New Item"</h2>
            <ItemForm add_item_action=add_item_action />
        </div>

        <div class="card">
            <h2>"Current Items"</h2>
            <Suspense fallback=move || view! { <p class="loading-indicator">"Loading items..."</p> }>
                <ErrorBoundary fallback = |_| view!{<p>"Error loading items"</p>}>
                    {move || items_resource.map(|items| view! { <ItemList items=items.clone() delete_item_action=delete_item_action /> })}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
} 