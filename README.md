# DS210_Project
Final project for DS210 at BU in Fall 2023.

Quick disclaimer: I have not reviewed the thousands of YouTube videos that were found via my webscraping and cannot ensure their appropriate content or subject-matter. The very first video that I began at was a video on Stoicism that was featured on the front page of the website in an incognito tab. I chose this video because I hope to use this data and more detailed graph analysis in a later research project on political pipelines in social media, and qualitative studies suggest that "self-help"-branded videos are an enticing starting point for said pipelines.

## list_of_edges.csv
This file is my list of edges. There are two columns and each element is a YouTube link. The first element is the starting node and the second element is the directed ending node. I web-scraped this myself using Selenium and a recursive function that starts at one video, scrapes the recommended videos' links, randomly moves to one, and recurses. Due to the way it was gathered, the **very first node should be connected to every other node**. This will be relevant in one of my tests later.

## read.rs
This is one of the modules I made with a function "read_file" that will take a file path and output a tuple in which the first element is a Vector of tuples (starting node, ending node) and the second element is simply the number of unique nodes. I am formatting it this way to match the input format of the Lecture 28 Breadth-First-Search (BFS) code that I adapted later. Something important to note here is that, because my csv file is links and, thus, strings, I had to convert each link to a unique integer. I used a HashMap to record a link with its corresponding unique integer and a HashSet to keep track of the links the program has already seen (unique links).

## graph_from_read.rs
This is another module, though it's quite short. It takes the output of read_file from read.rs and turns it into the Graph struct defined in main.rs. It returns that Graph.

## single_node.rs
This module contains the most fundamental function of the project, "single_node_distances." This function takes in a specific starting node and a Graph and outputs a tuple that contains the number of outgoing edges from the starting node, the maximum distance of those outgoing edges, the sum of all distances from the starting node, and the average distance as defined by the total distance divided by the number of outgoing edges. Most of this code was borrorwed from Lecture 28, though I had to create an if statement to catch "None" distance values which are quite common with my data. Another import thing to note is that the number of outgoing edges has 1 taken away from it after the loop because it counts the 0-distant node (itself) as an outgoing edge which is not what we want for our calculations.

I have another function, "single_node_analytics" in this module that simply prints out the aforementioned statistics in a nice formatted way. I split it up this way so that I do not need to rewrite the BFS algorithm in the next module, all_node.rs.

## all_node.rs
This is the final non-main module it essentially creates a nested for loop utilizing the aforementioned BFS function in single_node.rs. There's only one function in this module called "all_node_analytics." It iterates through each node in the graph and gathers the average distance for each, then computes the "average of averages" by summing those averages and dividing it by all of the nodes that have an average of greater than 0.0. It then prints out that average of averages as well as the number of nodes with and without connections.

## main.rs
First, I will explain the types, struct, and impl of that struct. All of them come from Lecture 28 but I removed the unnecessary indirect graph parts because my data is directed. Basically, we're just creating a data structure called Graph which has built in functions necessary for doing the BFS algorithm, such as adding edges. In the actual main function, we just use the functions from our other modules that we called in lines 1-4 and then create a variable called graph from the csv file inside of the src folder. Then, we run three calls of single_node_analytics to show how those outputs might differ and one call of all_node_analytics.

## Output Review and Explanation
<img width="643" alt="image" src="https://github.com/hw03hw03/DS210_Project/assets/90813540/fe6794da-80ad-4ca7-a9d0-279c9a92997c">

The above can be run simply by using "cargo run". If desired, you can change the starting node for the "single_node_analytics" function calls to see different outputs.

When we start at node 0, we can see that there are 1878 outgoing connections which is to be expected because node 0 is the starting YouTube video from which every other video (node) was walked to. Its maximum distance is 58 and its average distance is roughly 25. We can interpret these results and intuit that many of the recommended videos of certain nodes were revisited in the web-scraping process, thus creating more dense clusters than I had originally expected. The maximum possible value for maximum distance would be 150 because that is how many times I recursed in the gathering of my data.

When we start at node 1, there are 0 outgoing connections and thus 0 maximum distance and NaN average distance (as it is attempting to divide by 0). This is to be expected when investigating a node that has no edges.

When we start at node 8, there are fewer connections, a smaller maximum distance, and a smaller average distance when compared to the very first node -- also to be expected.

Finally, the average of average shortest distances is just under 8. What I find most interesting is that only 138 of the 1879 unique nodes have outgoing connections. 

## Tests
The first test relates to the aforementioned "grandparent" node. As stated before, the very first node should be connected to all of the other nodes. Thus, the "test_grandparent_node" function gathers the number of outgoing connections of the 0th node and panics if it's anything other than the length of the Graph minus 1. Here, we are subtracting one because we are not including the 0th node itself as a connection as per the way we set up our BFS function.

The second test ensures that the maximum number of connections from any node in the Graph is less than the length of the graph. This would panic if the graph has some sort of loop it's getting caught on.
