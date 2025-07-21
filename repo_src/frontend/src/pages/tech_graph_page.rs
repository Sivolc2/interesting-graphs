use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::components::tech_graph_view::{TechGraphView, Node, Edge};

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
        rdr.deserialize().collect::<Result<Vec<T>, _>>().map_err(|e| {
            logging::error!("Failed to parse CSV from {}: {:?}", url, e);
        })
    }
    #[cfg(not(feature = "hydrate"))]
    {
        // Server-side: return empty data
        logging::log!("Server-side: returning empty data for {}", url);
        Err(())
    }
}

#[component]
pub fn TechGraphPage() -> impl IntoView {
    let data_resource = create_local_resource(
        || (),
        |_| async {
            logging::log!("Starting to fetch CSV data...");
            let books = fetch_csv_data::<Book>("/data/books.csv").await;
            let techs = fetch_csv_data::<Tech>("/data/technologies.csv").await;
            let links = fetch_csv_data::<BookTechLink>("/data/book_tech_links.csv").await;
            logging::log!("All CSV fetch attempts completed");
            (books, techs, links)
        },
    );

    let (selected_technology, set_selected_technology) = create_signal::<Option<i32>>(None);
    let (selected_category, set_selected_category) = create_signal::<Option<String>>(None);

    let graph_data = create_memo(move |_| {
        match data_resource.get() {
            Some((books_res, techs_res, links_res)) => {
                logging::log!("Data resource available, checking results...");
                if let (Ok(books), Ok(techs), Ok(links)) = (books_res, techs_res, links_res) {
                    logging::log!("All data loaded successfully: {} books, {} techs, {} links", 
                                books.len(), techs.len(), links.len());
                    let tech_map: HashMap<i32, Tech> = techs.iter().cloned().map(|t| (t.id, t)).collect();
                    let book_map: HashMap<i32, Book> = books.iter().cloned().map(|b| (b.id, b)).collect();
                    
                    let mut nodes = Vec::new();
                    let mut edges = Vec::new();
                    
                    let tech_filter = selected_technology.get();
                    let category_filter = selected_category.get();

                    // Create category nodes (large nodes)
                    let categories: HashSet<String> = techs.iter().map(|t| t.category.clone()).collect();
                    for category in &categories {
                        let _is_selected = category_filter.as_ref() == Some(category);
                        nodes.push(Node {
                            id: format!("c_{}", category),
                            label: category.clone(),
                            group: "Category".to_string(),
                            title: format!("<b>Category: {}</b><br>Click to filter by this category", category),
                            shape: "diamond".to_string(),
                        });
                    }

                    // Create technology nodes (medium nodes) and connect to categories
                    for tech in &techs {
                        let is_filtered = tech_filter == Some(tech.id) || 
                                         category_filter.as_ref() == Some(&tech.category);
                        
                        nodes.push(Node {
                            id: format!("t_{}", tech.id),
                            label: tech.name.clone(),
                            group: if is_filtered { "TechnologyHighlighted".to_string() } else { "Technology".to_string() },
                            title: format!("<b>{}</b><br><i>{}</i><br>{}<br>Click to see related books", 
                                         tech.name, tech.subcategory, tech.description),
                            shape: "dot".to_string(),
                        });
                        
                        // Connect technology to its category
                        edges.push(Edge {
                            from: format!("t_{}", tech.id),
                            to: format!("c_{}", tech.category),
                        });
                    }

                    // Create book nodes (small nodes) and connect to technologies
                    let mut connected_books = HashSet::new();
                    
                    for link in &links {
                        if let (Some(tech), Some(book)) = (tech_map.get(&link.tech_id), book_map.get(&link.book_id)) {
                            let tech_is_filtered = tech_filter == Some(tech.id) || 
                                                  category_filter.as_ref() == Some(&tech.category);
                            
                            // Always include the book, but highlight if connected to filtered tech
                            if !connected_books.contains(&book.id) {
                                connected_books.insert(book.id);
                                nodes.push(Node {
                                    id: format!("b_{}", book.id),
                                    label: book.title.clone(),
                                    group: if tech_is_filtered { "BookHighlighted".to_string() } else { "Book".to_string() },
                                    title: format!("<b>{}</b><br>by {}<br>Series: {}", 
                                                 book.title, book.author, 
                                                 if book.series.is_empty() { "Standalone".to_string() } else { book.series.clone() }),
                                    shape: "box".to_string(),
                                });
                            }
                            
                            // Connect book to technology (only show if no filter or tech is relevant)
                            if (tech_filter.is_none() && category_filter.is_none()) || 
                               tech_filter == Some(tech.id) || 
                               category_filter.as_ref() == Some(&tech.category) {
                                edges.push(Edge {
                                    from: format!("b_{}", book.id),
                                    to: format!("t_{}", tech.id),
                                });
                            }
                        }
                    }

                    logging::log!("Generated graph with {} nodes and {} edges", nodes.len(), edges.len());
                    (nodes, edges)
                } else {
                    logging::error!("Failed to load some data resources");
                    (Vec::new(), Vec::new())
                }
            }
            None => {
                logging::log!("Data resource not yet available");
                (Vec::new(), Vec::new())
            }
        }
    });

    let nodes = Signal::derive(move || graph_data.get().0);
    let edges = Signal::derive(move || graph_data.get().1);

    view! {
        <div class="tech-graph-page">
            <h1>"Sci-Fi Technology Network"</h1>
            <p>"Explore the relationships between books, technologies, and categories. Blue diamonds are categories, purple circles are technologies, and orange boxes are books. Select a technology or category to highlight related connections."</p>
            
            <Suspense fallback=move || view!{<p>"Loading data..."</p>}>
                <ErrorBoundary fallback=|_| view!{<p>"Error loading graph data."</p>}>
                    { move || data_resource.map(|(_, techs, _)| {
                        let techs = techs.clone().unwrap_or_default();
                        let categories: Vec<String> = techs.iter()
                            .map(|t| t.category.clone())
                            .collect::<HashSet<_>>()
                            .into_iter()
                            .collect();
                        
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
                                        {techs.iter().map(|t| view!{ 
                                            <option value=t.id>{format!("{} ({})", t.name, t.category)}</option> 
                                        }).collect_view()}
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
                                        {categories.into_iter().map(|c| view!{ <option value=c.clone()>{c}</option> }).collect_view()}
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
            
            <TechGraphView nodes=nodes edges=edges />
        </div>
    }
} 