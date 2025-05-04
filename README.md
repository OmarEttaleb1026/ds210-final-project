# DS210 Final Project – Transit Centrality and Economic Opportunity in Philadelphia

## Usage Instructions

Go to the root directory and execute:
cargo build
cargo run

User input or command-line arguments are not necessary. The output directory will contain the output files. Usually taking two to three minutes, the runtime is mostly devoted to centrality calculations and census tract assignment to stops.

Run tests with:
cargo test

## Project Overview
This project looks into how Philadelphia's economic opportunity and public transportation accessibility relate to one another. We use transit centrality as a proxy for access and compare it to income data from the American Community Survey to answer the main research question, which is: How does access to central transit stops relate to household income across different census tracts? The main source of data is SEPTA's GTFS, which contains trip, route, and stop details for public buses. Additional information comes from the OpenDataPhilly census tract centroids for 2010 and the 2021 ACS data (table B19013), which shows the median household income by tract. Due to size restrictions, these datasets were downloaded and used locally.
## Data Processing
BufReader was used to import the data into Rust, where it was parsed line by line. Using the trip order as a guide, stop coordinates and directional connections were extracted from the GTFS files. A CSV file containing the names, IDs, and latitude and longitude centroids of each census tract was used to load the data. ACS income values were downloaded for economic data, and the tract ID was used as a key to merge them. The ACS data needed simple cleaning to get rid of string prefixes in the GEO_ID column, while the GTFS data needed directional edges from the stop_times and trips files assembled. Following processing, a Euclidean distance calculation was used to match each census tract to the stop that was closest to it geographically.
## Code Structure
The project is broken up into four major sections. The main.rs file prints the finished product and manages the program's workflow. The parser.rs module manages the loading of all data from CSV and GTFS sources. The TransitGraph structure is defined by the graph.rs module, which also contains methods for graph construction and centrality calculation. The analysis.rs module is in charge of assigning tracts to stops and exporting results. Nodes in the graph are represented by a custom Stop struct in the code, and all parsed GTFS input is stored in GTFSData. A crucial function called compute_closeness_centrality computes closeness scores by conducting a breadth-first search from every node. Cluster_neighborhoods_to_csv, another essential function, associates each tract with the nearest stop. Each module is in charge of a precisely defined task in the data pipeline, and the code organization guarantees that concerns are kept apart.
## Main Workflow
Using the parser module, the program loads stops and trip information from GTFS files at the start of the program. The graph module is then used to create a graph of stop-to-stop connections. The analysis module assigns each census tract to the closest stop after calculating the closeness centrality for each stop. These outcomes are stored in CSV output files. Last but not least, the main.rs reads and prints summaries that include the number of tracts per stop, the top five most central stops, and an overview of the tract-income-stop assignments. 
## Tests
Every significant component has its own set of unit tests. Testing GTFS loading, graph construction, census loading, and centrality calculation are a few of these. The results of the cargo test verify that each of the four tests was completed successfully. By ensuring that stops are parsed, the graph is populated, and the census data loads properly, each test confirms that the corresponding module functions as intended. Before analysis begins, these tests make sure the project is solid and that important features are operational. 
## Results
There are three CSV files and multiple terminal summaries in the final output. The terminal displays the names and scores of the five closest stops ranked by closeness score. Additionally, it prints how many census tracts are allocated to each stop. The combined tract-opportunity results are displayed last, along with the tract ID, name, assigned stop, and median income. A variety of tracts with low and middle incomes connected to central stops are depicted in the results. This implies that although a wide range of people are served by central stops, there isn't always a clear link between centrality and economic opportunity, at least in the Eastern PA/Philadelphia area.
## Usage Instructions
Go to the root directory and execute cargo build and cargo run to build and launch the project. User input or command-line arguments are not necessary. The output directory will contain the output files. Usually taking two to three minutes, the runtime is mostly devoted to centrality calculations and census tract assignment to stops. 
## AI Assistance and Citations
To ensure that I checked all the boxes for this written section, ChatGPT helped format code comments for my code. With a thorough comprehension of their reasoning and intent, all code and recommendations were examined and put into practice. Other data sources include ACS table B19013 from data and SEPTA GTFS data from OpenDataPhilly. OpenDataPhilly provides tract centroid data and census.gov data. Using these, the transit network was constructed, tracts were assigned, and transit access was linked to income levels.
