import React from 'react';
import { useCleanup } from '../hooks/useCleanup';
import type { BrowserCleanupResult, SystemCleanupResult, NetworkCleanupResult, ValidationResult, DryRunOperation } from '../types/ipc';

export function Dashboard() {
  const { isRunning, progress, summary, dryRunResult, osInfo, error, runDryRun, startCleanup, reset } = useCleanup();

  return (
    <div style={{ height: '100%', overflowY: 'auto', padding: 32 }}>
      <div style={{ maxWidth: 720, margin: '0 auto', display: 'flex', flexDirection: 'column', gap: 24 }}>

        {/* Header */}
        <div className="fade-up">
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 4 }}>
            <h1 style={{ margin: 0, fontSize: 22, fontWeight: 700, color: '#e2e8f0' }}>Dashboard</h1>
            {osInfo && (
              <span style={{
                fontSize: 11, padding: '4px 10px', borderRadius: 999,
                background: 'rgba(99,102,241,0.1)', color: '#818cf8',
                border: '1px solid rgba(99,102,241,0.2)', fontWeight: 500,
              }}>
                {osInfo.name}
              </span>
            )}
          </div>
          <p style={{ margin: 0, fontSize: 13, color: '#475569' }}>Reset your QA workstation to a clean baseline</p>
        </div>

        {/* Stat row */}
        <div className="fade-up" style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 12 }}>
          <StatCard
            icon={<IconGlobe />}
            label="Browsers"
            value={dryRunResult ? dryRunResult.operations.filter(o => o.name.includes('Cache')).length.toString() : '—'}
            sub="cache targets"
            color="#6366f1"
          />
          <StatCard
            icon={<IconHdd />}
            label="Estimated"
            value={dryRunResult ? formatBytes(dryRunResult.estimated_bytes_to_free) : '—'}
            sub="to free"
            color="#8b5cf6"
          />
          <StatCard
            icon={<IconFile />}
            label="Files"
            value={dryRunResult ? dryRunResult.estimated_files_to_remove.toLocaleString() : '—'}
            sub="to remove"
            color="#a855f7"
          />
        </div>

        {/* Main action card */}
        <div className="card fade-up" style={{ padding: 28 }}>
          {/* Idle */}
          {!isRunning && !dryRunResult && !summary && !error && (
            <div style={{ textAlign: 'center', padding: '24px 0' }}>
              <div style={{ position: 'relative', width: 72, height: 72, margin: '0 auto 20px' }}>
                <div style={{
                  position: 'absolute', inset: 0, borderRadius: '50%',
                  background: 'rgba(99,102,241,0.15)',
                }} className="pulse-ring" />
                <div style={{
                  width: 72, height: 72, borderRadius: '50%',
                  background: 'rgba(99,102,241,0.1)', border: '1px solid rgba(99,102,241,0.3)',
                  display: 'flex', alignItems: 'center', justifyContent: 'center',
                }}>
                  <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#818cf8" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                    <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
                  </svg>
                </div>
              </div>
              <h3 style={{ margin: '0 0 8px', fontSize: 16, fontWeight: 600, color: '#e2e8f0' }}>Ready to Scan</h3>
              <p style={{ margin: '0 0 24px', fontSize: 13, color: '#475569', maxWidth: 320, marginInline: 'auto' }}>
                Run a dry-run analysis to see what will be cleaned before committing to any changes.
              </p>
              <button className="btn-primary" onClick={runDryRun} style={{ paddingInline: 32 }}>
                Run Dry-Run Analysis
              </button>
            </div>
          )}

          {/* Running */}
          {isRunning && progress && (
            <div style={{ padding: '8px 0' }}>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                  <div style={{ width: 16, height: 16, border: '2px solid rgba(99,102,241,0.3)', borderTopColor: '#6366f1', borderRadius: '50%' }} className="spin" />
                  <span style={{ fontSize: 14, fontWeight: 600, color: '#e2e8f0' }}>{progress.phase}</span>
                </div>
                <span className="accent-text" style={{ fontSize: 13, fontWeight: 700 }}>
                  {Math.round(progress.progress * 100)}%
                </span>
              </div>
              <div className="progress-track" style={{ height: 6, marginBottom: 12 }}>
                <div className="progress-fill" style={{ height: '100%', width: `${progress.progress * 100}%` }} />
              </div>
              <p style={{ margin: 0, fontSize: 13, color: '#475569' }}>{progress.message}</p>
            </div>
          )}

          {/* Error */}
          {error && (
            <div style={{ padding: '16px 20px', borderRadius: 8, background: 'rgba(239,68,68,0.08)', border: '1px solid rgba(239,68,68,0.2)' }}>
              <div style={{ display: 'flex', alignItems: 'flex-start', gap: 12 }}>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#ef4444" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{ flexShrink: 0, marginTop: 1 }}>
                  <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
                <div style={{ flex: 1 }}>
                  <div style={{ fontSize: 13, color: '#f87171', fontWeight: 500 }}>{error}</div>
                  <button onClick={reset} style={{ marginTop: 8, fontSize: 12, color: '#6366f1', background: 'none', border: 'none', cursor: 'pointer', padding: 0 }}>
                    Try again →
                  </button>
                </div>
              </div>
            </div>
          )}

          {/* Dry-run result */}
          {dryRunResult && !summary && !isRunning && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: 12, padding: '14px 18px', borderRadius: 8, background: 'rgba(99,102,241,0.08)', border: '1px solid rgba(99,102,241,0.2)' }}>
                <div style={{ width: 8, height: 8, borderRadius: '50%' }} className="dot-blue" />
                <div style={{ flex: 1 }}>
                  <div style={{ fontSize: 13, fontWeight: 600, color: '#e2e8f0' }}>Analysis Complete</div>
                  <div style={{ fontSize: 12, color: '#6366f1', marginTop: 2 }}>
                    {dryRunResult.estimated_files_to_remove.toLocaleString()} files · {formatBytes(dryRunResult.estimated_bytes_to_free)} to free
                  </div>
                </div>
              </div>

              {dryRunResult.warnings.length > 0 && (
                <div style={{ padding: '12px 16px', borderRadius: 8, background: 'rgba(234,179,8,0.06)', border: '1px solid rgba(234,179,8,0.15)' }}>
                  <div style={{ fontSize: 12, fontWeight: 600, color: '#eab308', marginBottom: 6 }}>Warnings</div>
                  {dryRunResult.warnings.map((w, i) => (
                    <div key={i} style={{ fontSize: 12, color: '#ca8a04', lineHeight: 1.6 }}>· {w}</div>
                  ))}
                </div>
              )}

              <div style={{ display: 'flex', gap: 10 }}>
                <button className="btn-primary" onClick={startCleanup}>Confirm & Execute Cleanup</button>
                <button className="btn-ghost" onClick={reset}>Cancel</button>
              </div>
            </div>
          )}

          {/* Success */}
          {summary && !isRunning && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: 12, padding: '14px 18px', borderRadius: 8, background: 'rgba(34,197,94,0.08)', border: '1px solid rgba(34,197,94,0.2)' }}>
                <div style={{ width: 8, height: 8, borderRadius: '50%', flexShrink: 0 }} className="dot-green" />
                <div style={{ flex: 1 }}>
                  <div style={{ fontSize: 13, fontWeight: 600, color: '#e2e8f0' }}>Cleanup Complete</div>
                  <div style={{ fontSize: 12, color: '#475569', marginTop: 2, fontFamily: 'monospace' }}>{summary.log_path}</div>
                </div>
              </div>
              <div>
                <button className="btn-ghost" onClick={reset}>Run Another Cleanup</button>
              </div>
            </div>
          )}
        </div>

        {/* Operations breakdown (after dry-run) */}
        {dryRunResult && (
          <div className="card fade-up" style={{ padding: 24 }}>
            <h3 style={{ margin: '0 0 16px', fontSize: 14, fontWeight: 600, color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.08em' }}>
              Operations
            </h3>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
              {dryRunResult.operations.map((op, i) => (
                <OperationRow key={i} op={op} />
              ))}
            </div>
          </div>
        )}

        {/* Results (after cleanup) */}
        {summary && (
          <>
            <ResultsGrid
              title="Browser Cleanup"
              icon={<IconGlobe />}
              items={summary.browser_results.map(r => ({
                label: r.browser,
                ok: r.cache_cleared,
                detail: r.error || (r.items_removed ? `${r.items_removed} files removed` : 'Nothing to clear'),
              }))}
            />
            <ResultsGrid
              title="System Cleanup"
              icon={<IconHdd />}
              items={summary.system_results.map(r => ({
                label: r.category,
                ok: r.cleared,
                detail: r.error || (r.items_removed ? `${r.items_removed} files, ${formatBytes(r.bytes_freed)}` : 'Nothing to clear'),
              }))}
            />
            <ResultsGrid
              title="Network Reset"
              icon={<IconNetwork />}
              items={summary.network_results.map(r => ({
                label: r.operation,
                ok: r.success,
                detail: r.error || 'Success',
              }))}
            />
            <ResultsGrid
              title="Validation"
              icon={<IconCheck />}
              items={summary.validation_results.map(r => ({
                label: r.check,
                ok: r.passed,
                detail: r.details || (r.passed ? 'Passed' : 'Failed'),
              }))}
            />
          </>
        )}
      </div>
    </div>
  );
}

