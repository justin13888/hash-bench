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
import {
	algoKey,
	displayName,
	formatBytes,
	formatNs,
	formatValue,
} from "../lib/format";
import type { AlgorithmMeta, BenchmarkResult, FilterState } from "../types";

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
	algorithms: Record<string, AlgorithmMeta>;
	platformMap: Map<string, string>;
	filters: FilterState;
}

function getValue(b: BenchmarkResult, metric: "throughput" | "latency") {
	return metric === "throughput" ? b.throughput_bps : b.mean_ns;
}

export default function BenchmarkChart({
	benchmarks,
	algorithms,
	platformMap,
	filters,
}: Props) {
	const multiPlatform = filters.selectedPlatforms.size > 1;

	const option = useMemo(() => {
		if (benchmarks.length === 0) return null;

		if (multiPlatform) {
			return buildComparisonOption(
				benchmarks,
				algorithms,
				platformMap,
				filters,
			);
		}
		return buildSingleOption(benchmarks, algorithms, filters);
	}, [benchmarks, algorithms, platformMap, filters, multiPlatform]);

	const chartHeight = useMemo(() => {
		const uniqueAlgos = new Set(
			benchmarks.map((b) => algoKey(b.algorithm, b.variant)),
		).size;
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
	algorithms: Record<string, AlgorithmMeta>,
	filters: FilterState,
) {
	// Deduplicate by (algorithm, variant) — one row per implementation per platform.
	const byAlgo = new Map<string, BenchmarkResult>();
	for (const b of benchmarks) {
		byAlgo.set(algoKey(b.algorithm, b.variant), b);
	}

	const entries = [...byAlgo.values()];
	if (filters.metric === "throughput") {
		entries.sort((a, b) => a.throughput_bps - b.throughput_bps);
	} else {
		entries.sort((a, b) => b.mean_ns - a.mean_ns);
	}

	const labels = entries.map((e) => displayName(e.algorithm, e.variant));
	const values = entries.map((e) => getValue(e, filters.metric));
	const colors = entries.map((e) =>
		algorithms[algoKey(e.algorithm, e.variant)]?.category === "cryptographic"
			? CRYPTO_COLOR
			: NONCRYPTO_COLOR,
	);

	const errorData = entries.map((e) => {
		if (filters.metric === "throughput") {
			return [
				e.size_bytes / (e.ci_upper_ns * 1e-9),
				e.size_bytes / (e.ci_lower_ns * 1e-9),
			];
		}
		return [e.ci_lower_ns, e.ci_upper_ns];
	});

	return {
		tooltip: {
			trigger: "axis" as const,
			axisPointer: { type: "shadow" as const },
			formatter(params: Array<{ dataIndex: number }>) {
				const idx = params[0].dataIndex;
				const b = entries[idx];
				const cat =
					algorithms[algoKey(b.algorithm, b.variant)]?.category ?? "unknown";
				return (
					`<b>${displayName(b.algorithm, b.variant)}</b><br/>` +
					`Category: ${cat}<br/>` +
					`Throughput: ${formatBytes(b.throughput_bps)}<br/>` +
					`Mean latency: ${formatNs(b.mean_ns)}<br/>` +
					`Median latency: ${formatNs(b.median_ns)}<br/>` +
					`95% CI: ${formatNs(b.ci_lower_ns)} \u2013 ${formatNs(b.ci_upper_ns)}`
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
			data: labels,
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
	algorithms: Record<string, AlgorithmMeta>,
	platformMap: Map<string, string>,
	filters: FilterState,
) {
	// Key by (algorithm, variant) so e.g. SHA-256 [sw] and SHA-256 [sha-ext]
	// remain distinct rows in the cross-platform comparison.
	const algoMap = new Map<string, Map<string, BenchmarkResult>>();
	for (const b of benchmarks) {
		const k = algoKey(b.algorithm, b.variant);
		if (!algoMap.has(k)) algoMap.set(k, new Map());
		algoMap.get(k)?.set(b.platform, b);
	}

	const platformIds = [...filters.selectedPlatforms];
	const keys = [...algoMap.keys()];

	if (filters.metric === "throughput") {
		keys.sort((a, b) => {
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
		keys.sort((a, b) => {
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
		data: keys.map((algo) => {
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
				// `params[0].name` is the `(algorithm, variant)` join key set on
				// the yAxis data below.
				const key = params[0].name;
				const [algo, variant] = key.split("|");
				let html = `<b>${displayName(algo, variant)}</b> (${
					algorithms[key]?.category ?? "unknown"
				})<br/>`;
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
			data: keys,
			axisLabel: {
				fontSize: 11,
				formatter: (key: string) => {
					const [algo, variant] = key.split("|");
					return displayName(algo, variant);
				},
			},
			axisTick: { show: false },
		},
		series,
		animation: true,
		animationDuration: 300,
	};
}
