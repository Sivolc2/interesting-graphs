--- a/repo_src/frontend/index.html
+++ b/repo_src/frontend/index.html
@@ -5,9 +5,9 @@
     <link rel="preconnect" href="https://fonts.googleapis.com">
     <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
     <link href="https://fonts.googleapis.com/css2?family=Roboto+Mono:ital,wght@0,400;0,700;1,400&family=Roboto:wght@400;700&display=swap" rel="stylesheet">
-    <script type="text/javascript" src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
+    <script src="https://unpkg.com/cytoscape@3.23.0/dist/cytoscape.min.js"></script>
     <link rel="icon" href="data:;base64,iVBORw0KGgo="> <!-- Simple empty favicon -->
     <!-- {{ meta }} -->
   </head>
   <body>
     <!-- {{ body }} -->
   </body>
-  <script type="text/javascript" src="/js/graph_renderer.js"></script>
+  <script type="text/javascript" src="/js/cytoscape_renderer.js"></script>
 </html>
--- a/repo_src/frontend/public/js/graph_renderer.js
+++ /dev/null
@@ -1,78 +0,0 @@
-var network = null;
-
-function renderTechGraph(containerId, nodesJson, edgesJson) {
-    console.log("renderTechGraph called with:", containerId, "nodes:", nodesJson?.length, "edges:", edgesJson?.length);
-    var container = document.getElementById(containerId);
-    if (!container) {
-        console.error("Graph container not found:", containerId);
-        return;
-    }
-
-    // Destroy the old network instance if it exists
-    if (network !== null) {
-        network.destroy();
-        network = null;
-    }
-
-    // Create datasets.
-    var nodes = new vis.DataSet(nodesJson);
-    var edges = new vis.DataSet(edgesJson);
-
-    var data = {
-        nodes: nodes,
-        edges: edges,
-    };
-
-    var options = {
-        groups: {
-            Category: {
-                shape: "diamond",
-                size: 30,
-                color: {
-                    background: "#4a90e2",
-                    border: "#2171b5"
-                },
-                font: {
-                    size: 16,
-                    color: "#ffffff",
-                    strokeWidth: 2,
-                    strokeColor: "#000000"
-                },
-                borderWidth: 3,
-                shadow: true,
-            },
-            Technology: {
-                shape: "dot",
-                size: 20,
-                color: {
-                    background: "#7b68ee",
-                    border: "#5a4fcf"
-                },
-                font: {
-                    size: 12,
-                    color: "#ffffff",
-                    strokeWidth: 1,
-                    strokeColor: "#000000"
-                },
-                borderWidth: 2,
-                shadow: true,
-            },
-            TechnologyHighlighted: {
-                shape: "dot",
-                size: 25,
-                color: {
-                    background: "#ff6b6b",
-                    border: "#e55454"
-                },
-                font: {
-                    size: 13,
-                    color: "#ffffff",
-                    strokeWidth: 2,
-                    strokeColor: "#000000"
-                },
-                borderWidth: 3,
-                shadow: true,
-            },
-            Book: {
-                shape: "box",
-                size: 15,
-                color: {
-                    background: "#ffa726",
-                    border: "#ef6c00"
-                },
-                font: {
-                    size: 10,
-                    color: "#ffffff",
-                    strokeWidth: 1,
-                    strokeColor: "#000000"
-                },
-                borderWidth: 2,
-                shadow: true,
-            },
-            BookHighlighted: {
-                shape: "box",
-                size: 18,
-                color: {
-                    background: "#66bb6a",
-                    border: "#4caf50"
-                },
-                font: {
-                    size: 11,
-                    color: "#ffffff",
-                    strokeWidth: 2,
-                    strokeColor: "#000000"
-                },
-                borderWidth: 3,
-                shadow: true,
-            }
-        },
-        nodes: {
-            shape: "dot",
-            size: 16,
-            font: {
-                size: 14,
-                color: "#ffffff",
-                strokeWidth: 0,
-            },
-            borderWidth: 2,
-            shadow: true,
-        },
-        edges: {
-            width: 2,
-            color: {
-                color: "#848484",
-                highlight: "#c8c8c8",
-                hover: "#c8c8c8",
-            },
-            arrows: {
-                to: { enabled: false },
-            },
-            smooth: {
-                enabled: true,
-                type: "dynamic",
-                roundness: 0.5,
-            },
-        },
-        physics: {
-            enabled: true,
-            barnesHut: {
-                gravitationalConstant: -4000,
-                centralGravity: 0.15,
-                springLength: 150,
-                springConstant: 0.04,
-                damping: 0.1,
-                avoidOverlap: 0.2
-            },
-            solver: 'barnesHut',
-            stabilization: {
-                iterations: 1500,
-            },
-        },
-        interaction: {
-            hover: true,
-            tooltipDelay: 200,
-            hideEdgesOnDrag: true,
-            navigationButtons: true,
-        },
-        layout: {
-            randomSeed: 42,
-        }
-    };
-
-    network = new vis.Network(container, data, options);
-}
-
-// Explicitly attach to window to ensure global availability
-window.renderTechGraph = renderTechGraph;
-
-console.log("graph_renderer.js loaded, renderTechGraph available:", typeof renderTechGraph);
--- /dev/null
+++ b/repo_src/frontend/public/js/cytoscape_renderer.js
@@ -0,0 +1,111 @@
+var cy = null;
+
+function render_cytoscape_graph(containerId, elements) {
+    if (cy) {
+        cy.destroy();
+    }
+    var container = document.getElementById(containerId);
+    if (!container) {
+        console.error("Cytoscape container not found:", containerId);
+        return;
+    }
+
+    cy = cytoscape({
+        container: container,
+        elements: elements,
+        style: [
+            {
+                selector: 'node',
+                style: {
+                    'background-color': '#666',
+                    'label': 'data(label)',
+                    'color': '#fff',
+                    'text-valign': 'center',
+                    'text-halign': 'center',
+                    'font-size': '10px',
+                    'text-wrap': 'wrap',
+                    'text-max-width': '90px',
+                    'text-outline-color': '#333',
+                    'text-outline-width': 2,
+                }
+            },
+            {
+                selector: 'edge',
+                style: {
+                    'width': 1.5,
+                    'line-color': '#555',
+                    'target-arrow-color': '#555',
+                    'target-arrow-shape': 'triangle',
+                    'curve-style': 'bezier'
+                }
+            },
+            // Custom styles based on 'group' data property
+            {
+                selector: 'node[group="Category"]',
+                style: {
+                    'background-color': '#4a90e2',
+                    'shape': 'diamond',
+                    'width': '70px',
+                    'height': '70px',
+                    'font-size': '14px',
+                    'font-weight': 'bold',
+                }
+            },
+            {
+                selector: 'node[group="Technology"]',
+                style: {
+                    'background-color': '#7b68ee',
+                    'width': '40px',
+                    'height': '40px'
+                }
+            },
+            {
+                selector: 'node[group="TechnologyHighlighted"]',
+                style: {
+                    'background-color': '#ff6b6b',
+                    'border-color': '#e55454',
+                    'border-width': 3,
+                    'width': '50px',
+                    'height': '50px',
+                    'font-size': '12px',
+                    'z-index': 10
+                }
+            },
+            {
+                selector: 'node[group="Book"]',
+                style: {
+                    'background-color': '#ffa726',
+                    'shape': 'round-rectangle',
+                    'width': '35px',
+                    'height': '35px',
+                }
+            },
+        ],
+        layout: {
+            name: 'cose',
+            idealEdgeLength: 100,
+            nodeOverlap: 20,
+            refresh: 20,
+            fit: true,
+            padding: 30,
+            randomize: false,
+            componentSpacing: 100,
+            nodeRepulsion: 400000,
+            edgeElasticity: 100,
+            nestingFactor: 5,
+            gravity: 80,
+            numIter: 1000,
+            initialTemp: 200,
+            coolingFactor: 0.95,
+            minTemp: 1.0
+        }
+    });
+}
+
+// Explicitly attach to window to ensure global availability
+window.render_cytoscape_graph = render_cytoscape_graph;
+
+console.log("cytoscape_renderer.js loaded");
--- a/repo_src/frontend/src/components/tech_graph_view.rs
+++ b/repo_src/frontend/src/components/tech_graph_view.rs
@@ -1,30 +1,52 @@
 use leptos::*;
 use serde::{Deserialize, Serialize};
 
