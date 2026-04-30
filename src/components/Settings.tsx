import React from 'react';

interface SettingsProps {
  onClose: () => void;
}

export function Settings({ onClose }: SettingsProps) {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4 shadow-xl">
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-semibold">Settings</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white text-2xl leading-none"
          >
            ×
          </button>
        </div>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Excluded Paths
            </label>
            <textarea
              className="w-full bg-gray-900 border border-gray-700 rounded p-3 text-sm text-gray-300 focus:outline-none focus:border-blue-500"
              rows={4}
              placeholder="Enter paths to exclude from cleanup (one per line)"
              defaultValue={["/Users/*/Projects", "/home/*/work"].join('\n')}
            />
            <p className="text-xs text-gray-500 mt-1">
              These paths will never be cleaned
            </p>
          </div>

          <div className="flex items-center justify-between py-3 border-t border-gray-700">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Clear Clipboard
              </label>
              <p className="text-xs text-gray-500">Remove clipboard contents during cleanup</p>
            </div>
            <input type="checkbox" defaultChecked className="toggle" />
          </div>

          <div className="flex items-center justify-between py-3 border-t border-gray-700">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Reset Hosts File
              </label>
              <p className="text-xs text-gray-500">Restore hosts file to default</p>
            </div>
            <input type="checkbox" defaultChecked className="toggle" />
          </div>

          <div className="flex items-center justify-between py-3 border-t border-gray-700">
            <div>
              <label className="text-sm font-medium text-gray-300">
                Flush DNS Cache
              </label>
              <p className="text-xs text-gray-500">Clear system DNS cache</p>
            </div>
            <input type="checkbox" defaultChecked className="toggle" />
          </div>
        </div>

        <div className="mt-6 flex justify-end gap-3">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-300 hover:text-white transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={onClose}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
          >
            Save Changes
          </button>
        </div>
      </div>
    </div>
  );
}
