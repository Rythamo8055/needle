const { invoke } = window.__TAURI__.core;

const queryEl = document.getElementById("query");
const askBtn = document.getElementById("ask-btn");
const resultEl = document.getElementById("result");
const statusEl = document.getElementById("status");
const modal = document.getElementById("sudo-modal");
const passEl = document.getElementById("sudo-pass");
const okBtn = document.getElementById("sudo-ok");
const cancelBtn = document.getElementById("sudo-cancel");
const errorEl = document.getElementById("sudo-error");

let pendingQuery = null;

async function doAsk(query) {
  statusEl.textContent = "Thinking…";
  resultEl.innerHTML = '<div class="placeholder">Running…</div>';
  try {
    const res = await invoke("ask", { query });
    if (res.results[0] === "NEED_SUDO" || res.results[0].includes("Authentication required")) {
      pendingQuery = query;
      showSudo();
      resultEl.innerHTML = '<div class="placeholder">Sudo required — enter password</div>';
      statusEl.textContent = "Sudo required";
      return;
    }
    render(res);
  } catch (e) {
    resultEl.textContent = "Error: " + e;
    statusEl.textContent = "Error";
  }
}

function render(res) {
  const tools = res.tools.join(", ") || "no tool";
  let html = `<div class="tool-tag">${tools}</div>\n`;
  res.results.forEach((r, i) => {
    const label = res.tools[i] ? ` — ${res.tools[i]}` : "";
    html += `<div><strong>Result${label}:</strong>\n${escapeHtml(r)}</div>\n`;
    if (i < res.results.length - 1) html += "\n";
  });
  html += `<div class="confidence">Confidence: ${(res.confidence * 100).toFixed(0)}% · ${escapeHtml(res.reasoning)}</div>`;
  resultEl.innerHTML = html;
  statusEl.textContent = `Done · ${tools} · ${(res.confidence * 100).toFixed(0)}%`;
}

function escapeHtml(s) {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function showSudo() {
  modal.classList.remove("hidden");
  passEl.value = "";
  errorEl.classList.add("hidden");
  passEl.focus();
}
function hideSudo() {
  modal.classList.add("hidden");
  pendingQuery = null;
}

async function submitSudo() {
  const pw = passEl.value;
  if (!pw) return;
  okBtn.textContent = "…";
  const ok = await invoke("set_sudo", { password: pw });
  okBtn.textContent = "Unlock";
  if (ok) {
    hideSudo();
    statusEl.textContent = "Sudo cached";
    if (pendingQuery) {
      const q = pendingQuery;
      pendingQuery = null;
      doAsk(q);
    }
  } else {
    errorEl.classList.remove("hidden");
  }
}

// Events
askBtn.addEventListener("click", () => {
  const q = queryEl.value.trim();
  if (q) doAsk(q);
});
queryEl.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    const q = queryEl.value.trim();
    if (q) doAsk(q);
  }
});
document.querySelectorAll(".quick button").forEach((b) => {
  b.addEventListener("click", () => {
    const q = b.dataset.q;
    queryEl.value = q;
    doAsk(q);
  });
});
okBtn.addEventListener("click", submitSudo);
cancelBtn.addEventListener("click", hideSudo);
passEl.addEventListener("keydown", (e) => {
  if (e.key === "Enter") submitSudo();
});
modal.addEventListener("click", (e) => {
  if (e.target === modal) hideSudo();
});

// Init
invoke("check_sudo").then((ok) => {
  statusEl.textContent = ok ? "Ready · sudo cached · 5 tools · 100%" : "Ready · 5 tools (disk health needs pkexec)";
});
