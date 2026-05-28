import type { AlgorithmMeta, BenchmarkResult, FilterState } from "../types";
import { algoKey } from "./format";

/** Coordinate-axis dimensions that views may opt out of (sweep instead). */
export interface CoordinateOptions {
	platform?: boolean;
	threads?: boolean;
	size?: boolean;
}

const ALL_COORDS: Required<CoordinateOptions> = {
	platform: true,
	threads: true,
	size: true,
};

/**
 * Apply all FilterState dimensions to the benchmark dataset. Coordinate
 * dimensions (platform/threads/size) and algorithm-slice dimensions
 * (category/variants/output_bits/keyed/...) are combined with AND.
 *
 * Views that need to sweep one or more coordinate axes (heatmap sweeps size,
 * thread-line sweeps threads, winners-summary sweeps both) can pass
 * `coordinates: { size: false }` etc. to skip that axis. Algorithm-slice
 * filters always apply.
 *
 * Rows whose algorithm key is missing from the catalogue are kept and treated
 * as "unknown metadata" — they will only be filtered out when a non-permissive
 * filter is set on a field that requires metadata.
 */
export function applyFilters(
	benchmarks: BenchmarkResult[],
	algorithms: Record<string, AlgorithmMeta>,
	filters: FilterState,
	coordinates: CoordinateOptions = ALL_COORDS,
): BenchmarkResult[] {
	const useCoords = { ...ALL_COORDS, ...coordinates };
	return benchmarks.filter((b) => {
		if (useCoords.platform && !filters.selectedPlatforms.has(b.platform))
			return false;
		if (useCoords.threads && b.threads !== filters.threadCount) return false;
		if (useCoords.size && b.size !== filters.size) return false;

		const meta = algorithms[algoKey(b.algorithm, b.variant)];

		if (filters.category !== "all") {
			if (!meta || meta.category !== filters.category) return false;
		}

		if (filters.variants.size > 0 && !filters.variants.has(b.variant)) {
			return false;
		}

		if (filters.hwAcceleration !== "all") {
			if (!meta) return false;
			const wantsHw = filters.hwAcceleration === "hw-only";
			if (meta.hardware_required !== wantsHw) return false;
		}

		if (filters.outputBits.size > 0) {
			if (!meta || !filters.outputBits.has(meta.output_bits)) return false;
		}

		if (filters.outputKind !== "all") {
			if (!meta || meta.output_kind !== filters.outputKind) return false;
		}

		if (filters.internallyParallel !== "all") {
			if (!meta) return false;
			const wantsParallel = filters.internallyParallel === "yes";
			if (meta.internally_parallel !== wantsParallel) return false;
		}

		if (filters.keyedOnly) {
			if (!meta?.keyed) return false;
		}

		if (filters.dosResistantOnly) {
			if (!meta?.dos_resistant) return false;
		}

		return true;
	});
}