function StatCard({ icon, label, value, sub, color }: { icon: React.ReactNode; label: string; value: string; sub: string; color: string }) {
  return (
    <div className="card card-hover" style={{ padding: '16px 18px' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
        <span style={{ fontSize: 12, color: '#475569', fontWeight: 500 }}>{label}</span>
        <span style={{ color, opacity: 0.7 }}>{icon}</span>
      </div>
      <div style={{ fontSize: 22, fontWeight: 700, color: '#e2e8f0', lineHeight: 1 }}>{value}</div>
      <div style={{ fontSize: 11, color: '#475569', marginTop: 4 }}>{sub}</div>
    </div>
  );
}

function OperationRow({ op }: { op: DryRunOperation }) {
  const hasData = op.file_count > 0;
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 12, padding: '8px 0', borderBottom: '1px solid rgba(255,255,255,0.04)' }}>
      <div style={{ width: 6, height: 6, borderRadius: '50%', flexShrink: 0 }} className={hasData ? 'dot-blue' : 'dot-gray'} />
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{ fontSize: 13, color: '#cbd5e1', fontWeight: 500 }}>{op.name}</div>
        <div style={{ fontSize: 11, color: '#334155', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }} title={op.path}>{op.path}</div>
      </div>
      <div style={{ textAlign: 'right', flexShrink: 0 }}>
        {hasData ? (
          <>
            <div style={{ fontSize: 12, color: '#6366f1', fontWeight: 600 }}>{formatBytes(op.bytes)}</div>
            <div style={{ fontSize: 11, color: '#334155' }}>{op.file_count.toLocaleString()} files</div>
          </>
        ) : (
          <span style={{ fontSize: 11, color: '#334155' }}>{op.would_modify ? 'Will modify' : 'Empty'}</span>
        )}
      </div>
    </div>
  );
}

