export interface CleanupProgress {
  phase: string;
  progress: number;
  message: string;
  details?: string;
}

export interface BrowserCleanupResult {
  browser: string;
  cache_cleared: boolean;
  cookies_cleared: boolean;
  local_storage_cleared: boolean;
  items_removed: number;
  error?: string;
}

export interface SystemCleanupResult {
  category: string;
  cleared: boolean;
  items_removed: number;
  bytes_freed: number;
  error?: string;
}

export interface NetworkCleanupResult {
  operation: string;
  success: boolean;
  error?: string;
}

export interface ValidationResult {
  check: string;
  passed: boolean;
  details?: string;
}

export interface CleanupSummary {
  browser_results: BrowserCleanupResult[];
  system_results: SystemCleanupResult[];
  network_results: NetworkCleanupResult[];
  validation_results: ValidationResult[];
  log_path: string;
}

export interface OSInfo {
  name: string;
  temp_dir: string;
  hosts_file: string;
}
