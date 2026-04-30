import React from 'react';

interface SettingsProps {
  onClose: () => void;
}

export function Settings({ onClose }: SettingsProps) {
  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="glass-card rounded-2xl p-8 max-w-md w-full mx-4 shadow-2xl slide-in">
        <div className="flex justify-between items-center mb-8">
          <h2 className="text-2xl font-bold gradient-text">Settings</h2>
          <button
            onClick={onClose}
            className="w-10 h-10 rounded-xl glass flex items-center justify-center text-gray-400 hover:text-white transition-all hover:bg-white/10"
          >
            <span className="text-2xl leading-none">×</span>
          </button>
        </div>

        <div className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-3">
              Excluded Paths
            </label>
            <textarea
              className="w-full glass border border-white/10 rounded-xl p-4 text-sm text-gray-300 focus:outline-none focus:border-indigo-500/50 focus:ring-2 focus:ring-indigo-500/20 transition-all bg-transparent"
              rows={4}
              placeholder="Enter paths to exclude from cleanup (one per line)"
              defaultValue={["/Users/*/Projects", "/home/*/work"].join('\n')}
            />
            <p className="text-xs text-gray-500 mt-2">
              These paths will never be cleaned
            </p>
          </div>

          <div className="flex items-center justify-between py-4 px-4 rounded-xl glass hover:bg-white/5 transition-all">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Clear Clipboard
              </label>
              <p className="text-xs text-gray-500 mt-1">Remove clipboard contents during cleanup</p>
            </div>
            <input type="checkbox" defaultChecked className="toggle" />
          </div>

          <div className="flex items-center justify-between py-4 px-4 rounded-xl glass hover:bg-white/5 transition-all">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Reset Hosts File
              </label>
              <p className="text-xs text-gray-500 mt-1">Restore hosts file to default</p>
            </div>
            <input type="checkbox" defaultChecked className="toggle" />
          </div>

          <div className="flex items-center justify-between py-4 px-4 rounded-xl glass hover:bg-white/5 transition-all">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Flush DNS Cache
              </label>
              <p className="text-xs text-gray-500 mt-1">Clear system DNS cache</p>
            </div>
            <input type="checkbox" defaultChecked className="toggle" />
          </div>
        </div>

        <div className="mt-8 flex justify-end gap-3">
          <button
            onClick={onClose}
            className="px-6 py-3 glass rounded-xl text-gray-300 hover:text-white hover:bg-white/10 transition-all"
          >
            Cancel
          </button>
          <button
            onClick={onClose}
            className="px-8 py-3 bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-indigo-600 hover:to-purple-700 text-white rounded-xl transition-all hover-lift shadow-lg shadow-indigo-500/30"
          >
            Save Changes
          </button>
        </div>
      </div>
    </div>
  );
}
