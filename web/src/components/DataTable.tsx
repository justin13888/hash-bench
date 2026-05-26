import { useCallback, useMemo, useState } from "react";
import { algoKey, displayName, formatBytes, formatNs } from "../lib/format";
import type { BenchmarkResult } from "../types";

interface Props {
	benchmarks: BenchmarkResult[];
	categories: Record<string, string>;
	platformMap: Map<string, string>;
}

type SortKey =
	| "algorithm"
	| "category"
	| "platform"
	| "throughput_bps"
	| "mean_ns"
	| "median_ns";

export default function DataTable({
	benchmarks,
	categories,
	platformMap,
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

	const sorted = useMemo(() => {
		const rows = benchmarks.map((b) => ({
			...b,
			category: categories[algoKey(b.algorithm, b.variant)] ?? "unknown",
			display: displayName(b.algorithm, b.variant),
			platform_name: platformMap.get(b.platform) ?? b.platform,
		}));

		rows.sort((a, b) => {
			let va: string | number;
			let vb: string | number;

			switch (sortKey) {
				case "algorithm":
					// Sort by algorithm name first, then variant — keeps SHA-256 [sw]
					// and SHA-256 [sha-ext] adjacent.
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
	}, [benchmarks, categories, platformMap, sortKey, sortAsc]);

	const arrow = (key: SortKey) =>
		sortKey === key ? (sortAsc ? " \u25B2" : " \u25BC") : "";

	if (benchmarks.length === 0) return null;

	return (
		<div className="overflow-x-auto rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
			<table className="w-full text-sm">
				<thead>
					<tr>
						<Th onClick={() => handleSort("algorithm")}>
							Algorithm{arrow("algorithm")}
						</Th>
						<Th onClick={() => handleSort("category")}>
							Category{arrow("category")}
						</Th>
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
					{sorted.map((b) => (
						<tr
							key={`${b.platform}-${b.algorithm}-${b.variant}`}
							className="border-t border-gray-200 dark:border-gray-800"
						>
							<td className="px-3 py-2">{b.display}</td>
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
								{formatNs(b.mean_lower_ns)} &ndash; {formatNs(b.mean_upper_ns)}
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
