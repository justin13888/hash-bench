/**
 * Build-time script: reads ../results/ Criterion JSON files and produces
 * public/data/benchmarks.json for the web app.
 */
import { readdir, readFile, mkdir, writeFile, stat } from "node:fs/promises";
import { join, resolve } from "node:path";

const RESULTS_DIR = resolve(import.meta.dirname, "../../results");
const OUTPUT_DIR = resolve(import.meta.dirname, "../public/data");
const OUTPUT_FILE = join(OUTPUT_DIR, "benchmarks.json");

// ── Types matching the web app's expected format ────────────────────────────

interface ConfidenceInterval {
  lower_bound: number;
  upper_bound: number;
}

interface Estimate {
  point_estimate: number;
  confidence_interval: ConfidenceInterval;
}

interface EstimatesJson {
  mean: Estimate;
  median: Estimate;
}

interface BenchmarkJson {
  group_id: string;
  function_id: string;
  value_str: string;
  throughput: { Bytes: number };
}

interface BenchmarkResult {
  platform: string;
  thread_count: number;
  algorithm: string;
  size: string;
  size_bytes: number;
  mean_ns: number;
  median_ns: number;
  mean_lower_ns: number;
  mean_upper_ns: number;
  throughput_bps: number;
}

interface PlatformInfo {
  id: string;
  display_name: string;
  thread_counts: number[];
  sizes: string[];
}

interface ReportData {
  generated_at: string;
  platforms: PlatformInfo[];
  benchmarks: BenchmarkResult[];
  categories: Record<string, string>;
}

// ── Category mapping ────────────────────────────────────────────────────────

const CRYPTO_ALGORITHMS = new Set([
  "BLAKE3",
  "BLAKE3 (rayon)",
  "BLAKE2b512",
  "BLAKE2b256",
  "BLAKE2s256",
  "SHA-1",
  "SHA-224",
  "SHA-256",
  "SHA-384",
  "SHA-512",
  "SHA-512/224",
  "SHA-512/256",
  "SHA3-224",
  "SHA3-256",
  "SHA3-384",
  "SHA3-512",
  "SHAKE128",
  "SHAKE256",
  "Keccak-224",
  "Keccak-256",
  "Keccak-384",
  "Keccak-512",
  "MD5",
  "RIPEMD-128",
  "RIPEMD-160",
  "RIPEMD-256",
  "RIPEMD-320",
  "SM3",
  "Streebog-256",
  "Streebog-512",
  "Whirlpool",
  "Ascon-Hash256",
  "Tiger2", // legacy
]);

function getCategory(algorithm: string): string {
  return CRYPTO_ALGORITHMS.has(algorithm)
    ? "cryptographic"
    : "non-cryptographic";
}

const PLATFORM_NAMES: Record<string, string> = {
  "amd-7900x": "AMD Ryzen 9 7900X",
  "mbp-m3-36gb": "Apple M3 MacBook Pro",
  "m3-pro-macbook": "Apple M3 Pro MacBook Pro",
};

function getPlatformDisplayName(id: string): string {
  return PLATFORM_NAMES[id] ?? id;
}

// ── Directory walking ───────────────────────────────────────────────────────

function parseThreadCount(groupId: string): number | null {
  const match = groupId.match(/^(\d+)-threaded/);
  return match ? parseInt(match[1], 10) : null;
}

async function exists(path: string): Promise<boolean> {
  try {
    await stat(path);
    return true;
  } catch {
    return false;
  }
}

async function* walkEstimates(dir: string): AsyncGenerator<string> {
  try {
    const entries = await readdir(dir, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = join(dir, entry.name);
      if (entry.isDirectory()) {
        yield* walkEstimates(fullPath);
      } else if (
        entry.name === "estimates.json" &&
        dir.endsWith("/new")
      ) {
        yield fullPath;
      }
    }
  } catch {
    // Directory doesn't exist or isn't readable
  }
}

async function loadResults(): Promise<BenchmarkResult[]> {
  const results: BenchmarkResult[] = [];

  if (!(await exists(RESULTS_DIR))) {
    console.log("No results/ directory found. Generating empty data.");
    return results;
  }

  const platforms = await readdir(RESULTS_DIR, { withFileTypes: true });

  for (const platformEntry of platforms) {
    if (!platformEntry.isDirectory()) continue;
    const platformId = platformEntry.name;
    const platformDir = join(RESULTS_DIR, platformId);

    for await (const estimatesPath of walkEstimates(platformDir)) {
      const newDir = estimatesPath.replace(/\/estimates\.json$/, "");
      const benchmarkPath = join(newDir, "benchmark.json");

      if (!(await exists(benchmarkPath))) continue;

      try {
        const estimatesRaw = await readFile(estimatesPath, "utf-8");
        const benchmarkRaw = await readFile(benchmarkPath, "utf-8");

        const estimates: EstimatesJson = JSON.parse(estimatesRaw);
        const benchmark: BenchmarkJson = JSON.parse(benchmarkRaw);

        const threadCount = parseThreadCount(benchmark.group_id);
        if (threadCount === null) continue;

        const throughputBytes = benchmark.throughput.Bytes;
        const meanNs = estimates.mean.point_estimate;
        const throughputBps = throughputBytes / (meanNs * 1e-9);

        results.push({
          platform: platformId,
          thread_count: threadCount,
          algorithm: benchmark.function_id,
          size: benchmark.value_str,
          size_bytes: throughputBytes,
          mean_ns: meanNs,
          median_ns: estimates.median.point_estimate,
          mean_lower_ns: estimates.mean.confidence_interval.lower_bound,
          mean_upper_ns: estimates.mean.confidence_interval.upper_bound,
          throughput_bps: throughputBps,
        });
      } catch {
        // Skip malformed files
      }
    }
  }

  results.sort((a, b) =>
    a.platform.localeCompare(b.platform) ||
    a.thread_count - b.thread_count ||
    a.algorithm.localeCompare(b.algorithm) ||
    a.size_bytes - b.size_bytes
  );

  return results;
}

function buildReportData(benchmarks: BenchmarkResult[]): ReportData {
  const platformThreads = new Map<string, Set<number>>();
  const platformSizes = new Map<string, Set<string>>();
  const categories: Record<string, string> = {};

  for (const b of benchmarks) {
    if (!platformThreads.has(b.platform)) {
      platformThreads.set(b.platform, new Set());
      platformSizes.set(b.platform, new Set());
    }
    platformThreads.get(b.platform)!.add(b.thread_count);
    platformSizes.get(b.platform)!.add(b.size);
    categories[b.algorithm] = getCategory(b.algorithm);
  }

  const platforms: PlatformInfo[] = [...platformThreads.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([id, threads]) => ({
      id,
      display_name: getPlatformDisplayName(id),
      thread_counts: [...threads].sort((a, b) => a - b),
      sizes: [...platformSizes.get(id)!],
    }));

  return {
    generated_at: new Date().toISOString(),
    platforms,
    benchmarks,
    categories,
  };
}

// ── Main ────────────────────────────────────────────────────────────────────

const benchmarks = await loadResults();
console.log(`Loaded ${benchmarks.length} benchmark results.`);

const reportData = buildReportData(benchmarks);
if (reportData.platforms.length > 0) {
  console.log(
    `Platforms: ${reportData.platforms.map((p) => `${p.display_name} (${p.thread_counts.length} thread configs)`).join(", ")}`
  );
}

await mkdir(OUTPUT_DIR, { recursive: true });
await writeFile(OUTPUT_FILE, JSON.stringify(reportData));
console.log(`Written to ${OUTPUT_FILE}`);
