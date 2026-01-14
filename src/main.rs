use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    os::unix::io::AsRawFd,
};

fn main() {
    let f = File::open("./data/measurements.txt").unwrap();
    let map = mmap(&f);
    //let mut f=BufReader::new(f);
    let mut stats: HashMap<Vec<u8>, (i16, i64, usize, i16)> = HashMap::new();
    for line in map.split(|c| *c == b'\n') {
        let line = line;
        if line.is_empty() {
            break;
        }
        let mut fields = line.rsplitn(2, |c| *c == b';');
        //let temperature=fields.next().unwrap();
        //let station=fields.next().unwrap();
        let (Some(temperature), Some(station)) = (fields.next(), fields.next()) else {
            panic!("Invalid line: {:?}", unsafe {
                std::str::from_utf8_unchecked(line)
            });
        };

        //let (station,temperature)=line.rsplit(b';').unwrap();
        // Safety promised
        //let mut temperature:i16=0;
        //let mut skip=temperature.iter().copied();
        let skip = if temperature[0] == b'-' { 1 } else { 0 };
        let mut t: i16 = 0;
        let mut mul = 1;
        for &d in temperature.iter().rev() {
            match d {
                b'.' => {
                    continue;
                }
                b'-' => {
                    t = -t;
                    break;
                }
                _ => {
                    t += i16::from(d - b'0') * mul;
                    mul *= 10;
                }
            }
        }
        /*for i in [0,1,3]
        {

        }*/
        let skip = if temperature[0] == b'-' { 1 } else { 0 }; //let temperature:f64=unsafe {std::str::from_utf8_unchecked(temperature)}.parse().unwrap();
        //let stats=stats.entry(station.to_string()).or_insert((f64::MAX,0.0,0,f64::MIN));
        let stats = match stats.get_mut(station) {
            Some(stats) => stats,
            None => stats
                .entry(station.to_vec())
                .or_insert((i16::MAX, 0, 0, i16::MIN)),
        };
        stats.0 = stats.0.min(t);
        stats.1 += i64::from(t);
        stats.2 += 1;
        stats.3 = stats.3.max(t);
    }
    print!("{{");
    let mut stats: BTreeMap<String, (i16, i64, usize, i16)> = BTreeMap::from_iter(
        stats
            .into_iter()
            .map(|(station, stats)| (unsafe { String::from_utf8_unchecked(station) }, stats)),
    );
    //let mut stats=stats.into_iter().peekable();
    //stats.sort_unstable_by(|a,b| a.0.cmp(&b.0));
    let mut stats = stats.into_iter().peekable();
    //for ((station,temperature), (min,max,count,sum)) in stats{
    while let Some((station, (min, sum, count, max))) = stats.next() {
        print!(
            "{station}={:.1}/{:.1}{:.1}",
            (min as f64) / 10.,
            (sum as f64) / 10. / (count as f64),
            (max as f64) / 10.
        );
        if stats.peek().is_some() {
            print!(",");
        }
    }
    print!("}}");
    //}
}
fn mmap(f: &File) -> &'_ [u8] {
    let len = f.metadata().unwrap().len();
    let mut mmap = unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            len as libc::size_t,
            libc::PROT_READ,
            libc::MAP_SHARED,
            f.as_raw_fd(),
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("{:?}", std::io::Error::last_os_error());
        } else {
            if libc::madvise(ptr, len as libc::size_t, libc::MADV_SEQUENTIAL) != 0 {
                panic!("{:?}", std::io::Error::last_os_error());
            }
            return std::slice::from_raw_parts(ptr as *const u8, len as usize);
        }

        //std::ptr::from_raw_parts(ptr, f.metadata().unwrap().len() as usize)
    };
}
