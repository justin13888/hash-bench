import { shareFile } from "@choochmeque/tauri-plugin-sharekit-api";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

/** Mirrors the `BenchOutput` returned by the `run_bench` Tauri command. */
interface BenchOutput {
	/** Pretty-printed results.json (schema v3). */
	json: string;
	/** Suggested file name, e.g. `pixel-8-results.json`. */
	filename: string;
	/** Absolute path the report was written to, for the share step. */
	cached_path: string;
}

type Preset = "quick" | "full";
type Status = "idle" | "running" | "done" | "error";

const PRESETS: Record<Preset, { label: string; blurb: string }> = {
	quick: {
		label: "Quick",
		blurb:
			"1 KiB–1 MiB · 1 thread · 10 samples. Fast, noisier — exploration only.",
	},
	full: {
		label: "Full",
		blurb:
			"64 B–10 MiB · 1/physical/logical threads · 30 samples. Committable to results/.",
	},
};

export default function App() {
	const [machineId, setMachineId] = useState("");
	const [cpuModel, setCpuModel] = useState("");
	const [preset, setPreset] = useState<Preset>("quick");
	const [status, setStatus] = useState<Status>("idle");
	const [message, setMessage] = useState("");

	const running = status === "running";
	const trimmedId = machineId.trim();

	async function run() {
		if (!trimmedId) {
			setStatus("error");
			setMessage("Enter a machine id first (e.g. pixel-8).");
			return;
		}
		setStatus("running");
		setMessage(
			`Benchmarking (${PRESETS[preset].label})… this can take a while.`,
		);
		try {
			const out = await invoke<BenchOutput>("run_bench", {
				machineId: trimmedId,
				cpuModel: cpuModel.trim() || null,
				preset,
			});
			setStatus("done");
			setMessage(`Wrote ${out.filename}. Opening share sheet…`);
			try {
				await shareFile(out.cached_path, {
					mimeType: "application/json",
					title: out.filename,
				});
			} catch (shareErr) {
				// Sharing is best-effort (e.g. unavailable on desktop dev builds).
				// The file is already on disk, so surface its path as a fallback.
				setMessage(
					`Saved to ${out.cached_path}\n(share unavailable: ${String(shareErr)})`,
				);
			}
		} catch (err) {
			setStatus("error");
			setMessage(`Benchmark failed: ${String(err)}`);
		}
	}

	return (
		<main className="mx-auto flex min-h-full max-w-md flex-col gap-6 p-6">
			<header className="flex flex-col gap-1">
				<h1 className="font-bold text-2xl tracking-tight">hash-bench</h1>
				<p className="text-sm opacity-70">
					Run the hashing benchmark on this device and export a results.json.
				</p>
			</header>

			<div className="flex flex-col gap-2">
				<label className="font-medium text-sm" htmlFor="machine-id">
					Machine id
				</label>
				<input
					id="machine-id"
					className="rounded-lg border border-black/20 bg-transparent px-3 py-2 text-base dark:border-white/20"
					type="text"
					inputMode="text"
					autoCapitalize="none"
					autoCorrect="off"
					placeholder="e.g. pixel-8"
					value={machineId}
					disabled={running}
					onChange={(e) => setMachineId(e.target.value)}
				/>
				<p className="text-xs opacity-60">
					Becomes the <code>results/&lt;id&gt;/results.json</code> folder and{" "}
					<code>platform.id</code>.
				</p>
			</div>

			<div className="flex flex-col gap-2">
				<label className="font-medium text-sm" htmlFor="cpu-model">
					CPU model <span className="opacity-60">(optional)</span>
				</label>
				<input
					id="cpu-model"
					className="rounded-lg border border-black/20 bg-transparent px-3 py-2 text-base dark:border-white/20"
					type="text"
					placeholder="e.g. Snapdragon 8 Gen 3 / Apple A17 Pro"
					value={cpuModel}
					disabled={running}
					onChange={(e) => setCpuModel(e.target.value)}
				/>
			</div>

			<fieldset className="flex flex-col gap-2 border-0 p-0" disabled={running}>
				<legend className="font-medium text-sm">Preset</legend>
				<div className="flex gap-2">
					{(Object.keys(PRESETS) as Preset[]).map((key) => (
						<button
							key={key}
							type="button"
							aria-pressed={preset === key}
							onClick={() => setPreset(key)}
							className={`flex-1 rounded-lg border px-3 py-2 font-medium text-sm transition-colors ${
								preset === key
									? "border-transparent bg-black text-white dark:bg-white dark:text-black"
									: "border-black/20 dark:border-white/20"
							}`}
						>
							{PRESETS[key].label}
						</button>
					))}
				</div>
				<p className="text-xs opacity-60">{PRESETS[preset].blurb}</p>
			</fieldset>

			<button
				type="button"
				onClick={run}
				disabled={running}
				className="rounded-xl bg-black px-4 py-3 font-semibold text-base text-white disabled:opacity-50 dark:bg-white dark:text-black"
			>
				{running ? "Running…" : "Run & Export"}
			</button>

			{message && (
				<p
					className={`whitespace-pre-wrap text-sm ${
						status === "error" ? "text-red-600 dark:text-red-400" : "opacity-80"
					}`}
				>
					{message}
				</p>
			)}
		</main>
	);
}
