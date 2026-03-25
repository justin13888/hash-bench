import { BarChart, CustomChart } from "echarts/charts";
import {
	GridComponent,
	LegendComponent,
	TooltipComponent,
} from "echarts/components";
import * as echarts from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import ReactEChartsCore from "echarts-for-react/lib/core";
import { useMemo } from "react";
import {
	CRYPTO_COLOR,
	NONCRYPTO_COLOR,
	PLATFORM_COLORS,
} from "../lib/categories";
import { formatBytes, formatNs, formatValue } from "../lib/format";
import type { BenchmarkResult, FilterState } from "../types";

echarts.use([
	BarChart,
	CustomChart,
	TooltipComponent,
	GridComponent,
	LegendComponent,
	CanvasRenderer,
]);

interface Props {
	benchmarks: BenchmarkResult[];
	categories: Record<string, string>;
	platformMap: Map<string, string>;
	filters: FilterState;
}

function getValue(b: BenchmarkResult, metric: "throughput" | "latency") {
	return metric === "throughput" ? b.throughput_bps : b.mean_ns;
}

export default function BenchmarkChart({
	benchmarks,
	categories,
	platformMap,
	filters,
}: Props) {
	const multiPlatform = filters.selectedPlatforms.size > 1;

	const option = useMemo(() => {
		if (benchmarks.length === 0) return null;

		if (multiPlatform) {
			return buildComparisonOption(
				benchmarks,
				categories,
				platformMap,
				filters,
			);
		}
		return buildSingleOption(benchmarks, categories, filters);
	}, [benchmarks, categories, platformMap, filters, multiPlatform]);

	const chartHeight = useMemo(() => {
		const uniqueAlgos = new Set(benchmarks.map((b) => b.algorithm)).size;
		const barH = Math.max(24, Math.min(36, 600 / uniqueAlgos));
		const multiplier = multiPlatform ? filters.selectedPlatforms.size : 1;
		return Math.max(400, uniqueAlgos * barH * multiplier + 100);
	}, [benchmarks, multiPlatform, filters.selectedPlatforms.size]);

	if (!option) {
		return (
			<div className="mb-4 flex h-48 items-center justify-center rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
				<p className="text-gray-400">No data for current filter combination.</p>
			</div>
		);
	}

	return (
		<div className="mb-4 overflow-hidden rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-800 dark:bg-gray-900">
			<ReactEChartsCore
				echarts={echarts}
				option={option}
				style={{ height: chartHeight, width: "100%" }}
				notMerge
			/>
		</div>
	);
}

function buildSingleOption(
	benchmarks: BenchmarkResult[],
	categories: Record<string, string>,
	filters: FilterState,
) {
	// Deduplicate by algorithm (one platform)
	const byAlgo = new Map<string, BenchmarkResult>();
	for (const b of benchmarks) {
		byAlgo.set(b.algorithm, b);
	}

	const entries = [...byAlgo.values()];
	if (filters.metric === "throughput") {
		entries.sort((a, b) => a.throughput_bps - b.throughput_bps);
	} else {
		entries.sort((a, b) => b.mean_ns - a.mean_ns);
	}

	const algorithms = entries.map((e) => e.algorithm);
	const values = entries.map((e) => getValue(e, filters.metric));
	const colors = entries.map((e) =>
		categories[e.algorithm] === "cryptographic"
			? CRYPTO_COLOR
			: NONCRYPTO_COLOR,
	);

	const errorData = entries.map((e) => {
		if (filters.metric === "throughput") {
			return [
				e.size_bytes / (e.mean_upper_ns * 1e-9),
				e.size_bytes / (e.mean_lower_ns * 1e-9),
			];
		}
		return [e.mean_lower_ns, e.mean_upper_ns];
	});

	return {
		tooltip: {
			trigger: "axis" as const,
			axisPointer: { type: "shadow" as const },
			formatter(params: Array<{ dataIndex: number }>) {
				const idx = params[0].dataIndex;
				const b = entries[idx];
				const cat = categories[b.algorithm] ?? "unknown";
				return (
					`<b>${b.algorithm}</b><br/>` +
					`Category: ${cat}<br/>` +
					`Throughput: ${formatBytes(b.throughput_bps)}<br/>` +
					`Mean latency: ${formatNs(b.mean_ns)}<br/>` +
					`Median latency: ${formatNs(b.median_ns)}<br/>` +
					`95% CI: ${formatNs(b.mean_lower_ns)} \u2013 ${formatNs(b.mean_upper_ns)}`
				);
			},
		},
		grid: { left: 160, right: 80, top: 20, bottom: 40 },
		xAxis: {
			type: filters.logScale ? ("log" as const) : ("value" as const),
			name: filters.metric === "throughput" ? "Throughput" : "Latency",
			nameLocation: "middle" as const,
			nameGap: 25,
			axisLabel: {
				formatter: (val: number) => formatValue(val, filters.metric),
			},
		},
		yAxis: {
			type: "category" as const,
			data: algorithms,
			axisLabel: { fontSize: 11 },
			axisTick: { show: false },
		},
		series: [
			{
				type: "bar" as const,
				data: values.map((v, i) => ({
					value: v,
					itemStyle: { color: colors[i] },
				})),
				barMaxWidth: 30,
				label: {
					show: true,
					position: "right" as const,
					fontSize: 10,
					formatter: (p: { value: number }) =>
						formatValue(p.value, filters.metric),
				},
			},
			{
				type: "custom" as const,
				renderItem(
					params: { dataIndex: number },
					api: {
						value: (dim: number) => number;
						coord: (val: [number, number]) => [number, number];
					},
				) {
					const catIdx = api.value(1);
					const [lo, hi] = errorData[params.dataIndex];
					const loPoint = api.coord([lo, catIdx]);
					const hiPoint = api.coord([hi, catIdx]);
					const halfH = 4;
					return {
						type: "group",
						children: [
							{
								type: "line",
								shape: {
									x1: loPoint[0],
									y1: loPoint[1],
									x2: hiPoint[0],
									y2: hiPoint[1],
								},
								style: { stroke: "#888", lineWidth: 1 },
							},
							{
								type: "line",
								shape: {
									x1: loPoint[0],
									y1: loPoint[1] - halfH,
									x2: loPoint[0],
									y2: loPoint[1] + halfH,
								},
								style: { stroke: "#888", lineWidth: 1 },
							},
							{
								type: "line",
								shape: {
									x1: hiPoint[0],
									y1: hiPoint[1] - halfH,
									x2: hiPoint[0],
									y2: hiPoint[1] + halfH,
								},
								style: { stroke: "#888", lineWidth: 1 },
							},
						],
					};
				},
				data: values.map((v, i) => [v, i]),
				z: 10,
			},
		],
		animation: true,
		animationDuration: 300,
	};
}

