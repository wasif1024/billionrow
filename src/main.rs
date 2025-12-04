use std::{fs::File, io::{BufReader, BufRead},collections::{HashMap, BTreeMap}};

fn main() {
    let f=File::open("./data/measurements.txt").unwrap();
let mut f=BufReader::new(f);
let mut stats: HashMap<Vec<u8>,(f64, f64,usize, f64)> = HashMap::new();
for line in f.split(b'\n'){
    let line=line.unwrap();
    let mut fields=line.rsplitn(2,|c|*c==b';');
    let temperature=fields.next().unwrap();
    let station=fields.next().unwrap();
    
    //let (station,temperature)=line.rsplit(b';').unwrap();
    let temperature:f64=std::str::from_utf8(temperature).unwrap().parse().unwrap();
    //let stats=stats.entry(station.to_string()).or_insert((f64::MAX,0.0,0,f64::MIN));
    let stats=match stats.get_mut(station){
        Some(stats)=>stats,
        None=>{
            stats.entry(station.to_vec()).or_insert((f64::MAX,0.0,0,f64::MIN))
        }
    };
    stats.0=stats.0.min(temperature);
    stats.1+=temperature;
    stats.2+=1;
    stats.3=stats.3.max(temperature);
}
print!("{{");
let mut stats:  BTreeMap<String,(f64, f64,usize, f64)> =BTreeMap::from_iter(stats.into_iter().map(|(station,stats)| (String::from_utf8(station).unwrap(), stats)));
//let mut stats=stats.into_iter().peekable();
//stats.sort_unstable_by(|a,b| a.0.cmp(&b.0));
let mut stats=stats.into_iter().peekable();
//for ((station,temperature), (min,max,count,sum)) in stats{
while let Some((station,(min,sum,count,max)))=stats.next(){
    print!("{station}={min}/{}{max}",sum/(count as f64));
    if stats.peek().is_some(){
        print!(",");
    }
}
print!("}}");
//}
}
