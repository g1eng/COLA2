# About

COLA2 is a simple solver program for Collatz conjecture (known as `3n+1` problem).

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
for i in {1..32}; do
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
511 61
1023 62
2047 156
4095 157
8191 158
16383 159
32767 129
65535 130
131071 224
262143 225
524287 177
1048575 178
2097151 303
4194303 304
8388607 473
16777215 474
33554431 444
67108863 445
134217727 384
268435455 385
536870911 448
1073741823 449
2147483647 450
4294967295 451
```

## 3. interactive

`cola2 <enter>`


# About `Collatz conjecture`

See [wikipedia](https://en.wikipedia.org/wiki/3n%2B1).
In this program, I call the map `n|->n/2` as A and map `n|->3n+1` as B.


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

