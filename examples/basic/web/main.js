const consoleEl = document.getElementById("console");
let chart;

// --- Logger ---
function log(msg) {
  consoleEl.textContent += `[${new Date().toLocaleTimeString()}] >> ${msg}\n`;
  consoleEl.scrollTop = consoleEl.scrollHeight;
}

// --- Invoke Python command ---
async function invokeCommand(name, params = {}) {
  log(`Calling "${name}" with ${JSON.stringify(params)}`);
  const result = await window.invoke(name, params);
  log(`Response: ${JSON.stringify(result).slice(0, 200)}...`);
  return result;
}

// --- Calculation + Plot ---
document.getElementById("btnCalc").addEventListener("click", async () => {
  const funcs = [...document.querySelectorAll(".funcCheck:checked")].map(cb => cb.value);
  const out = document.getElementById("calcOutput");

  if (funcs.length === 0) {
    out.textContent = "⚠️ Please select at least one function!";
    return;
  }

  out.textContent = "⏳ Computing...";
  try {
    const results = [];
    for (const func of funcs) {
      const res = await invokeCommand("compute_function", { func });
      results.push({ func, ...res });
    }

    out.textContent = `Computed: ${funcs.join(", ")}`;
    plotMultiple(results);
  } catch (err) {
    out.textContent = "❌ Error: " + err.message;
  }
});

// --- Chart.js Plot ---
function plotMultiple(results) {
  const ctx = document.getElementById("plotCanvas").getContext("2d");
  if (chart) chart.destroy();

  const colors = ["#e74c3c", "#3498db", "#2ecc71", "#f39c12"];

  chart = new Chart(ctx, {
    type: "line",
    data: {
      labels: results[0].x.map(v => v.toFixed(2)),
      datasets: results.map((res, i) => ({
        label: res.func,
        data: res.values,
        borderColor: colors[i % colors.length],
        backgroundColor: colors[i % colors.length] + "33",
        borderWidth: 2,
        tension: 0.25,
        fill: false,
        pointRadius: 0
      }))
    },
options: {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    title: {
      display: true,
      text: "Scientific Function Comparison",
      color: "#eee",
      font: { 
        size: 30, 
        family: "Segoe UI, Roboto, sans-serif"   // neue Schriftart
      }
    },
    legend: {
      labels: {
        color: "#ddd",
        font: { 
          size: 16, 
          family: "Segoe UI, Roboto, sans-serif"
        }
      }
    }
  },
  scales: {
    x: {
      title: { 
        display: true, 
        text: "x", 
        color: "#ccc",
        font: { size: 20, weight: "bold", family: "Segoe UI, Roboto, sans-serif" }
      },
      ticks: { 
        color: "#ccc", 
        font: { size: 14, family: "Segoe UI, Roboto, sans-serif" }
      },
      grid: { color: "rgba(200,200,200,0.1)" }
    },
    y: {
      title: { 
        display: true, 
        text: "f(x)", 
        color: "#ccc",
        font: { size: 20, weight: "bold", family: "Segoe UI, Roboto, sans-serif" }
      },
      ticks: { 
        color: "#ccc", 
        font: { size: 14, family: "Segoe UI, Roboto, sans-serif" }
      },
      grid: { color: "rgba(200,200,200,0.1)" }
    }
  }
}

  });
}
