"""Validate and publish derivative corpus custody, without claiming timetable proof."""
import hashlib
import html
import json
import math
from pathlib import Path
import re
import shutil

ROOT = Path(__file__).resolve().parents[1]
ORIGINAL_SHA256 = '33270912db6e861169d34380d18453f41f70b4d5974b74de16cf0910a26ea7d7'
ORIGINAL = ROOT / 'data/history/2026-06-frequency-span-of-service-original.md'


def audit():
    if hashlib.sha256(ORIGINAL.read_bytes()).hexdigest() != ORIGINAL_SHA256:
        raise ValueError('Original finding bytes changed')
    rows = []
    for file in sorted((ROOT / 'corpus').glob('us-*.md')):
        text = file.read_text(encoding='utf-8')
        identity = re.search(r'^id:\s*(.+)$', text, re.M).group(1).strip()
        name = re.search(r'^# (.+)$', text, re.M).group(1).strip()
        tier = re.search(r'^tier:\s*(.+)$', text, re.M).group(1).strip()
        score = float(re.search(r'^score: DIM-07\s*\|\s*([0-9.]+)', text, re.M).group(1))
        quantities = re.findall(r'^quantity:\s*(.+)$', text, re.M)
        if len(quantities) != 1:
            raise ValueError('Expected one retained frequency quantity per entry')
        value, unit, label, source = [part.strip() for part in quantities[0].split('|')]
        value = float(value)
        if (unit, label, source) != ('round-trips-per-day', 'source-needed', 'wikipedia-amtrak-routes'):
            raise ValueError('Unverified original frequency must retain its ID and source-needed label')
        if not math.isfinite(value) or not 0 <= value <= 32 or not 0 <= score <= 10:
            raise ValueError('Invalid retained model value')
        rows.append({'id': identity, 'slug': file.stem, 'name': name, 'tier': tier,
                     'retained_round_trips_per_day': value, 'illustrative_score': score,
                     'unit': unit, 'evidence_label': label, 'source_id': source,
                     'primary_source_verified': False, 'primary_source_locator': None,
                     'primary_source_sha256': None, 'observation_window': None,
                     'operating_days_per_week': None, 'measured_span_hours': None,
                     'corpus_path': file.relative_to(ROOT).as_posix(),
                     'published_corpus': 'evidence/' + file.name,
                     'corpus_sha256': hashlib.sha256(file.read_bytes()).hexdigest()})
    if len(rows) != 12 or len({row['id'] for row in rows}) != 12:
        raise ValueError('Expected twelve unique historical corpus entries')
    return {'schema': 'gauge.historical-source-custody-audit.v1', 'audit_date': '2026-10-26',
            'quantity_count': 12, 'verified_primary_quantities': 0, 'source_needed_quantities': 12,
            'scope': 'selected historical model entries; frequency proxy only; no national representativeness',
            'original_finding': {'published': 'evidence/original-finding.md', 'sha256': ORIGINAL_SHA256,
                                 'status': 'superseded; national-network and intervention-target conclusions withdrawn'},
            'entries': rows}


def publish(output, source_audit):
    output = Path(output)
    (output / 'evidence').mkdir(exist_ok=True)
    custody = []
    sources = [(ROOT / row['corpus_path'], row['published_corpus']) for row in source_audit['entries']]
    sources += [(ROOT / 'data/sources.md', 'evidence/sources.md'),
                (ROOT / 'data/source-audit.md', 'evidence/source-audit.md'),
                (ROOT / 'docs/findings/2026-06-frequency-span-of-service.md', 'evidence/finding.md'),
                (ORIGINAL, 'evidence/original-finding.md')]
    for file, target in sources:
        shutil.copy2(file, output / target)
        custody.append({'source': file.relative_to(ROOT).as_posix(), 'published': target,
                        'sha256': hashlib.sha256(file.read_bytes()).hexdigest()})
    (output / 'source-custody.json').write_bytes((json.dumps(custody, indent=2) + '\n').encode('utf-8'))
    (output / 'source-audit.json').write_bytes((json.dumps(source_audit, indent=2) + '\n').encode('utf-8'))
    table_rows = []
    for row in source_audit['entries']:
        table_rows.append('<tr><th scope="row"><a href="' + html.escape(row['published_corpus'], quote=True)
                          + '">' + html.escape(row['name']) + '</a></th><td>'
                          + str(row['retained_round_trips_per_day']) + '</td><td>'
                          + str(row['illustrative_score']) + '</td><td>Source needed</td>'
                          + '<td>Unobserved</td></tr>')
    page = (output / 'index.html').read_text(encoding='utf-8')
    if page.count('<!-- SOURCE_AUDIT_ROWS -->') != 1:
        raise ValueError('Missing or duplicate audit table marker')
    (output / 'index.html').write_bytes(page.replace('<!-- SOURCE_AUDIT_ROWS -->', ''.join(table_rows)).encode('utf-8'))