function buildComparisonOption(
	benchmarks: BenchmarkResult[],
	categories: Record<string, string>,
	platformMap: Map<string, string>,
	filters: FilterState,
) {
	const algoMap = new Map<string, Map<string, BenchmarkResult>>();
	for (const b of benchmarks) {
		if (!algoMap.has(b.algorithm)) algoMap.set(b.algorithm, new Map());
		algoMap.get(b.algorithm)?.set(b.platform, b);
	}

	const platformIds = [...filters.selectedPlatforms];
	const algorithms = [...algoMap.keys()];

	if (filters.metric === "throughput") {
		algorithms.sort((a, b) => {
			const aAvg =
				platformIds.reduce(
					(s, p) => s + (algoMap.get(a)?.get(p)?.throughput_bps ?? 0),
					0,
				) / platformIds.length;
			const bAvg =
				platformIds.reduce(
					(s, p) => s + (algoMap.get(b)?.get(p)?.throughput_bps ?? 0),
					0,
				) / platformIds.length;
			return aAvg - bAvg;
		});
	} else {
		algorithms.sort((a, b) => {
			const aAvg =
				platformIds.reduce(
					(s, p) => s + (algoMap.get(a)?.get(p)?.mean_ns ?? 0),
					0,
				) / platformIds.length;
			const bAvg =
				platformIds.reduce(
					(s, p) => s + (algoMap.get(b)?.get(p)?.mean_ns ?? 0),
					0,
				) / platformIds.length;
			return bAvg - aAvg;
		});
	}

	const series = platformIds.map((pid, pi) => ({
		name: platformMap.get(pid) ?? pid,
		type: "bar" as const,
		data: algorithms.map((algo) => {
			const b = algoMap.get(algo)?.get(pid);
			return b ? getValue(b, filters.metric) : 0;
		}),
		itemStyle: { color: PLATFORM_COLORS[pi % PLATFORM_COLORS.length] },
		barMaxWidth: 20,
		label:
			platformIds.length <= 2
				? {
						show: true,
						position: "right" as const,
						fontSize: 9,
						formatter: (p: { value: number }) =>
							formatValue(p.value, filters.metric),
					}
				: { show: false },
	}));

	return {
		tooltip: {
			trigger: "axis" as const,
			axisPointer: { type: "shadow" as const },
			formatter(
				params: Array<{ name: string; seriesName: string; value: number }>,
			) {
				const algo = params[0].name;
				let html = `<b>${algo}</b> (${categories[algo] ?? "unknown"})<br/>`;
				for (const p of params) {
					if (p.value > 0) {
						html += `${p.seriesName}: ${formatValue(p.value, filters.metric)}<br/>`;
					}
				}
				return html;
			},
		},
		legend: { top: 0 },
		grid: { left: 160, right: 100, top: 40, bottom: 40 },
		xAxis: {
			type: filters.logScale ? ("log" as const) : ("value" as const),
			name: filters.metric === "throughput" ? "Throughput" : "Latency",
			nameLocation: "middle" as const,
			nameGap: 25,
			axisLabel: {
				formatter: (val: number) => formatValue(val, filters.metric),
			},
		},
		yAxis: {
			type: "category" as const,
			data: algorithms,
			axisLabel: { fontSize: 11 },
			axisTick: { show: false },
		},
		series,
		animation: true,
		animationDuration: 300,
	};
}
