use std::{
    collections::HashSet,
    env::args,
    error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

// TODO: Fix it so it shows the sequence with the highest score
// TODO: Get a certain number of top scoring i.e I want the 5 highest scoring ones
//       Could just sort and then index

#[derive(Clone)]
struct QueryStats {
    query: String,
    highest_score: f64,
    lowest_score: f64,
    average_score: f64,
    highest_e_value: f64,
    lowest_e_value: f64,
    average_e_value: f64,
    num_signif: i64,
    sigalign: Vec<SigAlign>,
}

impl QueryStats {
    fn new() -> QueryStats {
        QueryStats {
            query: String::new(),
            highest_score: 0.0,
            lowest_score: 0.0,
            average_score: 0.0,
            highest_e_value: 0.0,
            lowest_e_value: 0.0,
            average_e_value: 0.0,
            num_signif: 0,
            sigalign: Vec::new(),
        }
    }
}
impl Display for QueryStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Query: {}", self.query)?;
        writeln!(f, "Highest score: {}", self.highest_score)?;
        writeln!(f, "Lowest score: {}", self.lowest_score)?;
        writeln!(f, "Average score: {}", self.average_score)?;
        writeln!(f, "Highest E-value: {}", self.highest_e_value)?;
        writeln!(f, "Lowest E-value: {}", self.lowest_e_value)?;
        writeln!(f, "Average E-value: {}", self.average_e_value)?;
        writeln!(f, "Number of significant alignments: {}", self.num_signif)?;
        writeln!(f, "--------------------------------")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Query {
    name: String,
    has_signif: bool,
    num_hits: u64,
    sig_aligns: Vec<SigAlign>,
}

impl Query {
    fn new() -> Self {
        Self {
            name: String::from(""),
            has_signif: false,
            num_hits: 0,
            sig_aligns: Vec::new(),
        }
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }

    fn change_signif(&mut self, has_signif: bool, num_hits: u64, sig_aligns: Vec<SigAlign>) {
        self.has_signif = has_signif;
        self.num_hits = num_hits;
        self.sig_aligns = sig_aligns;
    }

    fn print(&self) {
        println!("Query: {} with {} alignments", self.name, self.num_hits);
        println!("Number of significant alignments: {}", self.num_hits);
        println!("Significant alignments:");
        for align in &self.sig_aligns {
            align.print();
        }
    }
}
impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Query: {}, has {}", self.name, self.num_hits)
    }
}

#[derive(Debug, Clone)]
struct SigAlign {
    id: String,
    typ: String,
    num_species: i64,
    score: f64,
    e_value: f64,
}

impl SigAlign {
    fn print(&self) {
        println!("\tID: {},", self.id);
        println!("\tType: {},", self.typ);
        println!("\tNumber of species: {},", self.num_species);
        println!("\tScore: {},", self.score);
        println!("\tE-value: {}; ", self.e_value);
        println!("\t---------");
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // Read in file using BufReader
    signif_parse()?;
    Ok(())
}

fn signif_parse() -> Result<(), Box<dyn error::Error>> {
    // Only read significant hits

    let filename = args().nth(1).expect("No filename given");
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    // Collect lines ignoring empty ones
    let mut lines: Vec<_> = reader
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .collect();

    // Collect all query's inside
    let mut query = Query::new();
    let mut all_queries: Vec<Query> = Vec::new();

    let mut line_iter = lines.iter_mut(); // Iterator
    'full: loop {
        // Break if end of file (None)
        let cur_line = match line_iter.next() {
            Some(x) => x,
            None => break 'full,
        };

        // Collect query names
        if cur_line.starts_with("Query= ") {
            query.change_name(cur_line.split_whitespace().nth(1).unwrap().to_string());
        }

        // If there are significants, collect them
        if cur_line.starts_with("Sequences producing significant") {
            // Collect inside new vec
            let mut signifs: Vec<SigAlign> = Vec::new();

            // Collect lines that are significant
            'signif: loop {
                let sig_line = line_iter.next().unwrap();
                if sig_line.starts_with('>') {
                    break 'signif;
                }
                // Go over the significant line, and extract the values
                let s = sig_line.split_whitespace();
                let mut s_iter = s.clone();
                let id = s_iter.next().unwrap().to_string();
                let typ = s_iter.next().unwrap().to_string();
                let num_species = s_iter.nth(1).unwrap().parse::<i64>().unwrap();
                let score = s_iter.nth(1).unwrap().parse::<f64>().unwrap();
                let e_value = s_iter.next().unwrap().parse::<f64>().unwrap();
                let sig_align = SigAlign {
                    id,
                    typ,
                    num_species,
                    score,
                    e_value,
                };
                signifs.push(sig_align);
                // As soon as we reach the end (i.e the next line starts with '>' we break out)
            }
            query.change_signif(true, signifs.len() as u64, signifs);
            all_queries.push(query.clone());
        }
    }

    ////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////

    // TODO: Refactor statistics to be a function of Query
    let stats = get_statistics(all_queries.clone());
    let stats_unique = get_statistics(get_uniques(all_queries));

    // Get query with highest score
    // get_highest_scoring_queries(&stats);
    // get_highest_scoring_queries(&stats_unique);

    // sorted_queries(stats);
    // sorted_queries(stats_unique, "evalue");
    //
    let a = get_top_queries(&stats_unique, "score");
    get_align_from_QS(a);

    Ok(())
}

