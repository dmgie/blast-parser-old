# What is it

This is a relatively simple BLAST output parser, mainly to alleviate having to write a bash script to parse it. It also allows us to create various statistics about the BLAST file, i.e filtering duplicate queries (names), counting the number of significant alignments for each query, highest scores etc.

This helps to process some information that is needed (or rather is nice to have) for `dmgie/IntergeneIdentifier`. 

# Running the program
The program takes a single file as input and can either be run via

``` shell
cargo run <file here>
```

OR

``` shell
./blast-output-parser <file here>
```


# Improvements
This could be improved to make plots or various other things that have not been planned yet.

# TODO

Currently, it can either pretty print or print it out csv-like if you change the source code (the `i.print()` line near the end of the `main` function). Could implemented a CLI switch that either makes it csv-like of pretty prints it.

The advantages of CSV is to be able to parse it as a person might want in another program
