import React, { useState } from 'react';
import { createRoot } from 'react-dom/client';

type Business = { id: string; name: string };

function App() {
  const [business, setBusiness] = useState<Business | null>(null);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  async function runTracer() {
    setLoading(true); setError(''); setBusiness(null);
    try {
      const created = await fetch('/v1/dev/seed-business', { method: 'POST' });
      if (!created.ok) throw new Error(`Create failed (${created.status})`);
      const item: Business = await created.json();
      const result = await fetch(`/v1/dev/businesses/${item.id}`);
      if (!result.ok) throw new Error(`Read failed (${result.status})`);
      setBusiness(await result.json());
    } catch (e) { setError(e instanceof Error ? e.message : 'Unexpected error'); }
    finally { setLoading(false); }
  }

  return <main style={{ fontFamily: 'system-ui', maxWidth: 640, margin: '5rem auto', padding: '0 1rem' }}>
    <h1>Platform — TB-01.1</h1>
    <p>React → Rust API → PostgreSQL → Rust API → React</p>
    <button onClick={runTracer} disabled={loading}>{loading ? 'Working…' : 'Create and load demo business'}</button>
    {business && <section><h2>Database result: {business.name}</h2><p>Business ID: {business.id}</p></section>}
    {error && <p role="alert">{error}</p>}
  </main>;
}

createRoot(document.getElementById('root')!).render(<React.StrictMode><App /></React.StrictMode>);
