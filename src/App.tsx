import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Dashboard } from './components/Dashboard';
import { Settings } from './components/Settings';
import type { BrowserUpdateInfo } from './types/ipc';

type View = 'dashboard' | 'browsers' | 'settings';

function App() {
  const [view, setView] = useState<View>('dashboard');

  return (
    <div style={{ display: 'flex', width: '100%', height: '100vh', position: 'relative', zIndex: 1 }}>
      <Sidebar view={view} setView={setView} />
      <main style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
        {view === 'dashboard' && <Dashboard />}
        {view === 'browsers' && <BrowserView />}
        {view === 'settings' && <Settings onClose={() => setView('dashboard')} />}
      </main>
    </div>
  );
}

function Sidebar({ view, setView }: { view: View; setView: (v: View) => void }) {
  return (
    <aside className="sidebar" style={{ width: 220, display: 'flex', flexDirection: 'column', padding: '24px 12px', gap: 4, flexShrink: 0 }}>
      {/* Logo */}
      <div style={{ padding: '0 8px 20px', borderBottom: '1px solid rgba(255,255,255,0.06)', marginBottom: 8 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          <div style={{
            width: 32, height: 32, borderRadius: 8,
            background: 'linear-gradient(135deg,#6366f1,#8b5cf6)',
            display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: 0,
          }}>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="23 4 23 10 17 10"/>
              <polyline points="1 20 1 14 7 14"/>
              <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
            </svg>
          </div>
          <div>
            <div style={{ fontSize: 13, fontWeight: 700, color: '#e2e8f0', lineHeight: 1.2 }}>CleanSlate</div>
            <div style={{ fontSize: 11, color: '#475569', lineHeight: 1.2 }}>QA Workstation</div>
          </div>
        </div>
      </div>

      {/* Nav */}
      <nav style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        <NavItem icon={<IconDashboard />} label="Dashboard" active={view === 'dashboard'} onClick={() => setView('dashboard')} />
        <NavItem icon={<IconBrowser />} label="Browsers" active={view === 'browsers'} onClick={() => setView('browsers')} />
        <NavItem icon={<IconSettings />} label="Settings" active={view === 'settings'} onClick={() => setView('settings')} />
      </nav>

      {/* Bottom info */}
      <div style={{ marginTop: 'auto', padding: '12px 8px', borderTop: '1px solid rgba(255,255,255,0.06)' }}>
        <div style={{ fontSize: 11, color: '#334155', lineHeight: 1.6 }}>
          QA Workstation Reset Tool
        </div>
      </div>
    </aside>
  );
}

function NavItem({ icon, label, active, onClick }: { icon: React.ReactNode; label: string; active: boolean; onClick: () => void }) {
  return (
    <button className={`nav-item${active ? ' active' : ''}`} onClick={onClick} style={{ border: 'none', background: 'none', textAlign: 'left', width: '100%' }}>
      <span style={{ width: 16, height: 16, flexShrink: 0 }}>{icon}</span>
      {label}
    </button>
  );
}

function BrowserView() {
  const [updates, setUpdates] = useState<BrowserUpdateInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<BrowserUpdateInfo[]>('check_browser_updates')
      .then(setUpdates)
      .catch(err => setError(String(err)))
      .finally(() => setLoading(false));
  }, []);

  const refresh = () => {
    setLoading(true);
    setError(null);
    invoke<BrowserUpdateInfo[]>('check_browser_updates')
      .then(setUpdates)
      .catch(err => setError(String(err)))
      .finally(() => setLoading(false));
  };

  return (
    <div style={{ padding: 32, overflowY: 'auto', height: '100%' }}>
      <div style={{ maxWidth: 640 }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 24 }}>
          <div>
            <h2 style={{ margin: 0, fontSize: 20, fontWeight: 700, color: '#e2e8f0' }}>Browser Status</h2>
            <p style={{ margin: '4px 0 0', fontSize: 13, color: '#475569' }}>Installed browsers and their versions</p>
          </div>
          <button className="btn-ghost" onClick={refresh} style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={loading ? 'spin' : ''}>
              <polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/>
              <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
            </svg>
            Refresh
          </button>
        </div>

        {loading ? (
          <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12, padding: '48px 0' }}>
            <div style={{ width: 32, height: 32, border: '3px solid rgba(99,102,241,0.2)', borderTopColor: '#6366f1', borderRadius: '50%' }} className="spin" />
            <span style={{ fontSize: 13, color: '#475569' }}>Detecting browsers...</span>
          </div>
        ) : error ? (
          <div className="card" style={{ padding: 20, borderColor: 'rgba(239,68,68,0.2)', background: 'rgba(239,68,68,0.05)' }}>
            <span style={{ color: '#f87171', fontSize: 14 }}>{error}</span>
          </div>
        ) : (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
            {updates.map((u, i) => (
              <div key={i} className="card card-hover" style={{ padding: '16px 20px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
                  <div style={{ width: 8, height: 8, borderRadius: '50%', flexShrink: 0 }}
                       className={u.error ? 'dot-gray' : u.update_available ? 'dot-yellow' : 'dot-green'} />
                  <div>
                    <div style={{ fontSize: 14, fontWeight: 600, color: '#e2e8f0' }}>{u.name}</div>
                    {u.error
                      ? <div style={{ fontSize: 12, color: '#ef4444', marginTop: 2 }}>{u.error}</div>
                      : <div style={{ fontSize: 12, color: '#475569', marginTop: 2 }}>v{u.current_version}</div>
                    }
                  </div>
                </div>
                <span style={{
                  fontSize: 11, fontWeight: 600, padding: '3px 10px', borderRadius: 999,
                  background: u.error ? 'rgba(71,85,105,0.2)' : u.update_available ? 'rgba(234,179,8,0.1)' : 'rgba(34,197,94,0.1)',
                  color: u.error ? '#64748b' : u.update_available ? '#eab308' : '#22c55e',
                  border: `1px solid ${u.error ? 'rgba(71,85,105,0.2)' : u.update_available ? 'rgba(234,179,8,0.2)' : 'rgba(34,197,94,0.2)'}`,
                }}>
                  {u.error ? 'Not Found' : u.update_available ? 'Update Available' : 'Up to Date'}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function IconDashboard() {
  return <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/>
    <rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/>
  </svg>;
}
function IconBrowser() {
  return <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/>
    <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
  </svg>;
}
function IconSettings() {
  return <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="3"/>
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
  </svg>;
}

export default App;
