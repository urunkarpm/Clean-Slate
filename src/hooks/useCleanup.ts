import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { CleanupSummary, OSInfo, CleanupProgress, DryRunResult } from '../types/ipc';

interface UseCleanupReturn {
  isRunning: boolean;
  progress: CleanupProgress | null;
  summary: CleanupSummary | null;
  dryRunResult: DryRunResult | null;
  osInfo: OSInfo | null;
  error: string | null;
  runDryRun: () => Promise<void>;
  startCleanup: () => Promise<void>;
  reset: () => void;
}

export function useCleanup(): UseCleanupReturn {
  const [isRunning, setIsRunning] = useState(false);
  const [progress, setProgress] = useState<CleanupProgress | null>(null);
  const [summary, setSummary] = useState<CleanupSummary | null>(null);
  const [dryRunResult, setDryRunResult] = useState<DryRunResult | null>(null);
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

  const runDryRun = async () => {
    setIsRunning(true);
    setError(null);
    setDryRunResult(null);
    
    // Simulate progress updates
    const phases = [
      { phase: 'Analyzing', progress: 0.25, message: 'Scanning browser caches...' },
      { phase: 'Analyzing', progress: 0.5, message: 'Scanning system files...' },
      { phase: 'Analyzing', progress: 0.75, message: 'Checking network operations...' },
      { phase: 'Complete', progress: 1.0, message: 'Analysis complete!' },
    ];

    for (const p of phases) {
      setProgress(p);
      await new Promise(resolve => setTimeout(resolve, 300));
    }

    try {
      const result = await invoke<DryRunResult>('run_dry_run');
      setDryRunResult(result);
    } catch (err) {
      setError(`Dry-run failed: ${err}`);
    } finally {
      setIsRunning(false);
    }
  };

  const startCleanup = async () => {
    if (!dryRunResult) {
      setError('Please run dry-run first');
      return;
    }
    
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
      const result = await invoke<CleanupSummary>('start_cleanup', { dryRunConfirmed: true });
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
    setDryRunResult(null);
    setError(null);
  };

  return {
    isRunning,
    progress,
    summary,
    dryRunResult,
    osInfo,
    error,
    runDryRun,
    startCleanup,
    reset,
  };
}
