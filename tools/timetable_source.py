"""Offline PDF and source-cell verification; no network or model promotion."""
import hashlib, json, re
from pathlib import Path
from pypdf import PdfReader
ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/'data/cascades-2023'
def times(line):
    return [(int(h)%12*60+int(m)+(720 if a=='pm' else 0),f'{h}:{m} {a}') for h,m,a in re.findall(r'(\d{1,2}):(\d{2})\s*(am|pm)',line)]
def verify():
    custody=json.loads((SOURCE/'custody.json').read_text(encoding='utf-8'))
    dataset=json.loads((SOURCE/'rows.json').read_text(encoding='utf-8'))
    raw=(SOURCE/'schedule.pdf').read_bytes()
    if len(raw)!=custody['bytes'] or hashlib.sha256(raw).hexdigest()!=custody['sha256'] or dataset['source_sha256']!=custody['sha256']:raise ValueError('Original PDF custody mismatch')
    reader=PdfReader(SOURCE/'schedule.pdf')
    if len(reader.pages)!=2 or len(dataset['rows'])!=14:raise ValueError('Unexpected source shape')
    for i,direction in enumerate(['southbound','northbound']):
        text=reader.pages[i].extract_text(extraction_mode='layout');lines=text.splitlines()
        for token in [direction.upper(),'December 11, 2023','Weekdays and weekends, all schedule times are the same.','SEA-LA long-distance train']:
            if token not in text:raise ValueError('Source heading/legend changed: '+token)
        sea=[l for l in lines if 'Seattle, WA' in l];pdx=[l for l in lines if 'Portland, OR' in l]
        departures=times(sea[1] if i==0 else pdx[0]);arrivals=times(pdx[0] if i==0 else sea[1])
        keys=['503','505'] if i==0 else ['500','502','504']
        header=next(l for l in lines if all(k in l for k in keys))
        numbers=[n if n.isdigit() else None for n in header.split()]
        rows=[r for r in dataset['rows'] if r['direction']==direction]
        if len(departures)!=7 or len(arrivals)!=7 or len(numbers)!=7 or len(rows)!=7:raise ValueError('Seven source columns required')
        for column,(row,dep,arr,num) in enumerate(zip(rows,departures,arrivals,numbers),1):
            if (row['page'],row['column'],row['train_number'],row['departure_minute'],row['departure_printed'],row['arrival_minute'],row['arrival_printed'])!=(i+1,column,num,*dep,*arr):raise ValueError('Source cell mismatch')
            if row['long_distance']!=(num in ['11','14']):raise ValueError('Long-distance column mismatch')
    print('Original PDF hash, two printed calendars/legends and all fourteen number/time columns verified offline')
    return dataset
if __name__=='__main__':verify()
