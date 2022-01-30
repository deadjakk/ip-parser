use std::fs::File;
use std::error::Error;
use itertools::Itertools;
use structopt::StructOpt;
use std::io::{stdin,Read};
use std::net::IpAddr;
use ipnetwork::IpNetwork;
#[derive(StructOpt,Debug)]
#[structopt(name="ips",about="ips - tool for parsing and filtering ip addresses",author="jonathan peterson aka @deadjakk")]
struct Opt {
    /// non-unique output, keep any duplicates
    #[structopt(short,long)]
    keep_duplicates:bool,

    /// file that contains CIDRs & IP addrs against which to filter output 
    /// (include)
    #[structopt(short,long)]
    whitelist_file: Option<String>,

    /// file that contains CIDRs & IP addrs against which to filter output 
    /// (exlude)
    #[structopt(short,long)]
    blacklist_file: Option<String>,
 
    /// single or comma-separated CIDRs for blacklisting
    #[structopt(short,long)]
    xclude: Option<String>,

    /// single or comma-separated CIDRs for whitelisting
    #[structopt(short,long)]
    include: Option<String>,

    /// file path from which to parse ip addresses.  
    /// omitting this forces reading from stdin.  
    file_name: Option<String>,

}

fn main()->Result<(),Box<dyn Error>>{
    let options = Opt::from_args();

    let mut output = String::new();
    match options.file_name {
        Some(v) => {
            File::open(v).expect("failed to open file").read_to_string(&mut output).expect("could not read from file");
        }
        None => {
            stdin().lock().read_to_string(&mut output).expect("could not read from stdin");
        }
    }
    
    // parse the ip addresses
    let mut net_ips:Vec<IpAddr> = output.split_whitespace().filter_map(|token|{
            token.parse::<IpAddr>().ok()
            }
        ).collect();

    // whitelisting cidr collection
    let mut whitelist_cidrs: Vec<IpNetwork>= Vec::new();
    let file_whitelist_cidrs= get_cidrs(options.whitelist_file);
    let cs_whitelist_cidrs= get_cidrs_from_cs(options.include);
    if let Some(mut v)=file_whitelist_cidrs{
        whitelist_cidrs.append(&mut v);
    }
    if let Some(mut v)=cs_whitelist_cidrs{
        whitelist_cidrs.append(&mut v);
    }
    
    // blacklisting cidr collection
    let mut blacklist_cidrs: Vec<IpNetwork> = Vec::new();
    let file_blacklist_cidrs= get_cidrs(options.blacklist_file);
    let cs_blacklist_cidrs= get_cidrs_from_cs(options.xclude);
    if let Some(mut v)=file_blacklist_cidrs{
        blacklist_cidrs.append(&mut v);
    }
    if let Some(mut v)=cs_blacklist_cidrs{
        blacklist_cidrs.append(&mut v);
    }

    // perform whitelisting
    if !whitelist_cidrs.is_empty(){
        net_ips = filter_vec(&mut net_ips,whitelist_cidrs,true).expect("error performing whitelisting");
    }
    // perform blacklisting
    if !blacklist_cidrs.is_empty(){ 
        net_ips = filter_vec(&mut net_ips,blacklist_cidrs,false).expect("error performing blacklisting");
    }
 
    //dedup
    let final_result: Vec<_> = match options.keep_duplicates {
        false=> net_ips.into_iter().unique().collect(),
        true=> net_ips,
    };

    // final return to stdout
    final_result.iter().for_each(|ip|println!("{}",ip));
    Ok(())
}

fn get_cidrs_from_cs(cidrs_comma: Option<String>)->Option<Vec<IpNetwork>>{
    if let Some(cidrs_comma) = cidrs_comma {
        let cidr_vec: Vec<IpNetwork> = cidrs_comma.split(',').filter_map(|token|{
                token.parse::<IpNetwork>().ok()
            }
        ).collect();
        return Some(cidr_vec);
    }
    None
}

fn get_cidrs(cidr_filename: Option<String>)->Option<Vec<IpNetwork>>{
    if let Some(cidr_filename) = cidr_filename {
        let mut file_contents = String::new();
        File::open(cidr_filename).expect("failed to CIDR file")
            .read_to_string(&mut file_contents).expect("could not read from file");
        let cidr_vec: Vec<IpNetwork> = file_contents.split_whitespace().filter_map(|token|{
                token.parse::<IpNetwork>().ok()
            }
        ).collect();
        return Some(cidr_vec);
    }
    None
}

fn filter_vec(in_vec: &mut Vec<IpAddr>, cidrs: Vec<IpNetwork>, whitelist: bool) -> Result<Vec<IpAddr>,Box<dyn Error>>{
    let mut remove_these: Vec<_> = Vec::new();
    for ip in &mut in_vec.iter(){
        let mut allowed = false;
        if !whitelist {
            allowed=true
        }
        for net in cidrs.iter() {
            if net.contains(*ip) { // found ip in provided cidr
                if whitelist { 
                    allowed=true;
                } else { // blacklisting mode
                    allowed=false;
                }
            } 
        }
        if !allowed{
            remove_these.push(ip);
        }
    }
    // perform the actual removals
    
    let return_vec: Vec<_> = in_vec.iter().filter_map(|ip|{
        if !remove_these.contains(&ip) {Some(*ip)}else{None}
    }).collect();
    Ok(return_vec)
}
