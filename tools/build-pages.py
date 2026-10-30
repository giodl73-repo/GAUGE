"""Build GAUGE's bounded frequency explorer from its native Rust core."""
import argparse
import runpy
import json
import shutil
import subprocess
from pathlib import Path

from timetable_source import verify as verify_timetable

ROOT = Path(__file__).resolve().parents[1]


def run(*args):
    subprocess.run(args, cwd=ROOT, check=True)


def main():
    custody_tools = runpy.run_path(str(ROOT / 'tools/source-custody.py'))
    source_audit = custody_tools['audit']()
    verify_timetable()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/GAUGE"))
    args = parser.parse_args()
    output = args.output.resolve()
    if output == ROOT / "dist" or ROOT / "dist" not in output.parents:
        raise ValueError("Output must be a directory inside dist")
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise ValueError(f"Expected wasm-bindgen 0.2.127, found {version}")
    run("cargo", "build", "--locked", "--release", "--target", "wasm32-unknown-unknown", "-p", "gauge-web", "--lib", "--features", "wasm")
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT))
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release/gauge_web.wasm"
    output.mkdir(parents=True, exist_ok=True)
    run("wasm-bindgen", str(wasm), "--target", "web", "--out-dir", str(output / "pkg"))
    for name in ("index.html", "style.css", "app.js", "worker.js", "timetable.js", "timetable-worker.js"):
        shutil.copy2(ROOT / "web" / name, output / name)
    shutil.copy2(ROOT / "LICENSE", output / "LICENSE")
    shutil.copy2(ROOT / "THIRD-PARTY-LICENSES.txt", output / "THIRD-PARTY-LICENSES.txt")
    def capture(*args):
        return subprocess.check_output(args, cwd=ROOT, text=True, encoding="utf-8").strip()
    # Native canonical references also guard all legacy numeric/scenario fields.
    cache = ROOT / '.cache'
    cache.mkdir(exist_ok=True)
    previous = json.loads((ROOT / 'fixtures/pre-correction-results.json').read_text(encoding='utf-8'))
    references = []
    for name, old in zip(['baseline', 'single', 'portfolio', 'exact-bar'], previous['cases'], strict=True):
        path = f'fixtures/browser-{name}.json'
        canonical = capture('cargo', 'run', '--locked', '--quiet', '-p', 'gauge-web', '--bin', 'gauge-browser', '--', path)
        result = json.loads(canonical)
        legacy = json.loads(canonical)
        legacy.pop('evidence')
        for run_name in ['baseline', 'scenario']:
            for row in legacy[run_name]['corridors']:
                row.pop('historical_evidence_label')
                row.pop('historical_primary_source_verified')
        if legacy != old['result']:
            raise ValueError('Existing numeric/scenario output changed: ' + name)
        references.append({'input': (ROOT / path).read_text(encoding='utf-8'), 'canonical': canonical})
    native_rows = {row['slug']: (row['historical_trips'], row['score'], row['historical_evidence_label']) for row in result['baseline']['corridors']}
    audit_rows = {row['slug']: (row['retained_round_trips_per_day'], row['illustrative_score'], row['evidence_label']) for row in source_audit['entries']}
    if native_rows != audit_rows:
        raise ValueError('Published audit differs from the embedded native corpus')
    (cache / 'native-results.json').write_bytes((json.dumps(references) + '\n').encode('utf-8'))
    custody_tools['publish'](output, source_audit)
    timetable_refs=[]
    for direction in ['southbound','northbound']:
        for include in ['false','true']:
            path=f'fixtures/timetable-{direction}-{include}.json'
            canonical=capture('cargo','run','--locked','--quiet','-p','gauge-web','--bin','gauge-timetable','--',path)
            timetable_refs.append({'input':(ROOT/path).read_text(encoding='utf-8'),'canonical':canonical})
    (cache/'timetable-results.json').write_bytes((json.dumps(timetable_refs)+'\n').encode())
    shutil.copytree(ROOT/'data/cascades-2023',output/'schedule',dirs_exist_ok=True)
    provenance = {"commit": capture("git", "rev-parse", "HEAD"), "dirty": bool(capture("git", "status", "--porcelain")), "rust": capture("rustc", "--version"), "bindings": version, "source": "https://github.com/giodl73-repo/GAUGE"}
    (output / "provenance.json").write_text(json.dumps(provenance, indent=2), encoding="utf-8")
    (output / ".nojekyll").touch()
    size = sum(path.stat().st_size for path in output.rglob("*") if path.is_file())
    if size >= 5_000_000:
        raise ValueError("Explorer exceeds its 5 MB delivery budget")
    print(f"Pages explorer: {size:,} bytes at {output}")


if __name__ == "__main__":
    main()
