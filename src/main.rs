// Second attempt

use std::{
    env::args,
    error,
    fs::File,
    io::{BufReader, Read},
};

// TODO: Either have SigAl have a query info section (i.e where it came from)
//       Or have each query have a SigAl section (i.e. list of its hits)

#[derive(Debug, Clone)]
struct Query {
    name: String,
    length: i64,
    // hits: Vec<SigAl>,
}

#[derive(Debug, Clone)]
struct SigAl {
    info: String,
    score: f64,
    e_value: f64,
    length: i64,
    origin: Query,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let filename = args().nth(1).expect("No filename given");
    let file = File::open(filename).expect("Could not open file");
    let mut reader = BufReader::new(file);
    let mut content = String::from("");
    reader.read_to_string(&mut content)?;

    let queries = &get_queries(content)[1..];

    let mut processed: Vec<SigAl> = Vec::new();
    for i in queries {
        for a in process(i.clone()) {
            processed.push(a)
        }
    }

    // sort processed by score
    processed.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // Only keep 2 from each query
    let filtered: Vec<SigAl> = keep_top(processed, 1);

    for i in filtered {
        println!("{:?}", i.info);
    }

    // process(queries[0].clone());

    Ok(())
}

/// How many significant alignments from each query do we keep
fn keep_top(to_filter: Vec<SigAl>, keep_num: i32) -> Vec<SigAl> {
    let mut filtered: Vec<SigAl> = Vec::new();
    for i in to_filter {
        if filtered
            .iter()
            .filter(|a| a.origin.name == i.origin.name)
            .count()
            < keep_num as usize
        {
            filtered.push(i)
        }
    }
    filtered
}

fn get_queries(content: String) -> Vec<String> {
    // Split by Query=, and return them as a vector
    let split_by_query: Vec<String> = content
        .split("Query= ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    split_by_query
}

// TODO: Make function more modular
fn process(query: String) -> Vec<SigAl> {
    let mut seq_al_processed: Vec<SigAl> = Vec::new();

    // 2 Step process, split by >
    // Get the first part, and extract query information from it
    //
    // Get the second part by looping through the lines of those things
    let alignments: Vec<String> = query.split('>').map(|x| x.to_string()).collect();

    // Getting the Query Header and length info
    let mut query_info = Query {
        name: "".to_string(),
        length: 0,
    };
    // Header, since that is the first element when splitting by '>'
    query_info.name = alignments[0].split_whitespace().collect::<Vec<&str>>()[0].to_string();
    // Length
    for i in alignments[0].lines() {
        if i.starts_with("Length=") {
            query_info.length = i.split('=').collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
        }
    }

    ///////////////////////////
    ///////////////////////////
    ///////////////////////////

    // Process all significant alignments within one query

    let signif_iter = alignments[1..].iter();
    for full_content in signif_iter {
        let mut sig_struct = SigAl {
            info: "".to_string(),
            score: 0.0,
            e_value: 0.0,
            length: 0,
            origin: query_info.clone(), // NOTE: Keep query info here, or keep SigAl in query struct?
        };

        // Info / header
        sig_struct.info = full_content
            .lines()
            .take_while(|x| !x.starts_with("Length"))
            .map(|x| x.trim())
            .collect::<String>();

        // Length
        sig_struct.length = full_content
            .lines()
            .find(|x| x.starts_with("Length"))
            .unwrap()
            .split('=')
            .collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap();

        // Score and E-value
        full_content
            .lines()
            .find(|x| x.trim().starts_with("Score"))
            .unwrap()
            .split(',')
            .for_each(|x| {
                let s = x.trim();
                if s.starts_with("Score") {
                    sig_struct.score = x.split_whitespace().collect::<Vec<&str>>()[2]
                        .trim()
                        .parse::<f64>()
                        .unwrap();
                } else if s.starts_with("Expect") {
                    sig_struct.e_value = x.split_whitespace().collect::<Vec<&str>>()[2]
                        .trim()
                        .parse::<f64>()
                        .unwrap();
                }
            });
        // println!("{:?}", &sig_struct);
        seq_al_processed.push(sig_struct);
    }

    seq_al_processed
}
