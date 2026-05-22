import { useCallback, useEffect, useMemo, useState } from "react";
import BenchmarkChart from "./components/BenchmarkChart";
import Controls from "./components/Controls";
import DataTable from "./components/DataTable";
import type { BenchmarkResult, FilterState, ReportData } from "./types";

export default function App() {
	const [data, setData] = useState<ReportData | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [filters, setFilters] = useState<FilterState | null>(null);

	useEffect(() => {
		fetch("/data/benchmarks.json")
			.then((r) => {
				if (!r.ok) throw new Error(`HTTP ${r.status}`);
				return r.json();
			})
			.then((d: ReportData) => {
				setData(d);
				const allThreads = [
					...new Set(d.benchmarks.map((b) => b.threads)),
				].sort((a, b) => a - b);
				const sizeOrder = new Map<string, number>();
				for (const b of d.benchmarks) sizeOrder.set(b.size, b.size_bytes);
				const allSizes = [...new Set(d.benchmarks.map((b) => b.size))].sort(
					(a, b) => (sizeOrder.get(a) ?? 0) - (sizeOrder.get(b) ?? 0),
				);

				setFilters({
					selectedPlatforms: new Set(d.platforms.map((p) => p.id)),
					threadCount: allThreads[0] ?? 1,
					size: allSizes[allSizes.length - 1] ?? "",
					category: "all",
					metric: "throughput",
					logScale: true,
				});
			})
			.catch((e) => setError(e.message));
	}, []);

	const filtered = useMemo<BenchmarkResult[]>(() => {
		if (!data || !filters) return [];
		return data.benchmarks.filter(
			(b) =>
				filters.selectedPlatforms.has(b.platform) &&
				b.threads === filters.threadCount &&
				b.size === filters.size &&
				(filters.category === "all" ||
					data.categories[b.algorithm] === filters.category),
		);
	}, [data, filters]);

	const platformMap = useMemo(() => {
		if (!data) return new Map<string, string>();
		return new Map(data.platforms.map((p) => [p.id, p.display_name]));
	}, [data]);

	const updateFilter = useCallback(
		<K extends keyof FilterState>(key: K, value: FilterState[K]) => {
			setFilters((prev) => (prev ? { ...prev, [key]: value } : prev));
		},
		[],
	);

	if (error) {
		return (
			<div className="flex min-h-screen items-center justify-center bg-gray-950 text-white">
				<div className="text-center">
					<h1 className="mb-2 text-2xl font-bold">hash-bench</h1>
					<p className="text-gray-400">
						No benchmark data available. Run benchmarks and redeploy.
					</p>
					<p className="mt-1 text-sm text-red-400">{error}</p>
				</div>
			</div>
		);
	}

	if (!data || !filters) {
		return (
			<div className="flex min-h-screen items-center justify-center bg-gray-950 text-white">
				<p className="text-gray-400">Loading benchmark data...</p>
			</div>
		);
	}

	const allThreads = [...new Set(data.benchmarks.map((b) => b.threads))].sort(
		(a, b) => a - b,
	);
	const sizeOrder = new Map<string, number>();
	for (const b of data.benchmarks) sizeOrder.set(b.size, b.size_bytes);
	const allSizes = [...new Set(data.benchmarks.map((b) => b.size))].sort(
		(a, b) => (sizeOrder.get(a) ?? 0) - (sizeOrder.get(b) ?? 0),
	);

	return (
		<div className="min-h-screen bg-white text-gray-900 dark:bg-gray-950 dark:text-gray-100">
			<div className="mx-auto max-w-[1400px] p-4">
				<header className="mb-4">
					<h1 className="text-2xl font-bold">hash-bench</h1>
					<p className="text-sm text-gray-500 dark:text-gray-400">
						Interactive benchmark results for hashing algorithms in Rust
					</p>
				</header>

				<Controls
					platforms={data.platforms}
					filters={filters}
					allThreadCounts={allThreads}
					allSizes={allSizes}
					onFilterChange={updateFilter}
					onPlatformToggle={(id) => {
						const next = new Set(filters.selectedPlatforms);
						if (next.has(id)) {
							next.delete(id);
							if (next.size === 0) next.add(id); // Keep at least one
						} else {
							next.add(id);
						}
						updateFilter("selectedPlatforms", next);
					}}
				/>

				<BenchmarkChart
					benchmarks={filtered}
					categories={data.categories}
					platformMap={platformMap}
					filters={filters}
				/>

				<DataTable
					benchmarks={filtered}
					categories={data.categories}
					platformMap={platformMap}
				/>

				<footer className="mt-8 border-t border-gray-200 pt-4 text-center text-xs text-gray-400 dark:border-gray-800 dark:text-gray-500">
					<p>
						Generated: {new Date(data.generated_at_unix_ms).toLocaleString()}
					</p>
					<a
						href="https://github.com/justin13888/hash-bench"
						target="_blank"
						rel="noopener noreferrer"
						className="text-blue-500 hover:underline"
					>
						github.com/justin13888/hash-bench
					</a>
				</footer>
			</div>
		</div>
	);
}
