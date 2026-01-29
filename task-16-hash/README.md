# DD1338 Week 16

## Database Management Systems (and lots of hash!)

This week we are doing hashing!

We have already fought through dtek, progp and logiken, let's sprinkle on some dbas; one more profession paradigm for y'all.

### Prepare for your assignment

1) Create a repository named `<KTH_ID>-hash`.
2) Clone your newly created repository and start coding.

## Assignment

Don't worry! We wont be cooking a full relational database... _yet_ (greetings from the ghost of future Dena), but rather learning the fundamentals behind them!

This week you'll be implementing a VERY simple database management system, which includes
- CLI interaction for entering, exiting and querying,
  - See [the rust book](https://rust-cli.github.io/book/tutorial/index.html) for how to write a CLI application.
- Lots of hash maps, and maybe even trees, and file management.

This can be done in any language. However, you must use your own implementation of hash maps, not from a library! The hashier, the better.

### DBMS

***Required***

The minimal feature requirements for your database system are:
- insertion
- deletion
- select all (list all)
- projection by id (get value of key)

The database can have one of the following formats:
- Structured data, aka. table structure. SQL databases follow this format. 
  - You don't have to interpret SQL querying. It's okay to be lazy and come up with something much simpler.
- Semi-structured data, aka. tree/graph structure. JSON and XML follow this format. 
  - Querying formats of semi-structured data varies a lot. Here you can be creative. XPath (XML querying) use paths with conditions to query data; for example: `my/data/with[title="hash"]/lots/*`. Once again, you only have to implement very simple querying (literally only a couple of query requests).

The minimal requirements doesn't include the features of creating and deleting tables/pools. Therefore, a possible MVP will only have one giant table, simply a huge key-value store. Multiple instances of this store can be configured at startup by entering configuration parameters, but this is not a requirement. 

Every modification of the database has to be saved long term. This is usually done by a file manager concurrently so that other parts of the database, like the query manager and query executor can run uninterrupted. You may multithread your application, but this is far from necessary at this scale! You have to simply maintain a CSV file (or similar) of your data store. This file can in term be read to restore the database at startup or application crash. 

**_(optional fun)_**:
- Add additional data-definition features, like create a table/pool.
- Add additional querying features, like projection by condition.
- A database usually maintain a log which allows for backtracking in case of a bad query. Add a log and regret feature.
- Transactions are groups of database queries. Transactions must me performed in isolation in relation to each other. A transaction manager can identify and handle conflicts between transactions, thereby allowing multiple sets of queries to run concurrently on the database. Implement so that single queries are run as transactions and that multiple queries can be grouped as transactions. 

### Hash map requirements

The hash map requirements are that your implementation has to be capable, in addition to serve as the buffer for your database, to dynamically resize its base arrays when necessary. See regular task for basic guidance. Also, the one and only fabled Ric's lecture is recommended to, at the very least, scroll through. However, notice that dynamic memory allocation is not a requirement of the regular task. 

You are also required to implement your own hashing function!
