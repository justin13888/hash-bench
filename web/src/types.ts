export interface BenchmarkResult {
	platform: string;
	threads: number;
	algorithm: string;
	/**
	 * Implementation tag — `"sw"` for pure-Rust, `"sha-ext"` for x86 SHA-NI /
	 * ARMv8 SHA2, `"aes-ext"` for AES-NI, `"clmul"` for PCLMULQDQ / PMULL,
	 * `"crc-ext"` for SSE4.2 / ARMv8 CRC. Together with `algorithm`, it forms
	 * the unique result identity.
	 */
	variant: string;
	size: string;
	size_bytes: number;
	mean_ns: number;
	median_ns: number;
	stddev_ns: number;
	ci_lower_ns: number;
	ci_upper_ns: number;
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

export interface AlgorithmMeta {
	name: string;
	variant: string;
	crate: string;
	output_bits: number;
	output_kind: "fixed" | "xof";
	category: "cryptographic" | "non-cryptographic";
	internally_parallel: boolean;
	keyed: boolean;
	dos_resistant: boolean;
	hardware_required: boolean;
	hardware_features: string[];
	notes: string;
}

export interface ReportData {
	generated_at_unix_ms: number;
	platforms: PlatformInfo[];
	benchmarks: BenchmarkResult[];
	/** Keyed by `<algorithm>|<variant>`. */
	algorithms: Record<string, AlgorithmMeta>;
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
