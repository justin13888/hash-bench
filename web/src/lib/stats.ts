import type { BenchmarkResult, Metric } from "../types";

/**
 * Throughput- or latency-axis CI bounds for a single row.
 *
 * For throughput, the CI on latency is inverted: a lower latency bound
 * yields a higher throughput bound and vice versa.
 */
export function ciBounds(
	row: BenchmarkResult,
	metric: Metric,
): [number, number] {
	if (metric === "throughput") {
		const lo = row.size_bytes / (row.ci_upper_ns * 1e-9);
		const hi = row.size_bytes / (row.ci_lower_ns * 1e-9);
		return [lo, hi];
	}
	return [row.ci_lower_ns, row.ci_upper_ns];
}

function overlaps(a: [number, number], b: [number, number]): boolean {
	return Math.max(a[0], b[0]) <= Math.min(a[1], b[1]);
}

/**
 * Group rows whose 95% CIs overlap on the chosen metric. Inputs are assumed
 * to be sorted in the user's display order (best first); tiers are formed by
 * walking adjacent rows and breaking when the next CI no longer overlaps the
 * previous one. Returns the rank (1-based) for each input row in order.
 *
 * Adjacency-based, not transitive-closure: avoids chaining wildly different
 * rows through a long overlap chain.
 */
export function adjacencyTierRanks(
	sortedRows: BenchmarkResult[],
	metric: Metric,
): number[] {
	if (sortedRows.length === 0) return [];

	const ranks: number[] = new Array(sortedRows.length);
	let tier = 1;
	ranks[0] = tier;
	let prev = ciBounds(sortedRows[0], metric);

	for (let i = 1; i < sortedRows.length; i++) {
		const cur = ciBounds(sortedRows[i], metric);
		if (!overlaps(prev, cur)) {
			tier += 1;
		}
		ranks[i] = tier;
		prev = cur;
	}

	return ranks;
}
