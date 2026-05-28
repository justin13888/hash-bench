import { useMemo } from "react";
import { applyFilters } from "../lib/filter";
import { algoKey, displayName, formatBytes } from "../lib/format";
import { adjacencyTierRanks } from "../lib/stats";
import type { AlgorithmMeta, BenchmarkResult, FilterState } from "../types";

interface Props {
	benchmarks: BenchmarkResult[];
	algorithms: Record<string, AlgorithmMeta>;
	platformMap: Map<string, string>;
	filters: FilterState;
}

interface Cell {
	platform: string;
	platformLabel: string;
	threads: number;
	size: string;
	sizeBytes: number;
	winners: BenchmarkResult[];
}

/**
 * One row per (platform, size, threads) coordinate. Each row lists the
 * tier-1 algorithm(s) by throughput — rows whose 95% CIs overlap with the
 * top mean are considered tied and shown together. Below the table, a
 * "covering set" callout lists the minimum unique set of algorithms that
 * wins (or ties for first) in at least one coordinate.
 *
 * Coordinate filters (`threadCount`, `size`) are ignored — this view sweeps
 * them internally so the user can see the whole matrix.
 */
export default function WinnersSummary({
	benchmarks,
	algorithms,
	platformMap,
	filters,
}: Props) {
	const cells: Cell[] = useMemo(() => {
		// Sweep threads + size; keep platform + algorithm-slice filters.
		const rows = applyFilters(benchmarks, algorithms, filters, {
			threads: false,
			size: false,
		});

		const buckets = new Map<string, BenchmarkResult[]>();
		for (const b of rows) {
			const key = `${b.platform}|${b.threads}|${b.size_bytes}|${b.size}`;
			if (!buckets.has(key)) buckets.set(key, []);
			buckets.get(key)?.push(b);
		}

		const out: Cell[] = [];
		for (const [key, rows] of buckets) {
			const [platform, threadsStr, sizeBytesStr, size] = key.split("|");
			if (rows.length === 0) continue;
			const sorted = [...rows].sort(
				(a, b) => b.throughput_bps - a.throughput_bps,
			);
			const tiers = adjacencyTierRanks(sorted, "throughput");
			const winners = sorted.filter((_, i) => tiers[i] === 1);
			out.push({
				platform,
				platformLabel: platformMap.get(platform) ?? platform,
				threads: Number(threadsStr),
				size,
				sizeBytes: Number(sizeBytesStr),
				winners,
			});
		}

		out.sort(
			(a, b) =>
				a.platformLabel.localeCompare(b.platformLabel) ||
				a.threads - b.threads ||
				a.sizeBytes - b.sizeBytes,
		);
		return out;
	}, [benchmarks, algorithms, filters, platformMap]);

	const coveringSet = useMemo(() => {
		const set = new Set<string>();
		for (const c of cells) {
			for (const w of c.winners) {
				set.add(algoKey(w.algorithm, w.variant));
			}
		}
		return [...set].sort();
	}, [cells]);

	if (cells.length === 0) {
		return (
			<div className="mb-4 flex h-48 items-center justify-center rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
				<p className="text-gray-400">
					No coordinates match the current filters.
				</p>
			</div>
		);
	}

	return (
		<div className="mb-4 rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
			<div className="overflow-x-auto">
				<table className="w-full text-sm">
					<thead>
						<tr>
							<Th>Platform</Th>
							<Th numeric>Threads</Th>
							<Th>Size</Th>
							<Th>Tier-1 algorithm(s)</Th>
							<Th numeric>Throughput</Th>
						</tr>
					</thead>
					<tbody>
						{cells.map((c) => {
							const top = c.winners[0];
							return (
								<tr
									key={`${c.platform}|${c.threads}|${c.size}`}
									className="border-t border-gray-200 dark:border-gray-800"
								>
									<td className="px-3 py-2">{c.platformLabel}</td>
									<td className="px-3 py-2 text-right tabular-nums">
										{c.threads}
									</td>
									<td className="px-3 py-2">{c.size}</td>
									<td className="px-3 py-2">
										<div className="flex flex-wrap gap-1">
											{c.winners.map((w) => {
												const meta =
													algorithms[algoKey(w.algorithm, w.variant)];
												const cat = meta?.category ?? "unknown";
												return (
													<span
														key={`${w.algorithm}|${w.variant}`}
														className={`rounded px-1.5 py-0.5 text-xs font-medium ${
															cat === "cryptographic"
																? "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400"
																: "bg-teal-100 text-teal-700 dark:bg-teal-900/30 dark:text-teal-400"
														}`}
													>
														{displayName(w.algorithm, w.variant)}
													</span>
												);
											})}
										</div>
									</td>
									<td className="px-3 py-2 text-right tabular-nums">
										{top ? formatBytes(top.throughput_bps) : "—"}
									</td>
								</tr>
							);
						})}
					</tbody>
				</table>
			</div>

			<div className="border-t border-gray-200 p-4 text-sm dark:border-gray-800">
				<p className="mb-1 font-semibold">
					Covering set ({coveringSet.length} algorithm
					{coveringSet.length === 1 ? "" : "s"}):
				</p>
				<div className="flex flex-wrap gap-1">
					{coveringSet.map((k) => {
						const [algo, variant] = k.split("|");
						return (
							<code
								key={k}
								className="rounded bg-gray-200 px-1.5 py-0.5 text-xs dark:bg-gray-800"
							>
								{displayName(algo, variant)}
							</code>
						);
					})}
				</div>
				<p className="mt-2 text-xs text-gray-500 dark:text-gray-400">
					Minimum unique set of algorithms that tier-1 (statistical ties
					included) in at least one (platform, threads, size) coordinate above.
				</p>
			</div>
		</div>
	);
}

function Th({
	children,
	numeric,
}: {
	children: React.ReactNode;
	numeric?: boolean;
}) {
	return (
		<th
			className={`sticky top-0 bg-gray-50 px-3 py-2 font-semibold whitespace-nowrap dark:bg-gray-900 ${
				numeric ? "text-right" : "text-left"
			}`}
		>
			{children}
		</th>
	);
}
