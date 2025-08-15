// --- Global State ---
let chainData = [];
const API_URL = "http://127.0.0.1:3000/api/chain";
const POLLING_INTERVAL = 3000; // Check for new data every 3 seconds

// --- D3 Visualization Logic ---
const svgContainer = d3.select("#visualization");
const tooltip = d3.select(".tooltip");

async function fetchChainData() {
    try {
        const response = await fetch(API_URL);
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        
        // Only re-render if the chain has actually changed
        if (data.chain.length !== chainData.length) {
            console.log("New block detected! Re-rendering visualization.");
            chainData = data.chain;
            renderChain();
        }
    } catch (error) {
        console.error("Could not fetch blockchain data:", error);
        svgContainer.html(`<div class="text-center p-8 text-red-400">
            <h3 class="text-lg font-semibold">Connection Error</h3>
            <p class="mt-2 text-sm">Could not connect to the DevChain node. Is it running?</p>
            <p class="mt-1 text-xs font-mono">${API_URL}</p>
        </div>`);
    }
}

function renderChain() {
    const { width } = svgContainer.node().getBoundingClientRect();
    const height = 500;
    
    svgContainer.selectAll("*").remove(); // Clear previous render

    const svg = svgContainer.append("svg")
        .attr("width", width)
        .attr("height", height);

    const blockWidth = 140;
    const blockHeight = 80;
    const blockPadding = 40;
    const totalWidth = chainData.length * (blockWidth + blockPadding) - blockPadding;
    
    const startX = (width - totalWidth) / 2;

    // --- Draw Links ---
    const links = svg.append("g").selectAll("line")
        .data(chainData.slice(1))
        .enter()
        .append("line")
        .attr("class", "hash-link")
        .attr("x1", (d, i) => startX + i * (blockWidth + blockPadding) + blockWidth)
        .attr("y1", height / 2)
        .attr("x2", (d, i) => startX + (i + 1) * (blockWidth + blockPadding))
        .attr("y2", height / 2)
        .attr("stroke-width", 2);

    // --- Draw Blocks ---
    const blocks = svg.append("g").selectAll("g")
        .data(chainData)
        .enter()
        .append("g")
        .attr("class", "block")
        .attr("transform", (d, i) => `translate(${startX + i * (blockWidth + blockPadding)}, ${height / 2 - blockHeight / 2})`);

    blocks.append("rect")
        .attr("width", blockWidth)
        .attr("height", blockHeight)
        .attr("rx", 8)
        .attr("ry", 8)
        .attr("fill", d => d.index === 0 ? "#4b5563" : "#16a34a")
        .attr("stroke", "#9ca3af")
        .attr("stroke-width", 1);

    blocks.append("text")
        .text(d => `Block #${d.index}`)
        .attr("x", blockWidth / 2)
        .attr("y", 25)
        .attr("text-anchor", "middle")
        .attr("fill", "white")
        .style("font-weight", "600");

    blocks.append("text")
        .text(d => `Lang: ${d.proof.language}`)
        .attr("x", blockWidth / 2)
        .attr("y", 50)
        .attr("text-anchor", "middle")
        .attr("fill", "#d1d5db")
        .style("font-size", "12px");
    
    blocks.append("text")
        .text(d => d.hash.substring(0, 6) + "...")
        .attr("x", blockWidth / 2)
        .attr("y", 70)
        .attr("text-anchor", "middle")
        .attr("fill", "#a1a1aa")
        .style("font-size", "10px")
        .style("font-family", "'Roboto Mono', monospace");

    // --- Tooltip Interaction ---
    blocks.on("mouseover", (event, d) => {
        tooltip.transition().duration(200).style("opacity", .95);
        tooltip.html(`
            <strong>Index:</strong> ${d.index}<br>
            <strong>Timestamp:</strong> ${new Date(d.timestamp * 1000).toLocaleString()}<br>
            <strong>Author:</strong> ${d.proof.author}<br>
            <hr class="my-1 border-gray-500">
            <strong>Hash:</strong> <span class="text-green-400">${d.hash.substring(0, 20)}...</span><br>
            <strong>Prev Hash:</strong> <span class="text-yellow-400">${d.previous_hash.substring(0, 20)}...</span><br>
            <hr class="my-1 border-gray-500">
            <strong>Proof (${d.proof.language}):</strong>
            <pre class="bg-gray-800 p-1 rounded mt-1 text-xs">${d.proof.code.length > 50 ? d.proof.code.substring(0, 50) + '...' : d.proof.code}</pre>
        `)
        .style("left", (event.pageX + 15) + "px")
        .style("top", (event.pageY - 28) + "px");
    })
    .on("mouseout", () => {
        tooltip.transition().duration(500).style("opacity", 0);
    });
}

// --- Manual Refresh Button ---
document.getElementById("add-block-btn").addEventListener("click", () => {
    console.log("Manual refresh triggered. Refetching chain data...");
    fetchChainData();
});

// --- Initial Load & Auto-Refresh ---
fetchChainData(); // Fetch data immediately on page load
setInterval(fetchChainData, POLLING_INTERVAL); // Then, check for new data every few seconds
window.addEventListener('resize', renderChain);