function ResultsGrid({ title, icon, items }: {
  title: string;
  icon: React.ReactNode;
  items: Array<{ label: string; ok: boolean; detail: string }>;
}) {
  return (
    <div className="card fade-up" style={{ padding: 24 }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 16 }}>
        <span style={{ color: '#475569' }}>{icon}</span>
        <h3 style={{ margin: 0, fontSize: 14, fontWeight: 600, color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.08em' }}>{title}</h3>
      </div>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
        {items.map((item, i) => (
          <div key={i} style={{ display: 'flex', alignItems: 'center', gap: 12, padding: '8px 12px', borderRadius: 6, background: 'rgba(255,255,255,0.02)' }}>
            <div style={{ width: 6, height: 6, borderRadius: '50%', flexShrink: 0 }} className={item.ok ? 'dot-green' : 'dot-red'} />
            <div style={{ flex: 1 }}>
              <span style={{ fontSize: 13, color: '#cbd5e1', fontWeight: 500 }}>{item.label}</span>
              {item.detail && <span style={{ fontSize: 12, color: '#475569', marginLeft: 8 }}>{item.detail}</span>}
            </div>
            <span style={{
              fontSize: 11, padding: '2px 8px', borderRadius: 999, fontWeight: 600,
              background: item.ok ? 'rgba(34,197,94,0.1)' : 'rgba(239,68,68,0.1)',
              color: item.ok ? '#22c55e' : '#ef4444',
            }}>
              {item.ok ? 'Done' : 'Failed'}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
}

function IconGlobe() {
  return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/>
    <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/>
  </svg>;
}
function IconHdd() {
  return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <line x1="22" y1="12" x2="2" y2="12"/>
    <path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>
    <line x1="6" y1="16" x2="6.01" y2="16"/><line x1="10" y1="16" x2="10.01" y2="16"/>
  </svg>;
}
function IconFile() {
  return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
    <polyline points="13 2 13 9 20 9"/>
  </svg>;
}
function IconNetwork() {
  return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <rect x="2" y="2" width="6" height="6" rx="1"/><rect x="16" y="2" width="6" height="6" rx="1"/>
    <rect x="9" y="16" width="6" height="6" rx="1"/>
    <path d="M5 8v4c0 1.1.9 2 2 2h10a2 2 0 0 0 2-2V8"/><line x1="12" y1="14" x2="12" y2="16"/>
  </svg>;
}
function IconCheck() {
  return <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <polyline points="9 11 12 14 22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/>
  </svg>;
}
