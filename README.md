# About

COLA2 is a simple solver program for Collatz conjecture (known as `3n+1` problem).
About Collatz conjencture, see [wikipedia](https://en.wikipedia.org/wiki/3n%2B1). 

In this program, map `n|->n/2` is called as A, and map `n|->3n+1` is called as B.

# Requirement

* rustc (>=1.53.0 recommended)

# Installation

For Unix-like operating system, simply make it:

```
make
make install
```

# Usage

## 1. give numbers via pipeline

```
echo 153 | cola2
153 36
echo 155555555555555555555555555553 | cola2
155555555555555555555555555553 901
```

## 2. give numbers from a file

```
for i in {1..8}; do
  echo 2^$i-1 | bc
done > num.dat
cola2 < num.dat
1 0
3 7
7 16
15 17
31 106
63 107
127 46
255 47
```

## 3. interactive

`cola2 <enter>`

# Tips: Make dot file to visualize convergence process

Following sample script generates a DOT file to visualize Collatz map for numbers less than 257:

```
{
  echo strict digraph map \{
  for i in {1..256}
  do
    echo $i | cola2 -d
  done
  echo \}
} > collatz_map.dot
```

![Collatz map 256](docs/sample.png)

# Author

Nomura Suzume <suzume315[ATT]g00.g1e.org>
