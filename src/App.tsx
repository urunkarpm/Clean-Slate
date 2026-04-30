import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Dashboard } from './components/Dashboard';
import { Settings } from './components/Settings';
import type { BrowserUpdateInfo } from './types/ipc';

function App() {
  const [showSettings, setShowSettings] = useState(false);
  const [showBrowserUpdates, setShowBrowserUpdates] = useState(false);

  return (
    <div className="relative">
      <Dashboard />

      {/* Settings Button */}
      <button
        onClick={() => setShowSettings(true)}
        className="fixed top-4 right-4 p-3 bg-gray-800 hover:bg-gray-700 rounded-full shadow-lg transition-colors z-40"
        aria-label="Settings"
      >
        <svg
          className="w-6 h-6 text-gray-300"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
          />
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
      </button>

      {/* Browser Updates Button */}
      <button
        onClick={() => setShowBrowserUpdates(true)}
        className="fixed top-4 right-20 p-3 bg-gray-800 hover:bg-gray-700 rounded-full shadow-lg transition-colors z-40"
        aria-label="Check Browser Updates"
      >
        <svg
          className="w-6 h-6 text-gray-300"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>
      </button>

      {/* Settings Modal */}
      {showSettings && (
        <Settings onClose={() => setShowSettings(false)} />
      )}

      {/* Browser Updates Modal */}
      {showBrowserUpdates && (
        <BrowserUpdatesModal onClose={() => setShowBrowserUpdates(false)} />
      )}
    </div>
  );
}

function BrowserUpdatesModal({ onClose }: { onClose: () => void }) {
  const [updates, setUpdates] = useState<BrowserUpdateInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const checkUpdates = async () => {
      try {
        const result = await invoke<BrowserUpdateInfo[]>('check_browser_updates');
        setUpdates(result);
      } catch (err) {
        setError(`Failed to check updates: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    checkUpdates();
  }, []);

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4 shadow-xl">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-semibold">Browser Updates</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white text-2xl leading-none"
          >
            ×
          </button>
        </div>

        {loading ? (
          <div className="py-8 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 mx-auto"></div>
            <p className="text-gray-400 text-sm mt-4">Checking for browser updates...</p>
          </div>
        ) : error ? (
          <div className="bg-red-900/30 border border-red-700 rounded-lg p-4">
            <p className="text-red-300 text-sm">{error}</p>
          </div>
        ) : (
          <div className="space-y-3">
            {updates.length === 0 ? (
              <p className="text-gray-400 text-sm text-center py-4">No browsers detected</p>
            ) : (
              updates.map((update, idx) => (
                <div
                  key={idx}
                  className={`rounded-lg p-4 border ${
                    update.error
                      ? 'bg-gray-700/50 border-gray-600'
                      : update.update_available
                      ? 'bg-yellow-900/30 border-yellow-700'
                      : 'bg-green-900/30 border-green-700'
                  }`}
                >
                  <div className="flex justify-between items-start">
                    <div>
                      <h3 className="font-semibold text-white">{update.name}</h3>
                      {update.error ? (
                        <p className="text-red-400 text-sm mt-1">{update.error}</p>
                      ) : (
                        <>
                          <p className="text-gray-400 text-sm mt-1">
                            Current: {update.current_version}
                          </p>
                          {update.latest_version && (
                            <p className="text-gray-400 text-sm">
                              Latest: {update.latest_version}
                            </p>
                          )}
                        </>
                      )}
                    </div>
                    <span
                      className={`text-sm px-2 py-1 rounded ${
                        update.error
                          ? 'bg-gray-600 text-gray-300'
                          : update.update_available
                          ? 'bg-yellow-600 text-yellow-100'
                          : 'bg-green-600 text-green-100'
                      }`}
                    >
                      {update.error
                        ? 'Unknown'
                        : update.update_available
                        ? 'Update Available'
                        : 'Up to Date'}
                    </span>
                  </div>
                </div>
              ))
            )}
          </div>
        )}

        <div className="mt-6 flex justify-end">
          <button
            onClick={onClose}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
