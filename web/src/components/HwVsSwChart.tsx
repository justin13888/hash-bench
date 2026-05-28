import { BarChart } from "echarts/charts";
import {
	GridComponent,
	LegendComponent,
	TooltipComponent,
} from "echarts/components";
import * as echarts from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import ReactEChartsCore from "echarts-for-react/lib/core";
import { useMemo } from "react";
import { applyFilters } from "../lib/filter";
import { formatBytes } from "../lib/format";
import type { AlgorithmMeta, BenchmarkResult, FilterState } from "../types";

echarts.use([
	BarChart,
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

const SW_COLOR = "#9aa0a6";
const HW_COLOR = "#f4a261";

/**
 * Paired bars per algorithm name where both a `sw` variant and a HW-required
 * variant exist on the selected platform at the chosen (size, threads) point.
 * A label shows the speedup ratio (HW / SW).
 */
export default function HwVsSwChart({
	benchmarks,
	algorithms,
	filters,
}: Props) {
	const option = useMemo(() => {
		const platform = [...filters.selectedPlatforms][0];
		if (!platform) return null;

		// All coordinate filters apply (single point); algorithm-slice filters
		// still apply (e.g. user restricts to crypto). Variant filter must NOT
		// be respected here — this view needs both sw and HW rows.
		const rows = applyFilters(benchmarks, algorithms, {
			...filters,
			variants: new Set<string>(),
			hwAcceleration: "all",
		});
		const byAlgo = new Map<string, Map<string, BenchmarkResult>>();
		for (const b of rows) {
			if (b.platform !== platform) continue;
			if (!byAlgo.has(b.algorithm)) byAlgo.set(b.algorithm, new Map());
			byAlgo.get(b.algorithm)?.set(b.variant, b);
		}

		// Keep only algorithms that have both a sw row and a HW-required row.
		const pairs: Array<{
			algorithm: string;
			sw: BenchmarkResult;
			hw: BenchmarkResult;
			hwVariant: string;
		}> = [];
		for (const [name, variants] of byAlgo) {
			const sw = variants.get("sw");
			if (!sw) continue;
			for (const [vName, vRow] of variants) {
				if (vName === "sw") continue;
				const meta = algorithms[`${name}|${vName}`];
				if (!meta?.hardware_required) continue;
				pairs.push({ algorithm: name, sw, hw: vRow, hwVariant: vName });
			}
		}
		if (pairs.length === 0) return null;

		pairs.sort((a, b) => b.hw.throughput_bps - a.hw.throughput_bps);

		const labels = pairs.map((p) => p.algorithm);
		const swValues = pairs.map((p) => p.sw.throughput_bps);
		const hwValues = pairs.map((p) => p.hw.throughput_bps);
		const ratios = pairs.map((p) => p.hw.throughput_bps / p.sw.throughput_bps);

		return {
			tooltip: {
				trigger: "axis" as const,
				axisPointer: { type: "shadow" as const },
				formatter: (
					params: Array<{ name: string; seriesName: string; value: number }>,
				) => {
					const name = params[0]?.name;
					const p = pairs.find((x) => x.algorithm === name);
					if (!p) return "";
					const ratio = p.hw.throughput_bps / p.sw.throughput_bps;
					return (
						`<b>${p.algorithm}</b><br/>` +
						`SW: ${formatBytes(p.sw.throughput_bps)}<br/>` +
						`HW [${p.hwVariant}]: ${formatBytes(p.hw.throughput_bps)}<br/>` +
						`Speedup: ${ratio.toFixed(2)}×`
					);
				},
			},
			legend: { top: 0 },
			grid: { left: 100, right: 80, top: 40, bottom: 40 },
			yAxis: {
				type: "category" as const,
				data: labels,
				axisLabel: { fontSize: 11 },
				axisTick: { show: false },
			},
			xAxis: {
				type: filters.logScale ? ("log" as const) : ("value" as const),
				name: "Throughput",
				nameLocation: "middle" as const,
				nameGap: 25,
				axisLabel: { formatter: (v: number) => formatBytes(v) },
			},
			series: [
				{
					name: "SW",
					type: "bar" as const,
					data: swValues,
					itemStyle: { color: SW_COLOR },
					barMaxWidth: 14,
				},
				{
					name: "HW",
					type: "bar" as const,
					data: hwValues,
					itemStyle: { color: HW_COLOR },
					barMaxWidth: 14,
					label: {
						show: true,
						position: "right" as const,
						fontSize: 10,
						formatter: (p: { dataIndex: number }) =>
							`${ratios[p.dataIndex].toFixed(2)}×`,
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
					No algorithms with both SW and HW-accelerated variants at this
					coordinate.
				</p>
			</div>
		);
	}

	const labelCount = (option.yAxis.data as string[]).length;
	const height = Math.max(300, labelCount * 36 + 100);

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