/// From a list of all hits which can contain many queries with the same name (due to the layout)
/// of the experiment, filter them to only contain query name once
fn get_uniques(all_queries: Vec<Query>) -> Vec<Query> {
    let mut unique_hits: Vec<Query> = Vec::new();
    let mut unique_names: HashSet<String> = HashSet::new();
    for hit in all_queries {
        if !unique_names.contains(&hit.name) {
            unique_names.insert(hit.name.clone());
            unique_hits.push(hit);
        }
    }
    unique_hits
}

/// Get statistics for each query
/// Given a vector of queries find (per query):
/// 1. Highest score
/// 2. Lowest score
/// 3. Average score
/// 4. Highest E-value
/// 5. Lowest E-value
/// 6. Average E-value
fn get_statistics(all_queries: Vec<Query>) -> Vec<QueryStats> {
    let mut query_stats: Vec<QueryStats> = Vec::new();
    // This is per query
    for queries in all_queries {
        let mut highest_score = 0.0;
        let mut lowest_score = 0.0;
        let mut average_score = 0.0;
        let mut highest_e_value = 0.0;
        let mut lowest_e_value = f64::MAX;
        let mut average_e_value = 0.0;
        let mut num_signif = 0;
        for align in &queries.sig_aligns {
            if align.score > highest_score {
                highest_score = align.score;
            }
            if align.score < lowest_score {
                lowest_score = align.score;
            }
            if align.e_value > highest_e_value {
                highest_e_value = align.e_value;
            }
            if align.e_value < lowest_e_value {
                lowest_e_value = align.e_value;
            }
            average_score += align.score;
            average_e_value += align.e_value;
            num_signif += 1;
        }
        average_score /= num_signif as f64;
        average_e_value /= num_signif as f64;
        query_stats.push(QueryStats {
            query: queries.name,
            highest_score,
            lowest_score,
            average_score: average_score / num_signif as f64,
            highest_e_value,
            lowest_e_value,
            average_e_value: average_e_value / num_signif as f64,
            num_signif,
            sigalign: queries.sig_aligns,
        });
    }
    query_stats
}

fn sorted_queries(query_stats: Vec<QueryStats>, score_or_evalue: &str) -> Vec<QueryStats> {
    // Sort by highest score or e_value
    let mut sorted_queries: Vec<QueryStats> = query_stats;
    match score_or_evalue {
        "score" => {
            sorted_queries.sort_by(|a, b| a.highest_score.partial_cmp(&b.highest_score).unwrap());
        }
        "evalue" => {
            sorted_queries.sort_by(|a, b| a.lowest_e_value.partial_cmp(&b.lowest_e_value).unwrap());
        }
        _ => println!("Please enter either 'score' or 'evalue'"),
    }

    // Print it in sorted manner
    for i in &sorted_queries {
        println!(
            "Query: {}, Highest Score (from all matches): {}, Lowest E-val (from all matches): {:?}",
            i.query, i.highest_score, i.lowest_e_value
        );
    }
    sorted_queries
}

/// This prints the alignments that have that specific score or evalue, given a vector of QueryStats
/// This means that it checks each alignment for the 'highest score' or 'lowest e value'
/// and if there is a match, it prints them to stdout.
fn get_align_from_QS(query_stats: Vec<QueryStats>) {
    for i in &query_stats {
        for j in &i.sigalign {
            if j.score == i.highest_score || j.e_value == i.lowest_e_value {
                println!(
                    "For query: {}; SeqID: {}, Type: {}, Score: {}, E-Value: {}",
                    i.query, j.id, j.typ, j.score, j.e_value
                );
            }
        }
    }
}

/// Gets the top-scoring queryStats object (or multiple if they share the same score/e-value)
fn get_top_queries(query_stats: &[QueryStats], score_or_evalue: &str) -> Vec<QueryStats> {
    let mut highest_score = 0.0;
    let mut lowest_e_val = f64::MAX;
    let mut highest_score_queries: Vec<QueryStats> = Vec::new();

    match score_or_evalue {
        "score" => {
            for i in query_stats {
                if i.highest_score > highest_score {
                    highest_score = i.highest_score;
                }
            }
            for i in query_stats {
                if i.highest_score == highest_score {
                    highest_score_queries.push(i.clone());
                }
            }
        }
        "evalue" => {
            for i in query_stats {
                if i.lowest_e_value < lowest_e_val {
                    lowest_e_val = i.lowest_e_value;
                }
            }
            for i in query_stats {
                if i.lowest_e_value == lowest_e_val {
                    highest_score_queries.push(i.clone());
                }
            }
        }
        _ => println!("Please enter either 'score' or 'evalue'"),
    }

    highest_score_queries
}
