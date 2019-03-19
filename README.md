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
$ patra --help
patra 0.2.0
zenito9970 <zenito9970@gmail.com>
Patra, Parallel Auto Test Runner type A.
For competitive programming contest, or Marathon Match.

USAGE:
    patra [OPTIONS] [command]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --in-dir <dir>         directory containing test cases
        --logfile <file>       Specify the file to save the log. default=log.ltsv
    -o, --out-dir <dir>        output target directory
        --threads <threads>    Specify the number of threads to execute in parallel. default=10

ARGS:
    <command>    target command
```

### Config file

You can setting to `patra.toml` file.  
The setting value of Options is used with priority.

| Key | Type | Description |
|-----|------|-------------|
| command | String | Specify the command to be executed |
| input_dir | Path | Specify input directory (containing test cases) |
| output_dir | Path | Specify output directory |
| logfile | Path | Specify the file to save the log. |
| threads | Integer | Specify the number of threads to execute in parallel. |

Examples
---

#### Don't use `patra.toml`

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
$ patra -i input -o output ./solver
[INFO] Running: case 1
[INFO] Running: case 2
[INFO] Running: case 3
[DEBUG] stderr(case 2): score: 300
[INFO] Finish: case 2
[DEBUG] stderr(case 1): score: 303
[INFO] Finish: case 1
[DEBUG] stderr(case 3): score: 311
[INFO] Finish: case 3
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

#### Use `patra.toml`

```
$ tree
.
├── input
│   ├── input_1.txt
│   ├── input_2.txt
│   └── input_3.txt
├── output
├── patra.toml
└── solver.cpp

2 directories, 5 files
$ g++ -std=c++14 -o solver solver.cpp
$ cat patra.toml
input_dir = "input"
output_dir = "output"
$ patra ./solver
[INFO] Running: case 1
[INFO] Running: case 2
[INFO] Running: case 3
[DEBUG] stderr(case 2): score: 300
[INFO] Finish: case 2
[DEBUG] stderr(case 1): score: 303
[INFO] Finish: case 1
[DEBUG] stderr(case 3): score: 311
[INFO] Finish: case 3
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
├── patra.toml
├── solver
└── solver.cpp

2 directories, 10 files
```



