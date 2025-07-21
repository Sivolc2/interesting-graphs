var network = null;

function renderTechGraph(containerId, nodesJson, edgesJson) {
    var container = document.getElementById(containerId);
    if (!container) {
        console.error("Graph container not found:", containerId);
        return;
    }

    // Destroy the old network instance if it exists
    if (network !== null) {
        network.destroy();
        network = null;
    }

    // Create datasets.
    var nodes = new vis.DataSet(nodesJson);
    var edges = new vis.DataSet(edgesJson);

    var data = {
        nodes: nodes,
        edges: edges,
    };

    var options = {
        groups: {
            Category: {
                shape: "diamond",
                size: 30,
                color: {
                    background: "#4a90e2",
                    border: "#2171b5"
                },
                font: {
                    size: 16,
                    color: "#ffffff",
                    strokeWidth: 2,
                    strokeColor: "#000000"
                },
                borderWidth: 3,
                shadow: true,
            },
            Technology: {
                shape: "dot",
                size: 20,
                color: {
                    background: "#7b68ee",
                    border: "#5a4fcf"
                },
                font: {
                    size: 12,
                    color: "#ffffff",
                    strokeWidth: 1,
                    strokeColor: "#000000"
                },
                borderWidth: 2,
                shadow: true,
            },
            TechnologyHighlighted: {
                shape: "dot",
                size: 25,
                color: {
                    background: "#ff6b6b",
                    border: "#e55454"
                },
                font: {
                    size: 13,
                    color: "#ffffff",
                    strokeWidth: 2,
                    strokeColor: "#000000"
                },
                borderWidth: 3,
                shadow: true,
            },
            Book: {
                shape: "box",
                size: 15,
                color: {
                    background: "#ffa726",
                    border: "#ef6c00"
                },
                font: {
                    size: 10,
                    color: "#ffffff",
                    strokeWidth: 1,
                    strokeColor: "#000000"
                },
                borderWidth: 2,
                shadow: true,
            },
            BookHighlighted: {
                shape: "box",
                size: 18,
                color: {
                    background: "#66bb6a",
                    border: "#4caf50"
                },
                font: {
                    size: 11,
                    color: "#ffffff",
                    strokeWidth: 2,
                    strokeColor: "#000000"
                },
                borderWidth: 3,
                shadow: true,
            }
        },
        nodes: {
            shape: "dot",
            size: 16,
            font: {
                size: 14,
                color: "#ffffff",
                strokeWidth: 0,
            },
            borderWidth: 2,
            shadow: true,
        },
        edges: {
            width: 2,
            color: {
                color: "#848484",
                highlight: "#c8c8c8",
                hover: "#c8c8c8",
            },
            arrows: {
                to: { enabled: false },
            },
            smooth: {
                enabled: true,
                type: "dynamic",
                roundness: 0.5,
            },
        },
        physics: {
            enabled: true,
            barnesHut: {
                gravitationalConstant: -4000,
                centralGravity: 0.15,
                springLength: 150,
                springConstant: 0.04,
                damping: 0.1,
                avoidOverlap: 0.2
            },
            solver: 'barnesHut',
            stabilization: {
                iterations: 1500,
            },
        },
        interaction: {
            hover: true,
            tooltipDelay: 200,
            hideEdgesOnDrag: true,
            navigationButtons: true,
        },
        layout: {
            randomSeed: 42,
        }
    };

    network = new vis.Network(container, data, options);
} 