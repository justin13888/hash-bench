export interface BenchmarkResult {
  platform: string;
  thread_count: number;
  algorithm: string;
  size: string;
  size_bytes: number;
  mean_ns: number;
  median_ns: number;
  mean_lower_ns: number;
  mean_upper_ns: number;
  throughput_bps: number;
}

export interface PlatformInfo {
  id: string;
  display_name: string;
  thread_counts: number[];
  sizes: string[];
}

export interface ReportData {
  generated_at: string;
  platforms: PlatformInfo[];
  benchmarks: BenchmarkResult[];
  categories: Record<string, string>;
}

export type Metric = "throughput" | "latency";
export type CategoryFilter = "all" | "cryptographic" | "non-cryptographic";

export interface FilterState {
  selectedPlatforms: Set<string>;
  threadCount: number;
  size: string;
  category: CategoryFilter;
  metric: Metric;
  logScale: boolean;
}
