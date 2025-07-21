use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = renderTechGraph)]
    fn render_tech_graph(container_id: &str, nodes: JsValue, edges: JsValue);
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub label: String,
    #[serde(rename = "group")]
    pub group: String,
    pub title: String, // Tooltip
    pub shape: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

#[component]
pub fn TechGraphView(
    nodes: Signal<Vec<Node>>,
    edges: Signal<Vec<Edge>>,
) -> impl IntoView {
    let graph_container_ref = create_node_ref::<html::Div>();

    create_effect(move |_| {
        let current_nodes = nodes.get();
        let current_edges = edges.get();
        
        if current_nodes.is_empty() {
            return;
        }

        if let Some(div) = graph_container_ref.get() {
            div.set_id("tech-graph-container");

            #[cfg(feature = "hydrate")]
            {
                // Use serde_wasm_bindgen to convert Rust structs to JsValue
                match (
                    serde_wasm_bindgen::to_value(&current_nodes),
                    serde_wasm_bindgen::to_value(&current_edges),
                ) {
                    (Ok(nodes_js), Ok(edges_js)) => {
                        render_tech_graph("tech-graph-container", nodes_js, edges_js);
                    }
                    (Err(e), _) => logging::error!("Failed to serialize nodes: {:?}", e),
                    (_, Err(e)) => logging::error!("Failed to serialize edges: {:?}", e),
                }
            }
        }
    });

    view! {
        <div class="graph-view-wrapper">
            {move || if nodes.get().is_empty() {
                view! {
                    <div class="graph-placeholder">
                        <p>"Loading technology network data..."</p>
                    </div>
                }.into_view()
            } else {
                view! {
                     <div class="graph-container" _ref=graph_container_ref>
                        // The JS will render the graph here
                     </div>
                }.into_view()
            }}
        </div>
    }
} 