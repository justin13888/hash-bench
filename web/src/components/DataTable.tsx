import { useCallback, useMemo, useState } from "react";
import { algoKey, displayName, formatBytes, formatNs } from "../lib/format";
import { adjacencyTierRanks } from "../lib/stats";
import type {
	AlgorithmMeta,
	BenchmarkResult,
	FilterState,
	Metric,
} from "../types";

interface Props {
	benchmarks: BenchmarkResult[];
	algorithms: Record<string, AlgorithmMeta>;
	platformMap: Map<string, string>;
	filters: FilterState;
}

type SortKey =
	| "algorithm"
	| "category"
	| "platform"
	| "throughput_bps"
	| "mean_ns"
	| "median_ns";

interface Row extends BenchmarkResult {
	category: string;
	display: string;
	platform_name: string;
	output_bits: number | null;
	hardware_features: string[];
	keyed: boolean;
	dos_resistant: boolean;
}

export default function DataTable({
	benchmarks,
	algorithms,
	platformMap,
	filters,
}: Props) {
	const [sortKey, setSortKey] = useState<SortKey>("throughput_bps");
	const [sortAsc, setSortAsc] = useState(false);

	const handleSort = useCallback(
		(key: SortKey) => {
			if (sortKey === key) {
				setSortAsc((prev) => !prev);
			} else {
				setSortKey(key);
				setSortAsc(key === "algorithm");
			}
		},
		[sortKey],
	);

	const sorted = useMemo<Row[]>(() => {
		const rows: Row[] = benchmarks.map((b) => {
			const meta = algorithms[algoKey(b.algorithm, b.variant)];
			return {
				...b,
				category: meta?.category ?? "unknown",
				display: displayName(b.algorithm, b.variant),
				platform_name: platformMap.get(b.platform) ?? b.platform,
				output_bits: meta?.output_bits ?? null,
				hardware_features: meta?.hardware_features ?? [],
				keyed: meta?.keyed ?? false,
				dos_resistant: meta?.dos_resistant ?? false,
			};
		});

		rows.sort((a, b) => {
			let va: string | number;
			let vb: string | number;

			switch (sortKey) {
				case "algorithm":
					va = `${a.algorithm.toLowerCase()}|${a.variant}`;
					vb = `${b.algorithm.toLowerCase()}|${b.variant}`;
					break;
				case "category":
					va = a.category;
					vb = b.category;
					break;
				case "platform":
					va = a.platform_name;
					vb = b.platform_name;
					break;
				case "throughput_bps":
					va = a.throughput_bps;
					vb = b.throughput_bps;
					break;
				case "mean_ns":
					va = a.mean_ns;
					vb = b.mean_ns;
					break;
				case "median_ns":
					va = a.median_ns;
					vb = b.median_ns;
					break;
			}

			if (va < vb) return sortAsc ? -1 : 1;
			if (va > vb) return sortAsc ? 1 : -1;
			return 0;
		});

		return rows;
	}, [benchmarks, algorithms, platformMap, sortKey, sortAsc]);

	const tierRanks = useMemo(() => {
		if (!filters.ciTieGrouping) return null;
		// Tier ranks only make sense when sorting by a performance metric in the
		// "best first" direction. Throughput is best when largest; latency when
		// smallest. Other sort keys (algorithm/category/platform) don't carry a
		// statistical ordering, so skip tier rendering for them.
		const metric: Metric | null =
			sortKey === "throughput_bps" && !sortAsc
				? "throughput"
				: (sortKey === "mean_ns" || sortKey === "median_ns") && sortAsc
					? "latency"
					: null;
		if (metric === null) return null;
		return adjacencyTierRanks(sorted, metric);
	}, [sorted, filters.ciTieGrouping, sortKey, sortAsc]);

	const arrow = (key: SortKey) =>
		sortKey === key ? (sortAsc ? " ▲" : " ▼") : "";

	if (benchmarks.length === 0) return null;

	return (
		<div className="overflow-x-auto rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
			<table className="w-full text-sm">
				<thead>
					<tr>
						{tierRanks && (
							<Th numeric>
								<abbr title="Tier rank — rows whose 95% CIs overlap share a tier.">
									Tier
								</abbr>
							</Th>
						)}
						<Th onClick={() => handleSort("algorithm")}>
							Algorithm{arrow("algorithm")}
						</Th>
						<Th>Variant</Th>
						<Th onClick={() => handleSort("category")}>
							Category{arrow("category")}
						</Th>
						<Th numeric>Bits</Th>
						<Th>HW features</Th>
						<Th>Keyed</Th>
						<Th>DoS-res.</Th>
						<Th onClick={() => handleSort("platform")}>
							Platform{arrow("platform")}
						</Th>
						<Th onClick={() => handleSort("throughput_bps")} numeric>
							Throughput{arrow("throughput_bps")}
						</Th>
						<Th onClick={() => handleSort("mean_ns")} numeric>
							Mean Latency{arrow("mean_ns")}
						</Th>
						<Th onClick={() => handleSort("median_ns")} numeric>
							Median Latency{arrow("median_ns")}
						</Th>
						<Th numeric>95% CI</Th>
					</tr>
				</thead>
				<tbody>
					{sorted.map((b, i) => (
						<tr
							key={`${b.platform}-${b.algorithm}-${b.variant}`}
							className="border-t border-gray-200 dark:border-gray-800"
						>
							{tierRanks && (
								<td className="px-3 py-2 text-right tabular-nums">
									#{tierRanks[i]}
									{i > 0 && tierRanks[i] === tierRanks[i - 1] ? "=" : ""}
								</td>
							)}
							<td className="px-3 py-2">{b.algorithm}</td>
							<td className="px-3 py-2">
								<code className="rounded bg-gray-200 px-1.5 py-0.5 text-xs dark:bg-gray-800">
									{b.variant}
								</code>
							</td>
							<td className="px-3 py-2">
								<span
									className={`inline-block rounded px-1.5 py-0.5 text-xs font-semibold ${
										b.category === "cryptographic"
											? "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400"
											: "bg-teal-100 text-teal-700 dark:bg-teal-900/30 dark:text-teal-400"
									}`}
								>
									{b.category}
								</span>
							</td>
							<td className="px-3 py-2 text-right tabular-nums">
								{b.output_bits ?? "—"}
							</td>
							<td className="px-3 py-2">
								{b.hardware_features.length === 0 ? (
									<span className="text-gray-400">—</span>
								) : (
									<div className="flex flex-wrap gap-1">
										{b.hardware_features.map((f) => (
											<code
												key={f}
												className="rounded bg-amber-100 px-1.5 py-0.5 text-xs text-amber-800 dark:bg-amber-900/30 dark:text-amber-400"
											>
												{f}
											</code>
										))}
									</div>
								)}
							</td>
							<td className="px-3 py-2">{b.keyed ? "✓" : ""}</td>
							<td className="px-3 py-2">{b.dos_resistant ? "✓" : ""}</td>
							<td className="px-3 py-2">{b.platform_name}</td>
							<td className="px-3 py-2 text-right tabular-nums">
								{formatBytes(b.throughput_bps)}
							</td>
							<td className="px-3 py-2 text-right tabular-nums">
								{formatNs(b.mean_ns)}
							</td>
							<td className="px-3 py-2 text-right tabular-nums">
								{formatNs(b.median_ns)}
							</td>
							<td className="px-3 py-2 text-right tabular-nums">
								{formatNs(b.ci_lower_ns)} – {formatNs(b.ci_upper_ns)}
							</td>
						</tr>
					))}
				</tbody>
			</table>
		</div>
	);
}

function Th({
	children,
	onClick,
	numeric,
}: {
	children: React.ReactNode;
	onClick?: () => void;
	numeric?: boolean;
}) {
	return (
		<th
			onClick={onClick}
			className={`sticky top-0 bg-gray-50 px-3 py-2 font-semibold whitespace-nowrap dark:bg-gray-900 ${
				onClick ? "cursor-pointer select-none hover:text-blue-500" : ""
			} ${numeric ? "text-right" : "text-left"}`}
		>
			{children}
		</th>
	);
}
