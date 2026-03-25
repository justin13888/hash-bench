export function formatBytes(bps: number): string {
	if (bps >= 1e12) return `${(bps / 1e12).toFixed(2)} TB/s`;
	if (bps >= 1e9) return `${(bps / 1e9).toFixed(2)} GB/s`;
	if (bps >= 1e6) return `${(bps / 1e6).toFixed(2)} MB/s`;
	if (bps >= 1e3) return `${(bps / 1e3).toFixed(2)} KB/s`;
	return `${bps.toFixed(0)} B/s`;
}

export function formatNs(ns: number): string {
	if (ns >= 1e9) return `${(ns / 1e9).toFixed(3)} s`;
	if (ns >= 1e6) return `${(ns / 1e6).toFixed(3)} ms`;
	if (ns >= 1e3) return `${(ns / 1e3).toFixed(3)} \u00B5s`;
	return `${ns.toFixed(1)} ns`;
}

export function formatValue(
	value: number,
	metric: "throughput" | "latency",
): string {
	return metric === "throughput" ? formatBytes(value) : formatNs(value);
}
