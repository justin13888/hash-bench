import { HeatmapChart } from "echarts/charts";
import {
	GridComponent,
	TooltipComponent,
	VisualMapComponent,
} from "echarts/components";
import * as echarts from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import ReactEChartsCore from "echarts-for-react/lib/core";
import { useMemo } from "react";
import { applyFilters } from "../lib/filter";
import { algoKey, displayName, formatBytes, formatNs } from "../lib/format";
import type { AlgorithmMeta, BenchmarkResult, FilterState } from "../types";

echarts.use([
	HeatmapChart,
	TooltipComponent,
	GridComponent,
	VisualMapComponent,
	CanvasRenderer,
]);

interface Props {
	benchmarks: BenchmarkResult[];
	algorithms: Record<string, AlgorithmMeta>;
	filters: FilterState;
}

/**
 * Heatmap of algorithm × input-size for one platform at the selected thread
 * count. The `size` filter is intentionally ignored: this view sweeps size
 * internally so a user can see whether the best algorithm at 64 B remains
 * best at 100 MiB.
 */
export default function WinnersHeatmap({
	benchmarks,
	algorithms,
	filters,
}: Props) {
	const option = useMemo(() => {
		// Single platform only — heatmap conflates platforms otherwise.
		const platform = [...filters.selectedPlatforms][0];
		if (!platform) return null;

		// Sweep size; keep platform + threads + algorithm-slice filters.
		const rows = applyFilters(benchmarks, algorithms, filters, {
			size: false,
		}).filter((b) => b.platform === platform);
		if (rows.length === 0) return null;

		// Algorithm keys and sizes encountered.
		const algoKeysSet = new Set<string>();
		const sizesMap = new Map<string, number>();
		for (const r of rows) {
			algoKeysSet.add(algoKey(r.algorithm, r.variant));
			sizesMap.set(r.size, r.size_bytes);
		}
		const sizes = [...sizesMap.entries()]
			.sort(([, a], [, b]) => a - b)
			.map(([s]) => s);
		const algoKeys = [...algoKeysSet];

		// Average throughput per algorithm to choose row order (best at the bottom
		// of the heatmap — ECharts y-axis renders bottom-up by default).
		const avgByAlgo = new Map<string, number>();
		for (const k of algoKeys) {
			const vals = rows
				.filter((r) => algoKey(r.algorithm, r.variant) === k)
				.map((r) => r.throughput_bps);
			avgByAlgo.set(
				k,
				vals.reduce((a, b) => a + b, 0) / Math.max(vals.length, 1),
			);
		}
		algoKeys.sort((a, b) => (avgByAlgo.get(a) ?? 0) - (avgByAlgo.get(b) ?? 0));

		// Build (xIdx, yIdx, throughput_bps) cells.
		const data: Array<[number, number, number]> = [];
		const byCoord = new Map<string, BenchmarkResult>();
		for (const r of rows) {
			byCoord.set(`${algoKey(r.algorithm, r.variant)}|${r.size}`, r);
		}
		for (let y = 0; y < algoKeys.length; y++) {
			for (let x = 0; x < sizes.length; x++) {
				const r = byCoord.get(`${algoKeys[y]}|${sizes[x]}`);
				if (r) data.push([x, y, r.throughput_bps]);
			}
		}

		const max = data.reduce((m, [, , v]) => Math.max(m, v), 0);

		return {
			tooltip: {
				formatter: (p: { value: [number, number, number] }) => {
					const [xIdx, yIdx, val] = p.value;
					const key = algoKeys[yIdx];
					const [algo, variant] = key.split("|");
					const size = sizes[xIdx];
					const row = byCoord.get(`${key}|${size}`);
					if (!row) return "";
					return (
						`<b>${displayName(algo, variant)}</b><br/>` +
						`Size: ${size}<br/>` +
						`Throughput: ${formatBytes(val)}<br/>` +
						`Mean latency: ${formatNs(row.mean_ns)}`
					);
				},
			},
			grid: { left: 180, right: 100, top: 30, bottom: 50 },
			xAxis: {
				type: "category" as const,
				data: sizes,
				name: "Input size",
				nameLocation: "middle" as const,
				nameGap: 30,
				axisLabel: { fontSize: 11 },
			},
			yAxis: {
				type: "category" as const,
				data: algoKeys.map((k) => {
					const [algo, variant] = k.split("|");
					return displayName(algo, variant);
				}),
				axisLabel: { fontSize: 11 },
			},
			visualMap: {
				min: 0,
				max,
				calculable: true,
				orient: "vertical" as const,
				right: 10,
				top: "middle" as const,
				inRange: { color: ["#0e1a30", "#2ec4b6", "#fff3b0"] },
				formatter: (v: number) => formatBytes(v),
			},
			series: [
				{
					type: "heatmap" as const,
					data,
					label: {
						show: false,
					},
					emphasis: {
						itemStyle: {
							shadowBlur: 8,
							shadowColor: "rgba(255,255,255,0.5)",
						},
					},
				},
			],
			animation: false,
		};
	}, [benchmarks, algorithms, filters]);

	if (!option) {
		return (
			<div className="mb-4 flex h-48 items-center justify-center rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
				<p className="text-gray-400">
					Heatmap is for a single platform — select one.
				</p>
			</div>
		);
	}

	const algoCount = (option.yAxis.data as string[]).length;
	const height = Math.max(400, algoCount * 22 + 80);

	return (
		<div className="mb-4 overflow-hidden rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
			<ReactEChartsCore
				echarts={echarts}
				option={option}
				style={{ height, width: "100%" }}
				notMerge
			/>
		</div>
	);
}
