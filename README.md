# minigrepCmdTool

A command Line tool that searches for a given string in a specified path

It's built using Rust, and the project is part of Rust lang book (https://doc.rust-lang.org/book/title-page.html) 
 
#### How to use
On Command line:

CASE_INSENSITIVE=X cargo run Type Query Path <br>
X     : 0 = case sensitive, 1 = case insensitive <br>
Type  : file for file , dir for Directory <br>
Query : Required String <br>
Path  : Interested path <br>


To perform some tests:<br>
Command line : cargo test<br>
These are self written tests, can be found in a module name tests in src/lib.rs<br>

### Features
1. Directory search support : ADDED <br>
2. Highlighted output for query : TO-DO <br>
3. More information about seach results : TO-DO <br>
