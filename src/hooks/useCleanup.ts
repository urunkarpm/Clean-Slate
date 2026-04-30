import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { CleanupSummary, OSInfo, CleanupProgress } from '../types/ipc';

interface UseCleanupReturn {
  isRunning: boolean;
  progress: CleanupProgress | null;
  summary: CleanupSummary | null;
  osInfo: OSInfo | null;
  error: string | null;
  startCleanup: () => Promise<void>;
  reset: () => void;
}

export function useCleanup(): UseCleanupReturn {
  const [isRunning, setIsRunning] = useState(false);
  const [progress, setProgress] = useState<CleanupProgress | null>(null);
  const [summary, setSummary] = useState<CleanupSummary | null>(null);
  const [osInfo, setOsInfo] = useState<OSInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadOSInfo = useCallback(async () => {
    try {
      const info = await invoke<OSInfo>('get_os_info');
      setOsInfo(info);
    } catch (err) {
      setError(`Failed to load OS info: ${err}`);
    }
  }, []);

  useEffect(() => {
    loadOSInfo();
  }, [loadOSInfo]);

  const startCleanup = async () => {
    setIsRunning(true);
    setError(null);
    setSummary(null);
    
    // Simulate progress updates
    const phases = [
      { phase: 'Initializing', progress: 0.1, message: 'Preparing cleanup...' },
      { phase: 'Browser Cleanup', progress: 0.25, message: 'Clearing browser caches...' },
      { phase: 'System Cleanup', progress: 0.5, message: 'Cleaning system files...' },
      { phase: 'Network Reset', progress: 0.75, message: 'Resetting network settings...' },
      { phase: 'Validation', progress: 0.9, message: 'Validating cleanup...' },
      { phase: 'Complete', progress: 1.0, message: 'Cleanup complete!' },
    ];

    for (const p of phases) {
      setProgress(p);
      await new Promise(resolve => setTimeout(resolve, 500));
    }

    try {
      const result = await invoke<CleanupSummary>('start_cleanup');
      setSummary(result);
    } catch (err) {
      setError(`Cleanup failed: ${err}`);
    } finally {
      setIsRunning(false);
    }
  };

  const reset = () => {
    setIsRunning(false);
    setProgress(null);
    setSummary(null);
    setError(null);
  };

  return {
    isRunning,
    progress,
    summary,
    osInfo,
    error,
    startCleanup,
    reset,
  };
}
