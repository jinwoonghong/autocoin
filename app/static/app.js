async function callApi(path, method = 'GET') {
  const res = await fetch(path, { method });
  return res.json();
}

function updateButtons(status) {
  const startBtn = document.getElementById('startBtn');
  const stopBtn = document.getElementById('stopBtn');
  const resetBtn = document.getElementById('resetBtn');

  startBtn.disabled = status === 'RUNNING' || status === 'STOPPING';
  stopBtn.disabled = status === 'IDLE';
  resetBtn.disabled = status === 'RUNNING' || status === 'STOPPING';
}

function renderStatus(s) {
  document.getElementById('status').textContent = s.status;
  document.getElementById('paper').textContent = s.paper_mode ? 'ON' : 'OFF';
  document.getElementById('thread').textContent = s.thread_alive ? 'YES' : 'NO';
  document.getElementById('market').textContent = s.market;
  document.getElementById('price').textContent = s.last_price ?? '-';
  document.getElementById('signal').textContent = s.last_signal;
  document.getElementById('iteration').textContent = s.iteration;
  document.getElementById('failures').textContent = s.consecutive_failures;
  document.getElementById('maxFailures').textContent = s.max_consecutive_failures;
  document.getElementById('lastError').textContent = s.last_error ?? '-';
  updateButtons(s.status);
}

function renderLogs(items) {
  const text = items.map((x) => `[${x.ts}] ${x.level} ${x.message}`).join('\n');
  document.getElementById('logs').textContent = text;
}

async function refresh() {
  const status = await callApi('/api/engine/status');
  renderStatus(status.data);
  const logs = await callApi('/api/logs/recent?limit=80');
  renderLogs(logs.data);
}

document.getElementById('startBtn').addEventListener('click', async () => {
  await callApi('/api/engine/start', 'POST');
  await refresh();
});

document.getElementById('stopBtn').addEventListener('click', async () => {
  await callApi('/api/engine/stop', 'POST');
  await refresh();
});

document.getElementById('resetBtn').addEventListener('click', async () => {
  await callApi('/api/engine/reset', 'POST');
  await refresh();
});

setInterval(refresh, 2000);
refresh();
