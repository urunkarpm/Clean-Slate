import React from 'react';
import { useCleanup } from '../hooks/useCleanup';

export function Dashboard() {
  const { isRunning, progress, summary, dryRunResult, osInfo, error, runDryRun, startCleanup, reset } = useCleanup();

  return (
    <div className="min-h-screen bg-gray-900 text-white p-8">
      <div className="max-w-4xl mx-auto">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-blue-400">CleanSlate QA v2.0</h1>
          <p className="text-gray-400 mt-2">
            Reset your QA workstation to a clean baseline in one click
          </p>
          {osInfo && (
            <p className="text-sm text-gray-500 mt-1">
              Running on: {osInfo.name}
            </p>
          )}
        </header>

        <main className="space-y-6">
          {/* Action Card */}
          <div className="bg-gray-800 rounded-lg p-6 shadow-lg">
            <h2 className="text-xl font-semibold mb-4">Cleanup Status</h2>
            
            {!isRunning && !dryRunResult && !summary && !error && (
              <div className="text-center py-8">
                <p className="text-gray-400 mb-4">Ready to analyze your system</p>
                <button
                  onClick={runDryRun}
                  className="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                >
                  Run Dry-Run Analysis
                </button>
              </div>
            )}

            {isRunning && progress && (
              <div className="py-4">
                <div className="mb-2 flex justify-between text-sm">
                  <span className="text-gray-300">{progress.phase}</span>
                  <span className="text-blue-400">{Math.round(progress.progress * 100)}%</span>
                </div>
                <div className="w-full bg-gray-700 rounded-full h-3 overflow-hidden">
                  <div
                    className="bg-blue-500 h-full transition-all duration-300"
                    style={{ width: `${progress.progress * 100}%` }}
                  />
                </div>
                <p className="text-gray-400 text-sm mt-3">{progress.message}</p>
              </div>
            )}

            {error && (
              <div className="bg-red-900/50 border border-red-700 rounded-lg p-4">
                <p className="text-red-300">{error}</p>
                <button
                  onClick={reset}
                  className="mt-3 text-red-400 hover:text-red-300 underline text-sm"
                >
                  Try Again
                </button>
              </div>
            )}

            {dryRunResult && !summary && (
              <div className="space-y-4">
                <div className="bg-yellow-900/30 border border-yellow-700 rounded-lg p-4">
                  <h3 className="text-yellow-400 font-semibold">⚠ Dry-Run Complete</h3>
                  <p className="text-yellow-300 text-sm mt-1">
                    Estimated: {dryRunResult.estimated_files_to_remove.toLocaleString()} files ({(dryRunResult.estimated_bytes_to_free / 1024 / 1024).toFixed(2)} MB) will be removed
                  </p>
                </div>
                
                {dryRunResult.warnings.length > 0 && (
                  <div className="bg-orange-900/30 border border-orange-700 rounded-lg p-4">
                    <h4 className="text-orange-400 font-semibold mb-2">Warnings:</h4>
                    <ul className="text-orange-300 text-sm list-disc list-inside">
                      {dryRunResult.warnings.map((warning, idx) => (
                        <li key={idx}>{warning}</li>
                      ))}
                    </ul>
                  </div>
                )}
                
                <div className="flex gap-3">
                  <button
                    onClick={startCleanup}
                    className="bg-green-600 hover:bg-green-700 text-white font-semibold py-3 px-8 rounded-lg transition-colors"
                  >
                    Confirm & Execute Cleanup
                  </button>
                  <button
                    onClick={reset}
                    className="bg-gray-700 hover:bg-gray-600 text-white font-semibold py-3 px-6 rounded-lg transition-colors"
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}

            {summary && (
              <div className="space-y-4">
                <div className="bg-green-900/30 border border-green-700 rounded-lg p-4">
                  <h3 className="text-green-400 font-semibold">✓ Cleanup Complete</h3>
                  <p className="text-green-300 text-sm mt-1">
                    Log saved to: {summary.log_path}
                  </p>
                </div>
                
                <button
                  onClick={reset}
                  className="bg-gray-700 hover:bg-gray-600 text-white font-semibold py-2 px-6 rounded-lg transition-colors"
                >
                  Run Another Cleanup
                </button>
              </div>
            )}
          </div>

          {/* Results Summary */}
          {summary && (
            <>
              <ResultsSection 
                title="Browser Cleanup" 
                results={summary.browser_results}
                type="browser"
              />
              <ResultsSection 
                title="System Cleanup" 
                results={summary.system_results}
                type="system"
              />
              <ResultsSection 
                title="Network Reset" 
                results={summary.network_results}
                type="network"
              />
              <ValidationSection results={summary.validation_results} />
            </>
          )}
        </main>
      </div>
    </div>
  );
}

interface ResultItem {
  browser?: string;
  category?: string;
  operation?: string;
  cache_cleared?: boolean;
  cleared?: boolean;
  success?: boolean;
  items_removed?: number;
  error?: string;
}

function ResultsSection({ title, results, type }: { 
  title: string; 
  results: ResultItem[]; 
  type: 'browser' | 'system' | 'network';
}) {
  return (
    <div className="bg-gray-800 rounded-lg p-6 shadow-lg">
      <h3 className="text-lg font-semibold mb-4">{title}</h3>
      <div className="space-y-2">
        {results.map((result, idx) => (
          <div key={idx} className="flex justify-between items-center py-2 border-b border-gray-700 last:border-0">
            <span className="text-gray-300">
              {type === 'browser' && result.browser}
              {type === 'system' && result.category}
              {type === 'network' && result.operation}
            </span>
            <span className={`text-sm ${
              (result.cache_cleared || result.cleared || result.success) 
                ? 'text-green-400' 
                : 'text-yellow-400'
            }`}>
              {(result.cache_cleared || result.cleared || result.success) ? '✓ Done' : '⚠ Skipped'}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}

function ValidationSection({ results }: { results: Array<{ check: string; passed: boolean; details?: string }> }) {
  return (
    <div className="bg-gray-800 rounded-lg p-6 shadow-lg">
      <h3 className="text-lg font-semibold mb-4">Validation Results</h3>
      <div className="space-y-2">
        {results.map((result, idx) => (
          <div key={idx} className="flex justify-between items-center py-2 border-b border-gray-700 last:border-0">
            <span className="text-gray-300">{result.check}</span>
            <span className={`text-sm ${result.passed ? 'text-green-400' : 'text-red-400'}`}>
              {result.passed ? '✓ Passed' : '✗ Failed'}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
