import React, { useState, useEffect } from 'react';

interface LogViewerProps {
  logPath?: string;
}

export function LogViewer({ logPath }: LogViewerProps) {
  const [logs, setLogs] = useState<Array<{
    id: string;
    timestamp: string;
    level: string;
    message: string;
    details?: unknown;
  }>>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (logPath) {
      loadLogs();
    }
  }, [logPath]);

  const loadLogs = async () => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would read the file via IPC
      // For now, we'll show a placeholder
      setLogs([]);
    } catch (err) {
      setError('Failed to load logs');
    } finally {
      setLoading(false);
    }
  };

  if (!logPath) {
    return null;
  }

  return (
    <div className="bg-gray-800 rounded-lg p-6 shadow-lg">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-lg font-semibold">Cleanup Log</h3>
        <button
          onClick={loadLogs}
          disabled={loading}
          className="text-sm text-blue-400 hover:text-blue-300 disabled:opacity-50"
        >
          {loading ? 'Loading...' : 'Refresh'}
        </button>
      </div>

      {error && (
        <div className="bg-red-900/30 border border-red-700 rounded p-3 mb-4">
          <p className="text-red-300 text-sm">{error}</p>
        </div>
      )}

      <div className="bg-gray-900 rounded p-4 font-mono text-xs max-h-64 overflow-y-auto">
        {logs.length === 0 ? (
          <p className="text-gray-500">No log entries yet. Log file: {logPath}</p>
        ) : (
          <ul className="space-y-2">
            {logs.map((log) => (
              <li key={log.id} className="flex gap-3">
                <span className="text-gray-500 shrink-0">
                  {new Date(parseInt(log.timestamp.split('.')[0]) * 1000).toLocaleTimeString()}
                </span>
                <span className={`shrink-0 w-16 ${
                  log.level === 'ERROR' ? 'text-red-400' :
                  log.level === 'WARN' ? 'text-yellow-400' :
                  'text-green-400'
                }`}>
                  [{log.level}]
                </span>
                <span className="text-gray-300">{log.message}</span>
              </li>
            ))}
          </ul>
        )}
      </div>

      <div className="mt-4 flex gap-3">
        <button
          onClick={() => navigator.clipboard.writeText(logPath)}
          className="text-sm text-gray-400 hover:text-gray-300 underline"
        >
          Copy Log Path
        </button>
        <a
          href={`file://${logPath}`}
          className="text-sm text-blue-400 hover:text-blue-300 underline"
          onClick={(e) => e.preventDefault()}
        >
          Open in Editor
        </a>
      </div>
    </div>
  );
}
