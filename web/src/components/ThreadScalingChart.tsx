import { LineChart } from "echarts/charts";
import {
	GridComponent,
	LegendComponent,
	TooltipComponent,
} from "echarts/components";
import * as echarts from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import ReactEChartsCore from "echarts-for-react/lib/core";
import { useMemo } from "react";
import { PLATFORM_COLORS } from "../lib/categories";
import { applyFilters } from "../lib/filter";
import { algoKey, displayName, formatBytes } from "../lib/format";
import type { AlgorithmMeta, BenchmarkResult, FilterState } from "../types";

echarts.use([
	LineChart,
	TooltipComponent,
	GridComponent,
	LegendComponent,
	CanvasRenderer,
]);

interface Props {
	benchmarks: BenchmarkResult[];
	algorithms: Record<string, AlgorithmMeta>;
	filters: FilterState;
}

/**
 * Throughput-vs-threads at the selected (platform, size). The `threads`
 * filter is ignored — we sweep threads internally. Each line is one
 * (algorithm, variant). Lines for internally-parallel implementations are
 * drawn solid; single-stream lines are dashed so the user can see how the
 * parallel impl scales relative to the aggregate independent-stream baseline.
 */
export default function ThreadScalingChart({
	benchmarks,
	algorithms,
	filters,
}: Props) {
	const option = useMemo(() => {
		const platform = [...filters.selectedPlatforms][0];
		if (!platform) return null;

		// Sweep threads; keep platform + size + algorithm-slice filters.
		const rows = applyFilters(benchmarks, algorithms, filters, {
			threads: false,
		}).filter((b) => b.platform === platform);
		if (rows.length === 0) return null;

		const threadsSet = new Set<number>();
		const byAlgo = new Map<string, Map<number, BenchmarkResult>>();
		for (const r of rows) {
			threadsSet.add(r.threads);
			const k = algoKey(r.algorithm, r.variant);
			if (!byAlgo.has(k)) byAlgo.set(k, new Map());
			byAlgo.get(k)?.set(r.threads, r);
		}
		const xs = [...threadsSet].sort((a, b) => a - b);

		const keys = [...byAlgo.keys()].sort((a, b) => {
			const aMax = Math.max(
				...[...(byAlgo.get(a)?.values() ?? [])].map((r) => r.throughput_bps),
			);
			const bMax = Math.max(
				...[...(byAlgo.get(b)?.values() ?? [])].map((r) => r.throughput_bps),
			);
			return bMax - aMax;
		});

		const series = keys.map((k, i) => {
			const [algo, variant] = k.split("|");
			const meta = algorithms[k];
			const parallel = meta?.internally_parallel ?? false;
			return {
				name: displayName(algo, variant),
				type: "line" as const,
				symbol: "circle" as const,
				symbolSize: 6,
				lineStyle: {
					width: parallel ? 2.5 : 1.5,
					type: parallel ? ("solid" as const) : ("dashed" as const),
					color: PLATFORM_COLORS[i % PLATFORM_COLORS.length],
				},
				itemStyle: {
					color: PLATFORM_COLORS[i % PLATFORM_COLORS.length],
				},
				data: xs.map((t) => byAlgo.get(k)?.get(t)?.throughput_bps ?? null),
			};
		});

		return {
			tooltip: {
				trigger: "axis" as const,
				formatter: (
					params: Array<{
						seriesName: string;
						value: number;
						axisValue: string;
					}>,
				) => {
					const t = params[0]?.axisValue;
					const lines = params
						.filter((p) => p.value != null)
						.sort((a, b) => b.value - a.value)
						.map((p) => `${p.seriesName}: ${formatBytes(p.value)}`)
						.join("<br/>");
					return `<b>${t} threads</b><br/>${lines}`;
				},
			},
			legend: {
				top: 0,
				type: "scroll" as const,
				textStyle: { fontSize: 10 },
			},
			grid: { left: 80, right: 40, top: 80, bottom: 50 },
			xAxis: {
				type: "category" as const,
				data: xs.map(String),
				name: "Threads",
				nameLocation: "middle" as const,
				nameGap: 30,
			},
			yAxis: {
				type: filters.logScale ? ("log" as const) : ("value" as const),
				name: "Throughput",
				nameLocation: "middle" as const,
				nameGap: 60,
				axisLabel: { formatter: (v: number) => formatBytes(v) },
			},
			series,
			animation: false,
		};
	}, [benchmarks, algorithms, filters]);

	if (!option) {
		return (
			<div className="mb-4 flex h-48 items-center justify-center rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
				<p className="text-gray-400">
					Thread scaling view needs a single platform and a size.
				</p>
			</div>
		);
	}

	return (
		<div className="mb-4 overflow-hidden rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
			<ReactEChartsCore
				echarts={echarts}
				option={option}
				style={{ height: 500, width: "100%" }}
				notMerge
			/>
		</div>
	);
}
