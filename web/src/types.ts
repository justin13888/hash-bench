export interface BenchmarkResult {
	platform: string;
	threads: number;
	algorithm: string;
	size: string;
	size_bytes: number;
	mean_ns: number;
	median_ns: number;
	stddev_ns: number;
	mean_lower_ns: number;
	mean_upper_ns: number;
	throughput_bps: number;
}

export interface PlatformInfo {
	id: string;
	display_name: string;
	cpu_model: string | null;
	physical_cores: number;
	logical_cores: number;
	os: string;
	arch: string;
	thread_counts: number[];
	sizes: string[];
}

export interface ReportData {
	generated_at_unix_ms: number;
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
