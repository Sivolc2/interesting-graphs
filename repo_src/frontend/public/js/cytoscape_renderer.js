var cy = null;

function render_cytoscape_graph(containerId, elements) {
    console.log("render_cytoscape_graph called with:", containerId, elements);
    
    // Check if Cytoscape is available
    if (typeof cytoscape === 'undefined') {
        console.error("Cytoscape library not loaded yet");
        return;
    }
    
    if (cy) {
        cy.destroy();
    }
    var container = document.getElementById(containerId);
    if (!container) {
        console.error("Cytoscape container not found:", containerId);
        return;
    }

    cy = cytoscape({
        container: container,
        elements: elements,
        style: [
            {
                selector: 'node',
                style: {
                    'background-color': '#666',
                    'label': 'data(label)',
                    'color': '#fff',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'font-size': '10px',
                    'text-wrap': 'wrap',
                    'text-max-width': '90px',
                    'text-outline-color': '#333',
                    'text-outline-width': 2,
                }
            },
            {
                selector: 'edge',
                style: {
                    'width': 1.5,
                    'line-color': '#555',
                    'target-arrow-color': '#555',
                    'target-arrow-shape': 'triangle',
                    'curve-style': 'bezier'
                }
            },
            // Custom styles based on 'group' data property
            {
                selector: 'node[group="Category"]',
                style: {
                    'background-color': '#4a90e2',
                    'shape': 'diamond',
                    'width': '70px',
                    'height': '70px',
                    'font-size': '14px',
                    'font-weight': 'bold',
                }
            },
            {
                selector: 'node[group="Technology"]',
                style: {
                    'background-color': '#7b68ee',
                    'width': '40px',
                    'height': '40px'
                }
            },
            {
                selector: 'node[group="TechnologyHighlighted"]',
                style: {
                    'background-color': '#ff6b6b',
                    'border-color': '#e55454',
                    'border-width': 3,
                    'width': '50px',
                    'height': '50px',
                    'font-size': '12px',
                    'z-index': 10
                }
            },
            {
                selector: 'node[group="Book"]',
                style: {
                    'background-color': '#ffa726',
                    'shape': 'round-rectangle',
                    'width': '35px',
                    'height': '35px',
                }
            },
        ],
        layout: {
            name: 'cose',
            idealEdgeLength: 100,
            nodeOverlap: 20,
            refresh: 20,
            fit: true,
            padding: 30,
            randomize: false,
            componentSpacing: 100,
            nodeRepulsion: 400000,
            edgeElasticity: 100,
            nestingFactor: 5,
            gravity: 80,
            numIter: 1000,
            initialTemp: 200,
            coolingFactor: 0.95,
            minTemp: 1.0
        }
    });
}

// Explicitly attach to window to ensure global availability
window.render_cytoscape_graph = render_cytoscape_graph;

console.log("cytoscape_renderer.js loaded"); 