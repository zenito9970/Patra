Patra
===

Patra, Parallel Auto Test Runner type A.  
For competitive programming contest, or Marathon Match.

Installation
---

```
$ git clone https://github.com/zenito9970/Patra.git
$ cd Patra
$ cargo install --path .
```

Usage
---

```
$ patra ./solver input-dir output-dir
```

#### Options

| Option | Description | Default value |
|--------|-------------|---------------|
| --threads | Specify the number of threads to execute in parallel. | 10 |
| --logfile | Specify the file to save the log. | log.ltsv |

Examples
---

```
$ tree
.
├── input
│   ├── input_1.txt
│   ├── input_2.txt
│   └── input_3.txt
├── output
└── solver.cpp

2 directories, 4 files
$ g++ -std=c++14 -o solver solver.cpp
$ patra ./solver input output
[Running] case 2
[Running] case 3
[Running] case 1
[stderr: case 2] score: 300
[Finish] case 2
[stderr: case 3] score: 311
[Finish] case 3
[stderr: case 1] score: 303
[Finish] case 1
$ tree
.
├── input
│   ├── input_1.txt
│   ├── input_2.txt
│   └── input_3.txt
├── log.ltsv
├── output
│   ├── input_1.txt.out
│   ├── input_2.txt.out
│   └── input_3.txt.out
├── solver
└── solver.cpp

2 directories, 9 files
```