+
 #[cfg(feature = "hydrate")]
 use wasm_bindgen::prelude::*;
 
 #[cfg(feature = "hydrate")]
 #[wasm_bindgen]
 extern "C" {
-    #[wasm_bindgen(js_name = renderTechGraph)]
-    fn render_tech_graph(container_id: &str, nodes: JsValue, edges: JsValue);
-}
-
-#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
-pub struct Node {
-    pub id: String,
-    pub label: String,
-    #[serde(rename = "group")]
-    pub group: String,
-    pub title: String, // Tooltip
-    pub shape: String,
+    #[wasm_bindgen(js_name = render_cytoscape_graph)]
+    fn render_cytoscape_graph(container_id: &str, elements: JsValue);
 }
 
 #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
-pub struct Edge {
-    pub from: String,
-    pub to: String,
+pub struct CyNodeData {
+    pub id: String,
+    pub label: String,
+    pub group: String,
+}
+
+#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
+pub struct CyNode {
+    pub data: CyNodeData,
+}
+
+#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
+pub struct CyEdgeData {
+    pub id: String,
+    pub source: String,
+    pub target: String,
+}
+
+#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
+pub struct CyEdge {
+    pub data: CyEdgeData,
+}
+
+#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
+pub struct Elements {
+    pub nodes: Vec<CyNode>,
+    pub edges: Vec<CyEdge>,
+}
+
+impl Elements {
+    pub fn is_empty(&self) -> bool {
+        self.nodes.is_empty() && self.edges.is_empty()
+    }
 }
 
 #[component]
@@ -34,60 +56,39 @@
 ) -> impl IntoView {
     let graph_container_ref = create_node_ref::<html::Div>();
 
+    // This effect will re-run whenever the `elements` signal changes.
     create_effect(move |_| {
-        let current_nodes = nodes.get();
-        let current_edges = edges.get();
-        
-        if current_nodes.is_empty() {
-            return;
-        }
+        if let Some(cy_elements) = elements.get() {
+            if cy_elements.nodes.is_empty() {
+                // Don't render if there are no nodes to avoid JS errors
+                return;
+            }
+            if let Some(div) = graph_container_ref.get() {
+                div.set_id("cytoscape-container");
 
-        if let Some(div) = graph_container_ref.get() {
-            div.set_id("tech-graph-container");
-
-            #[cfg(feature = "hydrate")]
-            {
-                // Use serde_wasm_bindgen to convert Rust structs to JsValue
-                match (
-                    serde_wasm_bindgen::to_value(¤t_nodes),
-                    serde_wasm_bindgen::to_value(¤t_edges),
-                ) {
-                    (Ok(nodes_js), Ok(edges_js)) => {
-                        // Add a small delay to ensure DOM and scripts are ready
-                        wasm_bindgen_futures::spawn_local(async move {
-                            gloo_timers::future::TimeoutFuture::new(100).await;
-                            render_tech_graph("tech-graph-container", nodes_js, edges_js);
-                        });
+                #[cfg(feature = "hydrate")]
+                {
+                    match serde_wasm_bindgen::to_value(&cy_elements) {
+                        Ok(elements_js) => {
+                             render_cytoscape_graph("cytoscape-container", elements_js);
+                        }
+                        Err(e) => logging::error!("Failed to serialize elements: {:?}", e),
                     }
-                    (Err(e), _) => logging::error!("Failed to serialize nodes: {:?}", e),
-                    (_, Err(e)) => logging::error!("Failed to serialize edges: {:?}", e),
                 }
             }
         }
     });
 
     view! {
         <div class="graph-view-wrapper">
-            {move || if nodes.get().is_empty() {
-                view! {
-                    <div class="graph-placeholder">
-                        <p>"Loading technology network data..."</p>
-                    </div>
-                }.into_view()
-            } else {
-                view! {
-                     <div class="graph-container" _ref=graph_container_ref>
-                        // The JS will render the graph here
-                     </div>
-                }.into_view()
+            {move || match elements.get() {
+                Some(e) if !e.nodes.is_empty() => view! {
+                    <div class="graph-container" _ref=graph_container_ref></div>
+                }.into_view(),
+                _ => view! {
+                    <div class="graph-placeholder"><p>"Select a filter to display the technology graph."</p></div>
+                }.into_view(),
             }}
         </div>
     }
 } 
-
-===== repo_src/frontend/src/pages/tech_graph_page.rs
+++ b/repo_src/frontend/src/pages/tech_graph_page.rs
@@ -1,8 +1,8 @@
 use leptos::*;
-use serde::{Deserialize, Serialize};
+use serde::Deserialize;
 use std::collections::{HashMap, HashSet};
-use crate::components::tech_graph_view::{TechGraphView, Node, Edge};
+use crate::components::tech_graph_view::{TechGraphView, Elements, CyNode, CyNodeData, CyEdge, CyEdgeData};
 
-#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
+#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
 struct Book {
     id: i32,
     title: String,
@@ -10,7 +10,7 @@
     series: String,
 }
 
-#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
+#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
 struct Tech {
     id: i32,
     name: String,
@@ -20,7 +20,7 @@
     description: String,
 }
 
-#[derive(Deserialize, Serialize, Clone, Debug)]
+#[derive(Deserialize, Clone, Debug)]
 struct BookTechLink {
     book_id: i32,
     tech_id: i32,
@@ -28,155 +28,127 @@
 
 async fn fetch_csv_data<T: for<'de> Deserialize<'de>>(url: &str) -> Result<Vec<T>, ()> {
     #[cfg(feature = "hydrate")]
-    {
+    {
         let text = gloo_net::http::Request::get(url).send().await.map_err(|e| {
             logging::error!("Failed to fetch {}: {:?}", url, e);
         })?.text().await.map_err(|e| {
             logging::error!("Failed to get text from {}: {:?}", url, e);
         })?;
         let mut rdr = csv::Reader::from_reader(text.as_bytes());
-        rdr.deserialize().collect::<Result<Vec<T>, _>>().map_err(|e| {
+        let result = rdr.deserialize().collect::<Result<Vec<T>, _>>();
+        if result.is_err() {
             logging::error!("Failed to parse CSV from {}: {:?}", url, e);
-        })
+        }
+        result.map_err(|_|())
     }
     #[cfg(not(feature = "hydrate"))]
     {
         // Server-side: return empty data
-        logging::log!("Server-side: returning empty data for {}", url);
         Err(())
     }
 }
 
 #[component]
 pub fn TechGraphPage() -> impl IntoView {
-    let data_resource = create_local_resource(
+    let data_resource = create_resource(
         || (),
         |_| async {
-            logging::log!("Starting to fetch CSV data...");
             let books = fetch_csv_data::<Book>("/data/books.csv").await;
             let techs = fetch_csv_data::<Tech>("/data/technologies.csv").await;
             let links = fetch_csv_data::<BookTechLink>("/data/book_tech_links.csv").await;
-            logging::log!("All CSV fetch attempts completed");
             (books, techs, links)
         },
     );
 
     let (selected_technology, set_selected_technology) = create_signal::<Option<i32>>(None);
     let (selected_category, set_selected_category) = create_signal::<Option<String>>(None);
 
-    let graph_data = create_memo(move |_| {
-        match data_resource.get() {
-            Some((books_res, techs_res, links_res)) => {
-                logging::log!("Data resource available, checking results...");
-                if let (Ok(books), Ok(techs), Ok(links)) = (books_res, techs_res, links_res) {
-                    logging::log!("All data loaded successfully: {} books, {} techs, {} links", 
-                                books.len(), techs.len(), links.len());
-                    let tech_map: HashMap<i32, Tech> = techs.iter().cloned().map(|t| (t.id, t)).collect();
-                    let book_map: HashMap<i32, Book> = books.iter().cloned().map(|b| (b.id, b)).collect();
-                    
-                    let mut nodes = Vec::new();
-                    let mut edges = Vec::new();
-                    
-                    let tech_filter = selected_technology.get();
-                    let category_filter = selected_category.get();
+    let graph_elements = create_memo(move |_| {
+        data_resource.and_then(|(books_res, techs_res, links_res)| {
+            let books = books_res.as_ref().ok()?;
+            let techs = techs_res.as_ref().ok()?;
+            let links = links_res.as_ref().ok()?;
 
-                    // Create category nodes (large nodes)
-                    let categories: HashSet<String> = techs.iter().map(|t| t.category.clone()).collect();
-                    for category in &categories {
-                        let _is_selected = category_filter.as_ref() == Some(category);
-                        nodes.push(Node {
-                            id: format!("c_{}", category),
-                            label: category.clone(),
-                            group: "Category".to_string(),
-                            title: format!("<b>Category: {}</b><br>Click to filter by this category", category),
-                            shape: "diamond".to_string(),
-                        });
-                    }
+            let mut cy_nodes = Vec::new();
+            let mut cy_edges = Vec::new();
 
-                    // Create technology nodes (medium nodes) and connect to categories
-                    for tech in &techs {
-                        let is_filtered = tech_filter == Some(tech.id) || 
-                                         category_filter.as_ref() == Some(&tech.category);
-                        
-                        nodes.push(Node {
-                            id: format!("t_{}", tech.id),
-                            label: tech.name.clone(),
-                            group: if is_filtered { "TechnologyHighlighted".to_string() } else { "Technology".to_string() },
-                            title: format!("<b>{}</b><br><i>{}</i><br>{}<br>Click to see related books", 
-                                         tech.name, tech.subcategory, tech.description),
-                            shape: "dot".to_string(),
-                        });
-                        
-                        // Connect technology to its category
-                        edges.push(Edge {
-                            from: format!("t_{}", tech.id),
-                            to: format!("c_{}", tech.category),
-                        });
-                    }
+            let tech_filter = selected_technology.get();
+            let category_filter = selected_category.get();
 
-                    // Create book nodes (small nodes) and connect to technologies
-                    let mut connected_books = HashSet::new();
-                    
-                    for link in &links {
-                        if let (Some(tech), Some(book)) = (tech_map.get(&link.tech_id), book_map.get(&link.book_id)) {
-                            let tech_is_filtered = tech_filter == Some(tech.id) || 
-                                                  category_filter.as_ref() == Some(&tech.category);
-                            
-                            // Always include the book, but highlight if connected to filtered tech
-                            if !connected_books.contains(&book.id) {
-                                connected_books.insert(book.id);
-                                nodes.push(Node {
-                                    id: format!("b_{}", book.id),
-                                    label: book.title.clone(),
-                                    group: if tech_is_filtered { "BookHighlighted".to_string() } else { "Book".to_string() },
-                                    title: format!("<b>{}</b><br>by {}<br>Series: {}", 
-                                                 book.title, book.author, 
-                                                 if book.series.is_empty() { "Standalone".to_string() } else { book.series.clone() }),
-                                    shape: "box".to_string(),
-                                });
-                            }
-                            
-                            // Connect book to technology (only show if no filter or tech is relevant)
-                            if (tech_filter.is_none() && category_filter.is_none()) || 
-                               tech_filter == Some(tech.id) || 
-                               category_filter.as_ref() == Some(&tech.category) {
-                                edges.push(Edge {
-                                    from: format!("b_{}", book.id),
-                                    to: format!("t_{}", tech.id),
-                                });
-                            }
-                        }
-                    }
+            if tech_filter.is_none() && category_filter.is_none() {
+                return Some(Elements::default());
+            }
 
-                    logging::log!("Generated graph with {} nodes and {} edges", nodes.len(), edges.len());
-                    (nodes, edges)
-                } else {
-                    logging::error!("Failed to load some data resources");
-                    (Vec::new(), Vec::new())
+            let mut relevant_techs = HashSet::new();
+            if let Some(tech_id) = tech_filter {
+                if let Some(tech) = techs.iter().find(|t| t.id == tech_id) {
+                    relevant_techs.insert(tech.clone());
                 }
             }
-            None => {
-                logging::log!("Data resource not yet available");
-                (Vec::new(), Vec::new())
+            if let Some(cat) = &category_filter {
+                for tech in techs.iter().filter(|t| &t.category == cat) {
+                    relevant_techs.insert(tech.clone());
+                }
             }
-        }
+
+            for tech in &relevant_techs {
+                cy_nodes.push(CyNode { data: CyNodeData { id: format!("t_{}", tech.id), label: tech.name.clone(), group: "TechnologyHighlighted".to_string() } });
+            }
+            
+            let mut relevant_books = HashSet::new();
+            for link in links.iter().filter(|l| relevant_techs.iter().any(|t| t.id == l.tech_id)) {
+                if let Some(book) = books.iter().find(|b| b.id == link.book_id) {
+                    relevant_books.insert(book.clone());
+                }
+            }
+            for book in &relevant_books {
+                cy_nodes.push(CyNode { data: CyNodeData { id: format!("b_{}", book.id), label: book.title.clone(), group: "Book".to_string() } });
+            }
+            
+            for link in links.iter().filter(|l| relevant_techs.iter().any(|t| t.id == l.tech_id) && relevant_books.iter().any(|b| b.id == l.book_id)) {
+                cy_edges.push(CyEdge { data: CyEdgeData { id: format!("e_{}_{}", link.book_id, link.tech_id), source: format!("b_{}", link.book_id), target: format!("t_{}", link.tech_id) } });
+            }
+            
+            Some(Elements { nodes: cy_nodes, edges: cy_edges })
+        })
     });
 
-    let nodes = Signal::derive(move || graph_data.get().0);
-    let edges = Signal::derive(move || graph_data.get().1);
-
     view! {
         <div class="tech-graph-page">
-            <h1>"Sci-Fi Technology Network"</h1>
-            <p>"Explore the relationships between books, technologies, and categories. Blue diamonds are categories, purple circles are technologies, and orange boxes are books. Select a technology or category to highlight related connections."</p>
+            <h1>"Technology Graph"</h1>
+            <p>"Visualize connections between technologies and sci-fi books."</p>
             
             <Suspense fallback=move || view!{<p>"Loading data..."</p>}>
                 <ErrorBoundary fallback=|_| view!{<p>"Error loading graph data."</p>}>
-                    { move || data_resource.map(|(_, techs, _)| {
+                    { move || data_resource.map(|(_, techs_res, _)| {
-                        let techs = techs.clone().unwrap_or_default();
-                        let categories: Vec<String> = techs.iter()
+                        let mut techs = techs_res.clone().unwrap_or_default();
+                        techs.sort_by(|a, b| a.name.cmp(&b.name));
+
+                        let mut categories: Vec<String> = techs.iter()
                             .map(|t| t.category.clone())
                             .collect::<HashSet<_>>()
                             .into_iter()
                             .collect();
+                        categories.sort();
                         
                         view! {
                             <div class="graph-controls card">
@@ -194,7 +166,7 @@
                                     >
                                         <option value="none">"-- Select a Technology --"</option>
                                         {techs.iter().map(|t| view!{ 
-                                            <option value=t.id>{format!("{} ({})", t.name, t.category)}</option> 
+                                            <option value=t.id selected=move || selected_technology.get() == Some(t.id)>{t.name.clone()}</option> 
                                         }).collect_view()}
                                     </select>
                                 </div>
@@ -212,18 +184,18 @@
                                         }
                                     >
                                         <option value="none">"-- Select a Category --"</option>
-                                        {categories.into_iter().map(|c| view!{ <option value=c.clone()>{c}</option> }).collect_view()}
+                                        {categories.into_iter().map(|c| view!{ <option value=c.clone() selected=move || selected_category.get() == Some(c.clone())>{c}</option> }).collect_view()}
                                     </select>
                                 </div>
                                 <button on:click=move |_| {
-                                    set_selected_technology.set(None);
-                                    set_selected_category.set(None);
+                                    set_selected_technology(None);
+                                    set_selected_category(None);
                                 }>
                                 "Clear Filter"
                                 </button>
                             </div>
                         }
-                    })}
+                    })} 
                 </ErrorBoundary>
             </Suspense>
             
-            <TechGraphView nodes=nodes edges=edges />
+            <TechGraphView elements=graph_elements />
         </div>
     }
 }