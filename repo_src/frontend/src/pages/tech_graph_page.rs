use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::components::tech_graph_view::{TechGraphView, Elements, CyNode, CyNodeData, CyEdge, CyEdgeData};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
struct Book {
    id: i32,
    title: String,
    author: String,
    series: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
struct Tech {
    id: i32,
    name: String,
    category: String,
    subcategory: String,
    description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct BookTechLink {
    book_id: i32,
    tech_id: i32,
}

async fn fetch_csv_data<T: for<'de> Deserialize<'de>>(url: &str) -> Result<Vec<T>, ()> {
    #[cfg(feature = "hydrate")]
    {
        let text = gloo_net::http::Request::get(url).send().await.map_err(|e| {
            logging::error!("Failed to fetch {}: {:?}", url, e);
        })?.text().await.map_err(|e| {
            logging::error!("Failed to get text from {}: {:?}", url, e);
        })?;
        let mut rdr = csv::Reader::from_reader(text.as_bytes());
        let result = rdr.deserialize().collect::<Result<Vec<T>, _>>();
        if let Err(ref e) = result {
            logging::error!("Failed to parse CSV from {}: {:?}", url, e);
        }
        result.map_err(|_|())
    }
    #[cfg(not(feature = "hydrate"))]
    {
        // Server-side: return empty data
        Err(())
    }
}

#[component]
pub fn TechGraphPage() -> impl IntoView {
    let data_resource = create_resource(
        || (),
        |_| async {
            let books = fetch_csv_data::<Book>("/data/books.csv").await;
            let techs = fetch_csv_data::<Tech>("/data/technologies.csv").await;
            let links = fetch_csv_data::<BookTechLink>("/data/book_tech_links.csv").await;
            (books, techs, links)
        },
    );

    let (selected_technology, set_selected_technology) = create_signal::<Option<i32>>(None);
    let (selected_category, set_selected_category) = create_signal::<Option<String>>(None);

    let graph_elements = create_memo(move |_| {
        data_resource.get().and_then(|(books_res, techs_res, links_res)| {
            let books = books_res.as_ref().ok()?;
            let techs = techs_res.as_ref().ok()?;
            let links = links_res.as_ref().ok()?;

            let mut cy_nodes = Vec::new();
            let mut cy_edges = Vec::new();

            let tech_filter = selected_technology.get();
            let category_filter = selected_category.get();

            if tech_filter.is_none() && category_filter.is_none() {
                return Some(Elements::default());
            }

            let mut relevant_techs = HashSet::new();
            if let Some(tech_id) = tech_filter {
                if let Some(tech) = techs.iter().find(|t| t.id == tech_id) {
                    relevant_techs.insert(tech.clone());
                }
            }
            if let Some(cat) = &category_filter {
                for tech in techs.iter().filter(|t| &t.category == cat) {
                    relevant_techs.insert(tech.clone());
                }
            }

            for tech in &relevant_techs {
                cy_nodes.push(CyNode { data: CyNodeData { id: format!("t_{}", tech.id), label: tech.name.clone(), group: "TechnologyHighlighted".to_string() } });
            }
            
            let mut relevant_books = HashSet::new();
            for link in links.iter().filter(|l| relevant_techs.iter().any(|t| t.id == l.tech_id)) {
                if let Some(book) = books.iter().find(|b| b.id == link.book_id) {
                    relevant_books.insert(book.clone());
                }
            }
            for book in &relevant_books {
                cy_nodes.push(CyNode { data: CyNodeData { id: format!("b_{}", book.id), label: book.title.clone(), group: "Book".to_string() } });
            }
            
            for link in links.iter().filter(|l| relevant_techs.iter().any(|t| t.id == l.tech_id) && relevant_books.iter().any(|b| b.id == l.book_id)) {
                cy_edges.push(CyEdge { data: CyEdgeData { id: format!("e_{}_{}", link.book_id, link.tech_id), source: format!("b_{}", link.book_id), target: format!("t_{}", link.tech_id) } });
            }
            
            Some(Elements { nodes: cy_nodes, edges: cy_edges })
        })
    });

    view! {
        <div class="tech-graph-page">
            <h1>"Technology Graph"</h1>
            <p>"Visualize connections between technologies and sci-fi books."</p>
            
            <Suspense fallback=move || view!{<p>"Loading data..."</p>}>
                <ErrorBoundary fallback=|_| view!{<p>"Error loading graph data."</p>}>
                    { move || data_resource.map(|(_, techs_res, _)| {
                        let mut techs = techs_res.clone().unwrap_or_default();
                        techs.sort_by(|a, b| a.name.cmp(&b.name));

                        let mut categories: Vec<String> = techs.iter()
                            .map(|t| t.category.clone())
                            .collect::<HashSet<_>>()
                            .into_iter()
                            .collect();
                        categories.sort();
                        
                        let techs_options = techs.iter().map(|t| {
                            let tech_id = t.id;
                            let tech_name = t.name.clone();
                            view!{ 
                                <option value=tech_id selected=move || selected_technology.get() == Some(tech_id)>{tech_name}</option> 
                            }
                        }).collect_view();

                        let categories_options = categories.iter().map(|c| {
                            let cat_name = c.clone();
                            let cat_name_2 = c.clone();
                            view!{ 
                                <option value=cat_name selected=move || selected_category.get() == Some(cat_name_2.clone())>{c.clone()}</option> 
                            }
                        }).collect_view();
                        
                        view! {
                            <div class="graph-controls card">
                                <div class="control-group">
                                    <label for="tech-filter">"Filter by Technology:"</label>
                                    <select
                                        id="tech-filter"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            if val == "none" {
                                                set_selected_technology.set(None);
                                            } else {
                                                set_selected_technology.set(val.parse::<i32>().ok());
                                            }
                                            set_selected_category.set(None);
                                        }
                                    >
                                        <option value="none">"-- Select a Technology --"</option>
                                        {techs_options}
                                    </select>
                                </div>
                                <div class="control-group">
                                    <label for="category-filter">"Filter by Category:"</label>
                                     <select
                                        id="category-filter"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            if val == "none" {
                                                set_selected_category.set(None);
                                            } else {
                                                set_selected_category.set(Some(val));
                                            }
                                            set_selected_technology.set(None);
                                        }
                                    >
                                        <option value="none">"-- Select a Category --"</option>
                                        {categories_options}
                                    </select>
                                </div>
                                <button on:click=move |_| {
                                    set_selected_technology.set(None);
                                    set_selected_category.set(None);
                                }>
                                "Clear Filter"
                                </button>
                            </div>
                        }
                    })} 
                </ErrorBoundary>
            </Suspense>
            
            <TechGraphView elements=graph_elements.into() />
        </div>
    }
} 