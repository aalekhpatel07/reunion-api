# A Disjoint-Set data structure (aka Union-Find w/ Rank)

## What is Union-Find?

Suppose you have a collection `S` of elements `e1`, `e2`, `...`, `en`, and wish to group them into different collections using operations:

- "put `ei` and `ej` into the same group" (union),
- "give me a representative of the group `ei` belongs to" (find).

Then a Union-Find data structure helps to store the underlying groups very efficiently and implements this API.

**Note**: The variant implemented uses Path Compression to further improve the performance.

## (Some) Applications

- **Detect Cycles in Graph**: Given a graph `G`, we can put the endpoints of edges into the same group (same connected component) unless there is a pair of endpoints `(ei, ej)` that share a group representative. If that happens, there was already a path existing between them, and adding this edge will add multiple paths, which cannot be the case for acyclic graphs.

- **Number of connected components in Graph**: Given a graph `G`, put the endpoints of edges into the same group (same connected component). Once all nodes are exhausted, the number of groups formed is the number of connected components in `G`.

Some [interesting lecture notes](https://www.cs.cmu.edu/~avrim/451f13/lectures/lect0912.pdf) regarding Union-Find.

## Usage

### Setup

Using any of the package installers, install `pyreunion` from the PyPi.

For example, `pip install pyreunion`.

### API

#### Example 1

*Task*: Create a UnionFind data structure of arbitrary size that contains `usize` at its elements.
Then, union a few elements and capture the state of the data structure after that.

*Solution*: 

```python
import pyreunion


def main():

    # Create an empty UnionFind data structure.
    uf = pyreunion.UnionFind()

    print("Initial state:", uf.str())
    print("All elements form their own group (singletons).")
    print(uf.subsets())

    uf.union(2, 1)
    print("After combining the groups that contains 2 and 1:", uf.str())

    uf.union(4, 3)
    print("After combining the groups that contains 4 and 3:", uf.str())

    uf.union(6, 5)
    print("After combining the groups that contains 6 and 5:", uf.str())

    hs1 = {1, 2}
    hs2 = {3, 4}
    hs3 = {5, 6}

    subsets = uf.subsets()
    assert (len(subsets) == 3)

    assert (hs1 in subsets)
    assert (hs2 in subsets)
    assert (hs3 in subsets)

    uf.union(1, 5)

    print("After combining the groups that contains 1 and 5", uf.str())

    subsets = uf.subsets()
    assert (len(subsets) == 2)

    for x in hs1:
        hs3.add(x)

    assert (hs3 in subsets)
    assert (hs2 in subsets)

    # It is possible to iterate over the subsets.
    for partition in uf.subsets():
        print(partition)


if __name__ == '__main__':
    main()

```

## Performance

The underlying implementation uses Path Compression and is written in Rust.
The implementation and some performance statistics are available [here](https://www.github.com/aalekhpatel07/reunion).

