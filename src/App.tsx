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
        className="fixed top-6 right-6 p-4 glass-card rounded-xl shadow-lg transition-all hover-lift z-40 group"
        aria-label="Settings"
      >
        <svg
          className="w-6 h-6 text-gray-300 group-hover:text-white transition-colors"
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
        className="fixed top-6 right-24 p-4 glass-card rounded-xl shadow-lg transition-all hover-lift z-40 group"
        aria-label="Check Browser Updates"
      >
        <svg
          className="w-6 h-6 text-gray-300 group-hover:text-white transition-colors"
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
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="glass-card rounded-2xl p-8 max-w-md w-full mx-4 shadow-2xl slide-in">
        <div className="flex justify-between items-center mb-8">
          <h2 className="text-2xl font-bold gradient-text">Browser Updates</h2>
          <button
            onClick={onClose}
            className="w-10 h-10 rounded-xl glass flex items-center justify-center text-gray-400 hover:text-white transition-all hover:bg-white/10"
          >
            <span className="text-2xl leading-none">×</span>
          </button>
        </div>

        {loading ? (
          <div className="py-12 text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-4 border-indigo-500/30 border-t-indigo-500 mx-auto"></div>
            <p className="text-gray-400 text-sm mt-6">Checking for browser updates...</p>
          </div>
        ) : error ? (
          <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-6">
            <p className="text-red-300 text-sm">{error}</p>
          </div>
        ) : (
          <div className="space-y-4">
            {updates.length === 0 ? (
              <p className="text-gray-400 text-sm text-center py-8">No browsers detected</p>
            ) : (
              updates.map((update, idx) => (
                <div
                  key={idx}
                  className={`rounded-xl p-5 border transition-all hover-lift ${
                    update.error
                      ? 'bg-gray-500/10 border-gray-500/30'
                      : update.update_available
                      ? 'bg-yellow-500/10 border-yellow-500/30'
                      : 'bg-green-500/10 border-green-500/30'
                  }`}
                >
                  <div className="flex justify-between items-start">
                    <div>
                      <h3 className="font-semibold text-white text-lg">{update.name}</h3>
                      {update.error ? (
                        <p className="text-red-400 text-sm mt-2">{update.error}</p>
                      ) : (
                        <>
                          <p className="text-gray-400 text-sm mt-2">
                            Current: <span className="text-white">{update.current_version}</span>
                          </p>
                          {update.latest_version && (
                            <p className="text-gray-400 text-sm">
                              Latest: <span className="text-white">{update.latest_version}</span>
                            </p>
                          )}
                        </>
                      )}
                    </div>
                    <span
                      className={`text-sm px-3 py-1.5 rounded-full font-medium ${
                        update.error
                          ? 'bg-gray-500/20 text-gray-300'
                          : update.update_available
                          ? 'bg-yellow-500/20 text-yellow-300'
                          : 'bg-green-500/20 text-green-300'
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

        <div className="mt-8 flex justify-end">
          <button
            onClick={onClose}
            className="px-8 py-3 bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-indigo-600 hover:to-purple-700 text-white rounded-xl transition-all hover-lift shadow-lg shadow-indigo-500/30"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
