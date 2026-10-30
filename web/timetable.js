const $=id=>document.getElementById('tt-'+id);
const worker=new Worker(new URL('./timetable-worker.js',import.meta.url),{type:'module'});
let ready=false,revision=0,canonical=null,watchdog;
function state(s){$('status').textContent=s;}
function clear(){revision++;canonical=null;$('results').hidden=true;$('download').disabled=true;$('rows').replaceChildren();clearTimeout(watchdog);}
function fail(s){ready=false;clear();worker.terminate();$('run').disabled=true;$('reset').disabled=true;state(s);}
function inspect(){clear();if(!ready)return;state('Inspecting frozen timetable...');worker.postMessage({type:'inspect',id:revision,input:$('query').value});watchdog=setTimeout(()=>fail('Timetable engine timed out. Reload to retry.'),15000);}
function render(raw){const d=JSON.parse(raw);canonical=raw;$('direction').value=d.direction;$('long').checked=d.include_long_distance;$('segment').textContent=d.segment;$('summary').textContent=d.scheduled_departures+' printed departures; '+d.first_departure+' to '+d.last_departure+'. Effective '+d.source.effective_date+'.';$('metrics').textContent='Scheduled departure span: '+d.departure_span_minutes+' min. Largest within-day adjacent departure gap: '+d.largest_adjacent_gap_minutes+' min.';
 for(const r of d.rows){const tr=document.createElement('tr');for(const text of [r.page+' / '+r.column,r.train_number??'Unnumbered source slot',r.departure_printed,r.arrival_printed,r.long_distance?'SEA-LA long-distance':'Other printed corridor slot']){const td=document.createElement('td');td.textContent=text;tr.append(td);}$('rows').append(tr);}
 $('results').hidden=false;$('download').disabled=false;state('Ready / frozen timetable inspected by Rust');}
worker.onmessage=({data})=>{if(data.type==='ready'){ready=true;$('run').disabled=false;$('reset').disabled=false;inspect();return;}if(data.id===undefined){fail('Timetable engine could not load: '+data.message);return;}if(data.id!==revision)return;clearTimeout(watchdog);if(data.type==='result'){try{render(data.canonical);}catch{clear();state('Invalid timetable response.');}}else{clear();state('Invalid source query: '+data.message);}};
worker.onerror=()=>fail('Timetable engine could not load. Reload to retry.');
$('controls').addEventListener('submit',e=>{e.preventDefault();inspect();});
$('query').addEventListener('input',()=>{clear();state('Selection changed; inspect to continue.');});
for(const id of ['direction','long'])$(id).addEventListener('change',()=>{$('query').value=JSON.stringify({direction:$('direction').value,include_long_distance:$('long').checked});inspect();});
$('reset').addEventListener('click',()=>{$('query').value='{"direction":"southbound","include_long_distance":false}';inspect();});
$('download').addEventListener('click',()=>{if(!canonical)return;const url=URL.createObjectURL(new Blob([canonical+'\n'],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download='gauge-historical-schedule.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
watchdog=setTimeout(()=>fail('Timetable engine timed out. Reload to retry.'),15000);worker.postMessage({type:'init'});
