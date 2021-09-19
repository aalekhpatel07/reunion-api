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

Install from npm using `npm install reunionjs`.

### API

#### Example 1

*Task*: Create a UnionFind data structure of arbitrary size that contains `usize` at its elements.
Then, union a few elements and capture the state of the data structure after that.

*Solution*: 

```js
import { UnionFind } from 'reunionjs';
import assert from 'assert/strict';


function exploreUnionFind() {
    // Create an empty UnionFind data structure.
    let uf = new UnionFind();

    console.log("Initial state:", uf.str());
    console.log("All elements form their own group (singletons).");
    console.log(uf.subsets());

    uf.union(BigInt(2), BigInt(1));
    console.log("After combining the groups that contains 2 and 1:", uf.str());

    uf.union(BigInt(4), BigInt(3));
    console.log("After combining the groups that contains 4 and 3:", uf.str());

    uf.union(BigInt(6), BigInt(5));
    console.log("After combining the groups that contains 6 and 5:", uf.str());

    let hs1 = new Set([BigInt(1), BigInt(2)]);
    let hs2 = new Set([BigInt(3), BigInt(4)]);
    let hs3 = new Set([BigInt(5), BigInt(6)]);

    let subsets = uf.subsets();
    console.assert(subsets.length == 3);
    
    console.assert(subsets.includes(hs1));
    console.assert(subsets.includes(hs2));
    console.assert(subsets.includes(hs3));

    uf.union(BigInt(1), BigInt(5));
    console.log("After combining the groups that contains 1 and 5", uf.str());

    subsets = uf.subsets();
    console.assert(subsets.length == 2);

    for (const elem of hs1) {
	hs3.add(elem);
    }

    console.assert(subsets.includes(hs3));
    console.assert(subsets.includes(hs2));


    // It is possible to iterate over the subsets.

    for partition in uf.subsets():
        console.log(partition);

}

```

## Performance

The underlying implementation uses Path Compression and is written in Rust.
The implementation and some performance statistics are available [here](https://www.github.com/aalekhpatel07/reunion).


