const $=id=>document.getElementById(id);
const worker=new Worker(new URL('./worker.js',import.meta.url),{type:'module'});
let ready=false,latest=0,result=null,timer,loaded=false,historical=[],portfolio={};
const defaults={corridor:0,round_trips:1,bar:7};
const params=new URLSearchParams(location.search);
let initial={...defaults};
for(const [key,min,max] of [['corridor',0,11],['round_trips',0,32],['bar',0,10]]){if(params.has(key)){const raw=params.get(key),value=Number(raw);if(!raw.trim()||!Number.isFinite(value)||value<min||value>max||(key==='corridor'?!Number.isInteger(value):Math.abs(value*10-Math.round(value*10))>1e-8)){initial={...defaults};break;}initial[key]=value;}}
try {if(params.has('portfolio')) {const raw=params.get('portfolio');if(raw.length>4096)throw Error();const value=JSON.parse(raw);if(!value||Array.isArray(value)||typeof value!=='object'||Object.keys(value).length>12||Object.entries(value).some(([k,v])=>!/^us-[a-z-]+$/.test(k)||!Number.isFinite(v)||v<0||v>32||Math.abs(v*10-Math.round(v*10))>1e-8))throw Error();portfolio=value;}}catch{portfolio={};initial={...defaults};}
$('trips').value=initial.round_trips;$('bar').value=initial.bar;
function status(text,error=false){$('status').textContent=text;$('status').classList.toggle('error',error);}
function disable(){result=null;$('share').disabled=true;$('download').disabled=true;}
function input(){return {corridor:loaded?Number($('corridor').value):initial.corridor,round_trips:Number($('trips').value),bar:Number($('bar').value),portfolio:{...portfolio}};}
function labels(){$('trips-value').textContent=$('trips').value;$('bar-value').textContent=$('bar').value;}
function compute(){clearTimeout(timer);labels();disable();latest++;if(!ready)return;status('Computing scenario…');worker.postMessage({type:'evaluate',id:latest,input:input()});}
const classification=value=>value==='SystemicRegion'?'Systemic frequency deficit':value==='TailRegion'?'Concentrated frequency tail':value;
function render(data,ms){result=data;historical=data.baseline.corridors;$('edited').textContent=Object.keys(portfolio).length+' corridor edits retained';const b=data.baseline,s=data.scenario,idx=data.input.corridor;
 if(!loaded){const selected=s.corridors[idx];if(selected.trips!==b.corridors[idx].historical_trips)portfolio[selected.slug]??=selected.trips;$('edited').textContent=Object.keys(portfolio).length+' corridor edits retained';$('corridor').replaceChildren(...b.corridors.map((c,i)=>{const o=document.createElement('option');o.value=i;o.textContent=c.name;return o;}));$('corridor').value=idx;$('corridor').disabled=false;loaded=true;$('trips').disabled=false;$('trips').value=s.corridors[idx].trips;labels();$('minimum').disabled=false;}
 $('name').textContent=s.corridors[idx].name;$('score').textContent=b.corridors[idx].score.toFixed(1)+' → '+s.corridors[idx].score.toFixed(1);$('selected-detail').textContent='DIM-07, out of 10; your bar is '+data.input.bar.toFixed(1);
 $('below').textContent=b.below+' → '+s.below;$('classification').textContent=classification(b.classification)+' → '+classification(s.classification);
 $('rows').replaceChildren(...s.corridors.map((c,i)=>{const row=document.createElement('tr');row.classList.toggle('selected',i===idx);for(const value of [c.name,c.historical_trips,c.trips,c.score.toFixed(1),c.below_bar?'Yes':'No',c.tier||'Unassigned']){const td=document.createElement('td');td.textContent=value;row.append(td);}return row;}));
 const c=s.corridors[idx];const link=document.createElement('a');link.href='https://github.com/giodl73-repo/GAUGE/blob/master/corpus/'+c.slug+'.md';link.textContent='Historical corpus entry';$('source').replaceChildren(link,document.createTextNode(' · '+c.historical_source_id+' · 2023/2024 source basis. Scenario frequency is hypothetical.'));
 $('share').disabled=false;$('download').disabled=false;status('Ready · Rust calculation '+ms.toFixed(2)+' ms');
}
worker.onmessage=({data})=>{if(data.type==='ready'){ready=true;compute();}else if(data.type==='error'&&data.id===undefined){ready=false;disable();status('Rust engine could not load: '+data.message,true);}else if(data.id===latest){if(data.type==='result')render(data.result,data.ms);else status(data.message,true);}};
worker.onerror=()=>{ready=false;disable();status('Rust engine could not load. Reload to try again.',true);};
$('controls').addEventListener('submit',e=>e.preventDefault());
$('corridor').addEventListener('change',()=>{const c=historical[Number($('corridor').value)];if(c)$('trips').value=portfolio[c.slug]??c.historical_trips;compute();});
for(const id of ['trips','bar'])$(id).addEventListener('input',()=>{if(id==='trips'&&loaded)portfolio[historical[Number($('corridor').value)].slug]=Number($('trips').value);disable();latest++;labels();clearTimeout(timer);timer=setTimeout(compute,100);});
$('controls').addEventListener('reset',()=>{setTimeout(()=>{portfolio={};initial={...defaults};$('corridor').value=0;$('trips').value=1;$('bar').value=7;history.replaceState(null,'',location.pathname);compute();},0);});
$('minimum').addEventListener('click',()=>{if(!loaded)return;portfolio=Object.fromEntries(historical.map(c=>[c.slug,Math.max(12,c.historical_trips)]));$('trips').value=portfolio[historical[Number($('corridor').value)].slug];compute();});
$('share').addEventListener('click',async()=>{if(!result)return;const url=new URL(location.href);url.search='';url.hash='';for(const[key,value]of Object.entries(result.input))url.searchParams.set(key,key==='portfolio'?JSON.stringify(value):value);history.replaceState(null,'',url);try{await navigator.clipboard.writeText(url.href);status('Scenario link copied.');}catch{status('Scenario link is in the address bar.');}});
$('download').addEventListener('click',()=>{if(!result)return;const url=URL.createObjectURL(new Blob([JSON.stringify(result,null,2)+'\n'],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download='gauge-scenario.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
labels();worker.postMessage({type:'init'});
