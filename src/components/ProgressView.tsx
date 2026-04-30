import React from 'react';
import type { CleanupProgress } from '../types/ipc';

interface ProgressViewProps {
  progress: CleanupProgress;
}

export function ProgressView({ progress }: ProgressViewProps) {
  return (
    <div className="w-full">
      <div className="flex justify-between items-center mb-2">
        <span className="text-lg font-medium text-gray-700">{progress.phase}</span>
        <span className="text-sm text-blue-600 font-semibold">
          {Math.round(progress.progress * 100)}%
        </span>
      </div>
      
      <div className="w-full bg-gray-200 rounded-full h-4 overflow-hidden shadow-inner">
        <div
          className="bg-gradient-to-r from-blue-500 to-blue-600 h-full transition-all duration-500 ease-out"
          style={{ width: `${progress.progress * 100}%` }}
        >
          <div className="w-full h-full animate-pulse bg-white/20" />
        </div>
      </div>
      
      <p className="mt-3 text-sm text-gray-600">{progress.message}</p>
      
      {progress.details && (
        <p className="mt-1 text-xs text-gray-500">{progress.details}</p>
      )}
    </div>
  );
}
