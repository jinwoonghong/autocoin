async function post(url) {
  const res = await fetch(url, { method: 'POST' });
  return res.json();
}

async function pull() {
  const status = await fetch('/api/engine/status').then(r => r.json());
  document.getElementById('status').textContent = status.data.status;
  document.getElementById('cycles').textContent = status.data.cycle_count;
  document.getElementById('error').textContent = status.data.last_error || '-';

  const orders = await fetch('/api/orders/recent').then(r => r.json());
  document.getElementById('orders').textContent = JSON.stringify(orders.data.orders, null, 2);

  const logs = await fetch('/api/logs/recent').then(r => r.json());
  document.getElementById('logs').textContent = JSON.stringify(logs.data.logs, null, 2);
}

document.getElementById('startBtn').addEventListener('click', async () => { await post('/api/engine/start'); await pull(); });
document.getElementById('stopBtn').addEventListener('click', async () => { await post('/api/engine/stop'); await pull(); });
document.getElementById('resetBtn').addEventListener('click', async () => { await post('/api/engine/reset'); await pull(); });

setInterval(pull, 3000);
pull();
