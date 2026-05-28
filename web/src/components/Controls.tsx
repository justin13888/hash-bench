import { useState } from "react";
import type {
	CategoryFilter,
	FilterState,
	HwAccelFilter,
	Metric,
	OutputKindFilter,
	PlatformInfo,
	TernaryFilter,
} from "../types";

interface Props {
	platforms: PlatformInfo[];
	filters: FilterState;
	allThreadCounts: number[];
	allSizes: string[];
	allVariants: string[];
	allOutputBits: number[];
	onFilterChange: <K extends keyof FilterState>(
		key: K,
		value: FilterState[K],
	) => void;
	onPlatformToggle: (id: string) => void;
}

export default function Controls({
	platforms,
	filters,
	allThreadCounts,
	allSizes,
	allVariants,
	allOutputBits,
	onFilterChange,
	onPlatformToggle,
}: Props) {
	const [advancedOpen, setAdvancedOpen] = useState(false);

	const toggleSetItem = <T extends string | number>(
		key: "variants" | "outputBits",
		set: Set<T>,
		value: T,
	) => {
		const next = new Set(set);
		if (next.has(value)) next.delete(value);
		else next.add(value);
		// FilterState fields here are typed Set<string>|Set<number>; the runtime
		// container is just a Set.
		onFilterChange(key, next as unknown as FilterState[typeof key]);
	};

	return (
		<div className="mb-4 rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-800 dark:bg-gray-900">
			<div className="flex flex-wrap items-end gap-4">
				{/* Platforms */}
				<div className="flex flex-col gap-1">
					<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
						Platforms
					</span>
					<div className="flex flex-wrap gap-3">
						{platforms.map((p) => (
							<label
								key={p.id}
								className="flex cursor-pointer items-center gap-1.5 text-sm"
							>
								<input
									type="checkbox"
									checked={filters.selectedPlatforms.has(p.id)}
									onChange={() => onPlatformToggle(p.id)}
									className="accent-blue-500"
								/>
								{p.display_name}
							</label>
						))}
					</div>
				</div>

				{/* Thread Count */}
				<div className="flex flex-col gap-1">
					<label
						htmlFor="filter-threads"
						className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400"
					>
						Threads
					</label>
					<select
						id="filter-threads"
						value={filters.threadCount}
						onChange={(e) =>
							onFilterChange("threadCount", parseInt(e.target.value, 10))
						}
						className="rounded border border-gray-300 bg-white px-2 py-1.5 text-sm dark:border-gray-700 dark:bg-gray-800"
					>
						{allThreadCounts.map((t) => (
							<option key={t} value={t}>
								{t}-threaded
							</option>
						))}
					</select>
				</div>

				{/* Data Size */}
				<div className="flex flex-col gap-1">
					<label
						htmlFor="filter-size"
						className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400"
					>
						Data Size
					</label>
					<select
						id="filter-size"
						value={filters.size}
						onChange={(e) => onFilterChange("size", e.target.value)}
						className="rounded border border-gray-300 bg-white px-2 py-1.5 text-sm dark:border-gray-700 dark:bg-gray-800"
					>
						{allSizes.map((s) => (
							<option key={s} value={s}>
								{s}
							</option>
						))}
					</select>
				</div>

				{/* Category */}
				<div className="flex flex-col gap-1">
					<label
						htmlFor="filter-category"
						className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400"
					>
						Category
					</label>
					<select
						id="filter-category"
						value={filters.category}
						onChange={(e) =>
							onFilterChange("category", e.target.value as CategoryFilter)
						}
						className="rounded border border-gray-300 bg-white px-2 py-1.5 text-sm dark:border-gray-700 dark:bg-gray-800"
					>
						<option value="all">All</option>
						<option value="cryptographic">Cryptographic</option>
						<option value="non-cryptographic">Non-cryptographic</option>
					</select>
				</div>

				{/* Metric Toggle */}
				<div className="flex flex-col gap-1">
					<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
						Metric
					</span>
					<div className="flex gap-0.5">
						{(["throughput", "latency"] as Metric[]).map((m) => (
							<button
								type="button"
								key={m}
								onClick={() => onFilterChange("metric", m)}
								className={`rounded px-3 py-1.5 text-sm font-medium ${
									filters.metric === m
										? "bg-blue-500 text-white"
										: "border border-gray-300 bg-white text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300"
								}`}
							>
								{m === "throughput" ? "Throughput" : "Latency"}
							</button>
						))}
					</div>
				</div>

				{/* Scale Toggle */}
				<div className="flex flex-col gap-1">
					<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
						Scale
					</span>
					<div className="flex gap-0.5">
						{[true, false].map((isLog) => (
							<button
								type="button"
								key={String(isLog)}
								onClick={() => onFilterChange("logScale", isLog)}
								className={`rounded px-3 py-1.5 text-sm font-medium ${
									filters.logScale === isLog
										? "bg-blue-500 text-white"
										: "border border-gray-300 bg-white text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300"
								}`}
							>
								{isLog ? "Log" : "Linear"}
							</button>
						))}
					</div>
				</div>
			</div>

			{/* Advanced filters — collapsed by default */}
			<div className="mt-3 border-t border-gray-200 pt-3 dark:border-gray-800">
				<button
					type="button"
					onClick={() => setAdvancedOpen((v) => !v)}
					className="text-xs font-semibold tracking-wide text-gray-500 uppercase hover:text-blue-500 dark:text-gray-400"
				>
					{advancedOpen ? "▼" : "▶"} Advanced filters
				</button>

				{advancedOpen && (
					<div className="mt-3 flex flex-wrap items-start gap-4">
						{/* Variants */}
						<div className="flex flex-col gap-1">
							<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
								Variants
							</span>
							<div className="flex flex-wrap gap-2">
								{allVariants.map((v) => {
									const active =
										filters.variants.size === 0 || filters.variants.has(v);
									return (
										<button
											type="button"
											key={v}
											onClick={() =>
												toggleSetItem("variants", filters.variants, v)
											}
											className={`rounded px-2 py-1 text-xs font-medium ${
												active
													? "bg-blue-500 text-white"
													: "border border-gray-300 bg-white text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300"
											}`}
										>
											{v}
										</button>
									);
								})}
							</div>
						</div>

						{/* HW acceleration */}
						<TernaryGroup
							label="HW acceleration"
							value={filters.hwAcceleration}
							options={[
								["all", "All"],
								["hw-only", "HW only"],
								["sw-only", "SW only"],
							]}
							onChange={(v) =>
								onFilterChange("hwAcceleration", v as HwAccelFilter)
							}
						/>

						{/* Output bits */}
						<div className="flex flex-col gap-1">
							<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
								Output bits
							</span>
							<div className="flex flex-wrap gap-1">
								{allOutputBits.map((bits) => {
									const active =
										filters.outputBits.size === 0 ||
										filters.outputBits.has(bits);
									return (
										<button
											type="button"
											key={bits}
											onClick={() =>
												toggleSetItem("outputBits", filters.outputBits, bits)
											}
											className={`rounded px-2 py-1 text-xs font-medium ${
												active
													? "bg-blue-500 text-white"
													: "border border-gray-300 bg-white text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300"
											}`}
										>
											{bits}
										</button>
									);
								})}
							</div>
						</div>

						{/* Output kind */}
						<TernaryGroup
							label="Output kind"
							value={filters.outputKind}
							options={[
								["all", "All"],
								["fixed", "Fixed"],
								["xof", "XOF"],
							]}
							onChange={(v) =>
								onFilterChange("outputKind", v as OutputKindFilter)
							}
						/>

						{/* Internally parallel */}
						<TernaryGroup
							label="Internally parallel"
							value={filters.internallyParallel}
							options={[
								["all", "All"],
								["yes", "Yes"],
								["no", "No"],
							]}
							onChange={(v) =>
								onFilterChange("internallyParallel", v as TernaryFilter)
							}
						/>

						{/* Keyed / DoS-resistant */}
						<div className="flex flex-col gap-1">
							<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
								Subset
							</span>
							<div className="flex flex-col gap-1 text-sm">
								<label className="flex cursor-pointer items-center gap-1.5">
									<input
										type="checkbox"
										checked={filters.keyedOnly}
										onChange={(e) =>
											onFilterChange("keyedOnly", e.target.checked)
										}
										className="accent-blue-500"
									/>
									Keyed only
								</label>
								<label className="flex cursor-pointer items-center gap-1.5">
									<input
										type="checkbox"
										checked={filters.dosResistantOnly}
										onChange={(e) =>
											onFilterChange("dosResistantOnly", e.target.checked)
										}
										className="accent-blue-500"
									/>
									DoS-resistant only
								</label>
							</div>
						</div>
					</div>
				)}
			</div>
		</div>
	);
}

function TernaryGroup({
	label,
	value,
	options,
	onChange,
}: {
	label: string;
	value: string;
	options: ReadonlyArray<readonly [string, string]>;
	onChange: (next: string) => void;
}) {
	return (
		<div className="flex flex-col gap-1">
			<span className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
				{label}
			</span>
			<div className="flex gap-0.5">
				{options.map(([v, l]) => (
					<button
						type="button"
						key={v}
						onClick={() => onChange(v)}
						className={`rounded px-2.5 py-1 text-xs font-medium ${
							value === v
								? "bg-blue-500 text-white"
								: "border border-gray-300 bg-white text-gray-700 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300"
						}`}
					>
						{l}
					</button>
				))}
			</div>
		</div>
	);
}
