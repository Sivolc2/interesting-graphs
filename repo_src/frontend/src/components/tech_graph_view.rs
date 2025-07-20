use leptos::*;
use serde::{Deserialize, Serialize};


#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = render_cytoscape_graph)]
    fn render_cytoscape_graph(container_id: &str, elements: JsValue);
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CyNodeData {
    pub id: String,
    pub label: String,
    pub group: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CyNode {
    pub data: CyNodeData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CyEdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CyEdge {
    pub data: CyEdgeData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Elements {
    pub nodes: Vec<CyNode>,
    pub edges: Vec<CyEdge>,
}

impl Elements {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty() && self.edges.is_empty()
    }
}

#[component]
pub fn TechGraphView(
    elements: Signal<Option<Elements>>,
) -> impl IntoView {
    let graph_container_ref = create_node_ref::<html::Div>();

    // This effect will re-run whenever the `elements` signal changes.
    create_effect(move |_| {
        if let Some(cy_elements) = elements.get() {
            if cy_elements.nodes.is_empty() {
                // Don't render if there are no nodes to avoid JS errors
                return;
            }
            if let Some(div) = graph_container_ref.get() {
                div.set_id("cytoscape-container");

                #[cfg(feature = "hydrate")]
                {
                    match serde_wasm_bindgen::to_value(&cy_elements) {
                        Ok(elements_js) => {
                            // Add a small delay to ensure DOM and scripts are ready
                            wasm_bindgen_futures::spawn_local(async move {
                                gloo_timers::future::TimeoutFuture::new(100).await;
                                render_cytoscape_graph("cytoscape-container", elements_js);
                            });
                        }
                        Err(e) => logging::error!("Failed to serialize elements: {:?}", e),
                    }
                }
            }
        }
    });

    view! {
        <div class="graph-view-wrapper">
            {move || match elements.get() {
                Some(e) if !e.nodes.is_empty() => view! {
                    <div class="graph-container" _ref=graph_container_ref></div>
                }.into_view(),
                _ => view! {
                    <div class="graph-placeholder"><p>"Select a filter to display the technology graph."</p></div>
                }.into_view(),
            }}
        </div>
    }
} 