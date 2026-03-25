import type {
  CategoryFilter,
  FilterState,
  Metric,
  PlatformInfo,
} from "../types";

interface Props {
  platforms: PlatformInfo[];
  filters: FilterState;
  allThreadCounts: number[];
  allSizes: string[];
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
  onFilterChange,
  onPlatformToggle,
}: Props) {
  return (
    <div className="mb-4 flex flex-wrap items-end gap-4 rounded-lg border border-gray-200 bg-gray-50 p-4 dark:border-gray-800 dark:bg-gray-900">
      {/* Platforms */}
      <div className="flex flex-col gap-1">
        <label className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
          Platforms
        </label>
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
        <label className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
          Threads
        </label>
        <select
          value={filters.threadCount}
          onChange={(e) =>
            onFilterChange("threadCount", parseInt(e.target.value))
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
        <label className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
          Data Size
        </label>
        <select
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
        <label className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
          Category
        </label>
        <select
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
        <label className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
          Metric
        </label>
        <div className="flex gap-0.5">
          {(["throughput", "latency"] as Metric[]).map((m) => (
            <button
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
        <label className="text-xs font-semibold tracking-wide text-gray-500 uppercase dark:text-gray-400">
          Scale
        </label>
        <div className="flex gap-0.5">
          {[true, false].map((isLog) => (
            <button
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
  );
}
