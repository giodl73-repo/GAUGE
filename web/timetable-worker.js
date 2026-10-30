import init,{inspect_timetable_json} from './pkg/gauge_web.js';
self.onmessage=async({data})=>{try{if(data.type==='init'){await init();self.postMessage({type:'ready'});}else if(data.type==='inspect')self.postMessage({type:'result',id:data.id,canonical:inspect_timetable_json(data.input)});}catch(e){self.postMessage({type:'error',id:data.id,message:String(e)});}};
