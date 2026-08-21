# Data Structures — Deep Interview Guide (with diagrams, Rust context)

> Goal of this doc: explain each structure the way you'd explain it to a smart friend who's never heard the jargon — starting from "what annoying problem were people trying to solve," then a picture, then how it actually works, then what makes it tricky specifically in Rust (because Rust won't let you just wave a pointer around like C++ does).

### The one idea to hold onto for the whole document

In most languages, a tree node is just: "a box with some data and arrows to other boxes." Anyone can point at any box. In Rust, **every value has exactly one owner**, so "just point at it" isn't free — you have to pick a strategy:

```
Box<Node>            -> strict parent owns child, tree-shaped, no cycles
Rc<RefCell<Node>>     -> shared ownership, needed for parent-pointers / cycles
Vec<Node> + usize     -> "arena" style: nodes live in a flat vector, you pass
                         around indices instead of pointers. No borrow-checker
                         fights. This is what real production Rust code uses
                         for anything with cycles or backlinks.
```

Keep this table in your head — I'll say which option fits each structure.

---

# 1. Tree Data Structures

## 8. Binary Tree

**The problem it solves:** you often want to split a big problem into two smaller versions of itself (this half, that half) and repeat. A binary tree is just that idea made into a data structure — a box that points to at most two smaller boxes.

```
                (8)
               /    \
            (3)      (10)
           /   \         \
         (1)   (6)       (14)
```

There's no rule about *where* values go — it's just shape. The rules come from what you build on top of it (see BST below).

**Rust:**
```rust
enum Tree { Leaf, Node(Box<Tree>, i32, Box<Tree>) }
```
Because a child is only ever owned by its parent (nothing else points at it), `Box` — the simplest, cheapest ownership type — is all you need.

**What to say if asked to traverse it:** recursion is natural (pre/in/post-order), but mention that very deep trees can blow the call stack in Rust (no guaranteed tail-call optimization), so an iterative version using an explicit `Vec` as a manual stack is the "production" answer.

---

## 9. Binary Search Tree (BST)

**The problem it solves:** searching a sorted *array* is fast (binary search, O(log n)) but inserting into the middle of an array is slow (you have to shift everything). A BST gives you the speed of binary search *and* fast insertion, by using pointers instead of contiguous memory.

**The one rule:** for every node, everything in its left subtree is smaller, everything in its right subtree is bigger.

```
                (8)
               /    \
            (3)      (10)
           /   \         \
         (1)   (6)       (14)

Looking for 6? Compare to 8 (go left), compare to 3 (go right), found it.
Just like binary search — but no shifting needed to insert.
```

**The catch (this is the #1 follow-up question):** if you insert already-sorted data (1, 2, 3, 4, 5...), the tree degenerates into a straight line — basically a linked list — and search becomes O(n) instead of O(log n):

```
Inserting 1,2,3,4,5 in order gives you this ugly shape:

(1)
   \
   (2)
      \
      (3)
         \
         (4)
            \
            (5)
```

That's *exactly* why AVL and Red-Black trees exist — they add rules to force the tree to stay bushy.

**Rust:** `Box<Node>` is fine — insert only ever walks one path down, delete is the fiddly part (3 cases: node has 0, 1, or 2 children; for 2 children you swap the value with the in-order successor and delete that instead).

---

## 10. AVL Tree

**The problem it solves:** "how do we stop a BST from turning into a line?" AVL's answer: after every insert/delete, check if any subtree got more than 1 level taller than its sibling, and if so, *rotate* nodes to rebalance — like re-stacking boxes so the pile doesn't lean.

```
Unbalanced (right-heavy by 2):        After a "left rotation":

(1)                                        (2)
   \                                      /    \
   (2)              --------->        (1)      (3)
      \
      (3)
```

**The four rotation cases** (memorize the pattern, not the code): Left-Left, Right-Right (single rotation), Left-Right, Right-Left (double rotation — rotate the child first, then the node).

```rust
fn rotate_right(mut y: Box<Node>) -> Box<Node> {
    let mut x = y.left.take().unwrap(); // "take" = temporarily steal ownership
    y.left = x.right.take();            // so we can freely rearrange pointers
    update_height(&mut y);
    x.right = Some(y);
    update_height(&mut x);
    x
}
```
`.take()` is the Rust-specific trick worth calling out: mid-rotation you briefly need to hold a subtree that "belongs" to two places at once, and Rust won't allow that — so you swap it out for `None` temporarily, move it where it needs to go, then it has one owner again.

**AVL vs Red-Black, the one-liner:** AVL is stricter (max height difference of 1) → faster reads, more rotations on writes. Red-Black is looser → slightly slower reads, cheaper writes. Pick AVL for read-heavy workloads, Red-Black for write-heavy ones.

---

## 11. Red-Black Tree

**The problem it solves:** same as AVL (stop the BST from degenerating), but with less rebalancing work — instead of tracking exact heights, each node just gets a color (red or black), and 4 simple rules about coloring guarantee the tree can never be more than roughly 2x taller than the perfectly balanced version.

```
        (10:B)
       /       \
   (5:R)       (15:R)
   /   \            \
(3:B) (7:B)        (20:B)

Rule of thumb: no red node has a red child, and every root-to-leaf
path passes through the same number of black nodes.
```

**The real Rust question here isn't the coloring — it's "how do you even write this."** Fixing up a Red-Black tree after a delete requires looking at a node's **sibling and parent**, which means walking *upward*. A plain `Box<Node>` tree has no way to go up. So the honest answer is one of:
- `Rc<RefCell<Node>>` with a `Weak<RefCell<Node>>` for the parent pointer (`Weak` specifically, so parent and child don't keep each other alive forever in a reference cycle — this detail is a strong signal to mention).
- **Arena style:** a `Vec<Node>` where each node stores `parent: Option<usize>`, `left: Option<usize>`, `right: Option<usize>` — no `Rc`, no runtime borrow-checking, just plain indices. This is what real Rust codebases do.

**Fun fact worth dropping:** Rust's own `std::collections::BTreeMap` does *not* use a Red-Black tree internally — it uses a B-Tree (next section), because B-Trees are more CPU-cache-friendly. Interviewers love it when you know this.

---

## 12. Heap (Min-Heap / Max-Heap)

**The problem it solves:** "give me the smallest (or biggest) item, fast, over and over, while also adding new items." A sorted array gives O(1) peek but O(n) insert. A heap gives O(log n) for both, by being *only partially* ordered — just enough to always know where the min/max is.

**The trick: it's a tree, but stored as a plain array — no pointers at all.**

```
Tree view (min-heap):              Array view:
        (2)                        [2, 4, 3, 8, 5, 9]
       /    \                       0  1  2  3  4  5
    (4)      (3)
    /  \      /
  (8)  (5)  (9)

Rule: parent <= both children (min-heap) or parent >= both children (max-heap).
Index math replaces pointers:
  children of index i  ->  2i+1, 2i+2
  parent of index i    ->  (i-1)/2
```

Because the tree is always "complete" (filled left-to-right, no gaps), the array never has holes, so index arithmetic works perfectly — no `Box`, no ownership headache at all. This is the friendliest tree in Rust for exactly that reason.

**Rust:** `std::collections::BinaryHeap<T>` is a **max-heap** by default. For a min-heap, wrap values in `std::cmp::Reverse(x)`. This exact question ("how do you get a min-heap in Rust?") gets asked a lot — know the `Reverse` trick cold.

**Used for:** priority queues, heap sort, Dijkstra's algorithm, "top K" problems.

---

## 13. B-Tree

**The problem it solves:** BSTs are great in memory, but terrible on disk — every pointer-follow might mean a slow disk seek. A B-Tree fixes this by cramming *many* keys into each node (not just one), so each disk read pulls in a whole chunk of useful data, and the tree stays very shallow.

```
                [ 10 | 20 ]
               /     |     \
        [1|5]   [12|15|18]   [25|30]

Each node can hold multiple keys and has multiple children —
not just "left" and "right," but "between 10 and 20," etc.
```

**Why it's shallow:** if each node holds, say, 100 keys, a billion records only need a tree of height ~5. Compare that to a BST, which for a billion records could be 30 levels deep.

**Rust reality:** this is literally how `BTreeMap`/`BTreeSet` work under the hood — flat arrays of keys per node, chosen specifically because scanning a few keys sequentially inside one node is *cache-friendly* (the CPU pulls a whole cache line at once), while chasing pointers all over memory (like a classic BST/Red-Black tree does) causes lots of cache misses. This is the single best "Rust-specific" fact you can drop in an interview.

---

## 14. B+ Tree

**The problem it solves:** B-Trees are good, but if you often need *ranges* of data ("give me everything between March and June"), you want the leaves themselves linked together so you can just walk sideways once you find the start.

```
Internal nodes only route you to the right leaf (no data stored there).
Leaves hold the actual data AND are linked to each other:

        [ 20 | 40 ]
        /    |     \
   [leaf]-[leaf]-[leaf]      <- linked list across the bottom
```

**Contrast with B-Tree, the key exam answer:** in a plain B-Tree, data can live in internal nodes too, so you might find your answer early. In a B+Tree, you *always* walk to a leaf — slightly slower for single lookups, but much faster for range scans, because of that linked-leaf trick. This is why databases and filesystems use B+Trees for indexes.

---

## 15. Trie (Prefix Tree)

**The problem it solves:** autocomplete. If you store "cat," "car," "card" in a normal structure, checking "does any word start with 'ca'" means scanning everything. A trie stores strings *character by character* along shared paths, so common prefixes are stored once.

```
root
 └── c
      └── a
           ├── t (end of "cat")
           └── r (end of "car")
                └── d (end of "card")
```

Searching "card" means walking c → a → r → d, just 4 steps, no matter how many other words are stored.

**Rust — the real design decision:**
```rust
// Flexible alphabet (any unicode char), but hashing overhead per step:
struct TrieNode { children: HashMap<char, Box<TrieNode>>, is_end: bool }

// Fixed alphabet (say, lowercase a-z), no hashing, just direct indexing:
struct TrieNode { children: [Option<Box<TrieNode>>; 26], is_end: bool }
```
Say both, then justify: "if the alphabet is small and fixed, the array is faster and simpler; if it's large or sparse (like full Unicode), a `HashMap` avoids wasting memory on empty slots."

---

## 16. Segment Tree

**The problem it solves:** "give me the sum (or min, or max) of any range in this array, fast, even while I keep updating individual elements." A plain array gives O(n) per range query; a segment tree gives O(log n) for both queries and updates.

**Idea:** build a tree where each node summarizes a *range*. The root summarizes the whole array, its two children summarize the two halves, and so on down to single elements.

```
Array: [2, 4, 5, 7, 1, 3]

Segment tree (each node = sum of its range):
                  [0-5]=22
                 /         \
           [0-2]=11       [3-5]=11
           /     \          /     \
       [0-1]=6  [2]=5   [3-4]=8  [5]=3
       /    \
    [0]=2  [1]=4
```

To answer "sum of range [1,4]," you only touch O(log n) nodes instead of scanning 4 elements one by one — the savings get huge as the array (and range) grows.

**Rust:** same trick as heaps — because the tree shape is predictable (always a near-complete binary tree), it's stored as a flat array (`Vec<i64>` of size ~4n), not as `Box` pointers. No ownership complexity.

**Good follow-up to volunteer:** "lazy propagation" lets you update a whole *range* (not just one element) in O(log n) too, by postponing updates to children until they're actually needed.

---

## 17. Fenwick Tree / Binary Indexed Tree (BIT)

**The problem it solves:** segment trees are powerful but a bit heavy (lots of code, ~4n memory). If all you need is **prefix sums** (sum from index 0 to i), there's a much slimmer structure that does the same job with way less code — the Fenwick tree.

**The trick:** it exploits binary representation of indices — each slot in the array is responsible for a range of elements determined by the lowest set bit of its index. It sounds like magic, but the effect is: update one element, or ask for a prefix sum, both in O(log n), using just one flat array.

```rust
fn update(bit: &mut [i64], mut i: usize, delta: i64) {
    while i < bit.len() { bit[i] += delta; i += i & i.wrapping_neg(); } // add lowest set bit
}
fn query(bit: &[i64], mut i: usize) -> i64 {
    let mut sum = 0;
    while i > 0 { sum += bit[i]; i -= i & i.wrapping_neg(); } // remove lowest set bit
    sum
}
```

**"Segment tree vs Fenwick" — the answer interviewers want:** Fenwick = much less code and memory, but *only* naturally supports "combinable from the left" operations like sum or XOR. Segment tree = more code, but handles anything (min, max, gcd) and supports lazy range updates. Use Fenwick when you can, segment tree when you need the extra power.

---

## 18. Suffix Tree

**The problem it solves:** "does this substring appear anywhere in this huge string, and how fast can I check?" A suffix tree is a compressed trie of *every suffix* of a string, so any substring search becomes a simple walk down the tree — O(pattern length), regardless of how big the original string is.

```
String: "banana"
Suffixes: banana, anana, nana, ana, na, a

The suffix tree merges all the shared prefixes among these suffixes
into one compact tree, so "ana" (which appears twice) is findable
in O(3) steps, not by scanning the whole string.
```

**Honest interview note:** building one properly (Ukkonen's algorithm) is genuinely hard and almost nobody is expected to code it live. What you're expected to know is *what problem it solves*, that it's non-trivial to build, and that the practical substitute (see #26, Suffix Array) gets you most of the benefit with far less pain.

---

# 2. Graph Data Structures

## 19. Graph (Directed / Undirected / Weighted / Unweighted)

**The problem it solves:** trees assume a strict hierarchy (one parent). Real-world relationships aren't always hierarchical — friends, road networks, dependencies — anything can connect to anything, including cycles. A graph is just: a bunch of dots (vertices), and lines between them (edges), with no shape restrictions at all.

```
Undirected, unweighted:          Directed, weighted:

  A --- B                         A --3--> B
  |     |                         ^        |
  D --- C                         5        2
                                  |        v
                                  D <--1-- C
```

**Two ways to represent it, and when to use which:**
```rust
// Adjacency list — good for sparse graphs (most real graphs), O(V+E) space
let graph: Vec<Vec<usize>> = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];

// Adjacency matrix — good for dense graphs, O(1) "is there an edge?" check
let matrix: Vec<Vec<bool>> = vec![
    vec![false, true, false, true],
    vec![true, false, true, false],
    // ...
];
```

**The Rust-specific insight worth volunteering:** unlike trees, graphs have cycles and multiple incoming edges, so a `Box`/`Rc` pointer-based graph gets painful fast. The idiomatic Rust answer is almost always **index-based**: nodes live in a `Vec`, and edges are just `usize` indices into that vector. No ownership fights, no `Rc<RefCell<>>` soup.

**Directed vs undirected:** undirected just means "add the edge both ways" in the adjacency list. **Weighted vs unweighted decides your algorithm:** plain BFS finds shortest paths when unweighted; once edges have weights, you need Dijkstra's (non-negative weights) or Bellman-Ford (handles negative weights).

---

## 20. Disjoint Set / Union-Find

**The problem it solves:** "are these two things in the same group?" and "merge these two groups" — over and over, quickly — without ever needing to know the full group membership or split groups apart. Classic use: checking if adding a road would create a cycle (Kruskal's minimum spanning tree algorithm), or "are these two people connected in a social network."

```
Start: every element is its own group (its own parent).
[0] [1] [2] [3] [4]

union(0,1), union(2,3):

  0        2
  |        |
  1        3       4 (still alone)

find(1) walks up to 0 -> "1's group leader is 0"
```

**Two cheap tricks that make it almost O(1):**
- **Path compression:** while walking up to find the group leader, re-point every node you pass directly to the leader — so next time it's a one-hop lookup.
- **Union by rank/size:** always attach the smaller tree under the bigger tree's root, so trees stay shallow.

```rust
struct DSU { parent: Vec<usize>, rank: Vec<usize> }
impl DSU {
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }
    fn union(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb { return; }
        if self.rank[ra] < self.rank[rb] { self.parent[ra] = rb; }
        else if self.rank[ra] > self.rank[rb] { self.parent[rb] = ra; }
        else { self.parent[rb] = ra; self.rank[ra] += 1; }
    }
}
```
**Rust bonus:** this structure is just two flat `Vec<usize>`s — zero ownership complexity, which makes it a great "clean, fast to write correctly" interview problem.

---

# 3. Hashing / Specialized Structures

## 21. Hash Set

**The problem it solves:** "is this element in my collection?" — as fast as possible. Scanning a list is O(n). A sorted tree gets you to O(log n). A hash set gets you to O(1) *on average* by converting the element into a number (a hash) and using that number to jump directly to roughly the right spot.

```
hash("apple")  = 7  -> bucket 7
hash("banana") = 3  -> bucket 3
hash("cherry") = 7  -> collision with "apple"! handled by chaining:

bucket 3: [banana]
bucket 7: [apple] -> [cherry]   (linked list, or another probe strategy)
```

**Rust-specific facts worth knowing:**
- `std::collections::HashSet`/`HashMap` internally use **SwissTable** (via the `hashbrown` crate) — an open-addressing design with SIMD-accelerated probing, faster in practice than classic chaining.
- Rust's **default hasher (SipHash) is randomized per process**, specifically to prevent "HashDoS" attacks — an adversary can't craft inputs that all collide into the same bucket to slow your program to a crawl, because they don't know the random seed. This is a deliberate security choice most languages don't make by default.

**Trade-off vs BTreeSet:** HashSet is faster (O(1) avg) but unordered; BTreeSet is a bit slower (O(log n)) but keeps things sorted. Pick based on whether you ever need sorted iteration.

---

## 22. Bloom Filter

**The problem it solves:** "have I seen this before?" when you have *billions* of items and can't afford to store them all — you're willing to accept a small, tunable chance of false positives ("might have seen it") in exchange for using a tiny fraction of the memory, as long as you *never* get false negatives ("definitely haven't seen it" must always be true).

```
Bit array (all zeros to start): [0,0,0,0,0,0,0,0]

add("apple"): run it through 3 hash functions -> set bits 1, 4, 6
              [0,1,0,0,1,0,1,0]

add("banana"): hash functions -> set bits 2, 4, 7
              [0,1,1,0,1,0,1,1]

check("apple"): are bits 1, 4, 6 all set? yes -> "probably yes"
check("cherry"): are its 3 bits all set? if even one is 0 -> "definitely no"
```

**Why this matters practically:** used as a cheap pre-filter before an expensive check — e.g., "before doing a slow disk lookup, first ask the bloom filter; if it says no, skip the disk entirely."

**Known limitation to mention unprompted:** you can't delete from a standard bloom filter (clearing a bit might break another item that shares it). The fix, if asked, is a **Counting Bloom Filter** — counters instead of plain bits, so you can decrement instead of clear.

---

## 23. Skip List

**The problem it solves:** balanced trees (AVL, Red-Black) give O(log n) search but need rotations to stay balanced, which is a pain to make thread-safe. A skip list gets the same O(log n) *expected* performance using pure randomness instead of strict rebalancing rules — just a linked list with random "express lanes" on top.

```
Level 3:  1 --------------------> 9
Level 2:  1 --------> 5 --------> 9
Level 1:  1 --> 3 --> 5 --> 7 --> 9
Level 0:  1->2->3->4->5->6->7->8->9   (the real, full list)

Searching for 7: start at top level, skip as far as possible without
overshooting, drop down a level, repeat. Way fewer hops than scanning
the bottom row one by one.
```

Each node's height is decided by a coin flip when it's inserted (heads → grow another level) — no rebalancing logic needed at all.

**Why Rust developers specifically like this:** rotating a balanced tree touches multiple nodes atomically, which is hard to do safely with multiple threads. A skip list only ever *splices forward pointers*, which is much easier to make lock-free. That's exactly why crates like `crossbeam-skiplist` use skip lists for concurrent sorted maps instead of a balanced tree.

---

## 24. Bitset / Bit Vector

**The problem it solves:** if you just need a big array of true/false flags, storing each as a whole `bool` (1 byte, sometimes more with padding) wastes massive amounts of memory. A bitset packs 8 (or 64, using a `u64`) flags into every single byte/word, and lets you do set operations (union, intersection) on 64 flags at once with a single CPU instruction.

```
Instead of: [true, false, true, true, false, false, true, false]  (8 bytes)
Store as one byte: 1  0  1  1  0  0  1  0   ->  0b10110010  (1 byte)

Union of two bitsets = just OR the underlying integers together.
```

**Rust:** the `bitvec` crate is the idiomatic, well-supported choice (arbitrary bit widths, slicing). You can also hand-roll it: `word = i / 64; bit = i % 64; is_set = (arr[word] >> bit) & 1`.

**Used for:** Sieve of Eratosthenes, "bitmask DP" (dynamic programming over subsets), Bloom filters, permission/flag sets.

---

# 4. String Data Structures

*(Trie = #15, Suffix Tree = #18, both covered above.)*

## 26. Suffix Array

**The problem it solves:** suffix trees are powerful but genuinely hard to implement. A suffix array gives almost the same power (substring search, longest common substring) using just a *sorted list of suffix starting positions* — a flat array instead of a pointer-heavy tree, which is both easier to build and more cache-friendly.

```
String: "banana"
All suffixes, sorted alphabetically:
  a       (index 5)
  ana     (index 3)
  anana   (index 1)
  banana  (index 0)
  na      (index 4)
  nana    (index 2)

Suffix array = [5, 3, 1, 0, 4, 2]
```

Once sorted, you can binary-search for any substring's location. Pair it with an **LCP (Longest Common Prefix) array** — recording how much each adjacent pair of sorted suffixes shares — to unlock even more queries (like longest repeated substring).

```rust
fn suffix_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by_key(|&i| &s[i..]); // simple O(n^2 log n); mention prefix-doubling
    sa                            // exists for O(n log^2 n) if pushed on efficiency
}
```

**Why interviewers prefer this over asking you to code a suffix tree:** it's realistically writable under time pressure, and it's "just a `Vec<usize>`" — no pointer/ownership complexity at all.

---

## 28. Rope

**The problem it solves:** editing a huge string (think: a text editor with a 500MB file open). A normal `String` stores everything contiguously, so inserting a character in the middle means shifting every byte after it — O(n) per keystroke. A rope avoids that by storing the text as a tree of small chunks.

```
                [len=11]
               /         \
        [len=5]           [len=6]
        /     \             /     \
   "Hello"   " Wor"      "ld! "  "🙂"

Each internal node just stores "how many characters are in my left
subtree" — so finding character #8, or splicing in new text, only
touches O(log n) nodes instead of the whole string.
```

**Rust:** the `ropey` crate is the real production answer — name-drop it, it shows ecosystem knowledge. Internally it's `Box`-tree-shaped (strict parent→child ownership, same reasoning as a BST), with one Rust-specific wrinkle worth mentioning: leaves must be careful never to split a chunk in the middle of a multi-byte UTF-8 character.

---

# 5. Advanced Data Structures

## 29. Treap (Tree + Heap)

**The problem it solves:** AVL and Red-Black trees stay balanced through fairly fiddly explicit rules (height tracking, color invariants). A treap gets balance "for free," using randomness: every node gets a random priority in addition to its key, and the tree is arranged to be a valid BST on the *keys* while also being a valid max-heap on the *priorities*.

```
Keys form BST order, priorities (in brackets) form heap order:

           (key=5, pri=90)
           /              \
   (key=2, pri=70)     (key=8, pri=60)
                              \
                          (key=9, pri=40)
```

Since priorities are random, the resulting shape is (in expectation) the same as if you'd inserted keys into a BST in a *random* order — which is provably balanced, O(log n) expected height, without ever explicitly checking heights or colors.

**Why this is a nice one to bring up if asked "what would you actually enjoy implementing under time pressure":** insert/delete are done with the exact same rotation primitive as AVL trees, but with none of the bookkeeping — just compare random numbers. `Box<Node>` works fine, no parent pointers needed.

---

## 30. Splay Tree

**The problem it solves:** "recently used things tend to get used again soon" (cache-style access patterns). A splay tree has *no* explicit balance rule at all — instead, every time you access a node, you rotate it all the way up to the root ("splaying"). Frequently-used items naturally end up near the top, making repeat access very fast.

```
Access node 9 in this tree:                After splaying 9 to the root:

     5                                              9
    / \                                            / \
   2   8                 ---splay(9)-->            5   ...
        \
         9

(Exact shape depends on rotation path, but the point stands:
 whatever you just touched ends up at the root.)
```

**Complexity:** amortized O(log n) — not guaranteed for any single operation, but guaranteed *on average* over many operations, and much better than that if your access pattern is skewed (some items accessed way more than others).

**Rust twist:** naive splaying needs to walk back *up* the tree, which (like Red-Black trees) needs parent pointers — `Rc<RefCell<>>` + `Weak`, or an arena. The cleaner Rust-friendly trick is **top-down splaying**, which restructures the tree *as you descend* looking for the node, so you never need to store parent pointers at all. Mentioning that you know top-down splaying exists is a strong signal.

---

## 31. Interval Tree

**The problem it solves:** "which of these date ranges/time slots overlap with this new one I want to book?" Checking every interval one by one is O(n). An interval tree answers this in O(log n + k), where k is just the number of actual matches.

```
It's a BST ordered by interval START, but each node ALSO stores the
max END value anywhere in its subtree, so you can skip whole branches
that can't possibly overlap:

        [15,20] max=30
        /             \
  [5,10] max=20      [25,30] max=30
       \
      [12,18] max=18

Query [6,7]: compare against root's max (30) - could overlap somewhere
on the left, so go left; check [5,10] - no overlap with [6,7], but its
max (20) says keep checking its subtree... etc.
```

**Used for:** calendar conflict detection, "which meetings overlap," computational geometry.

**Rust:** it's just a BST (`Box<Node>`) with one extra field (`max_end`) that gets updated bottom-up whenever you insert — a nice example of the general interview pattern "take a structure you know and augment it with one extra piece of info to unlock a new query."

---

## 32. K-D Tree (k-dimensional tree)

**The problem it solves:** "find the nearest point to me" in 2D, 3D, or higher-dimensional space (maps, graphics, ML). A K-D tree generalizes a BST to multiple dimensions by cycling *which* dimension you compare on as you go deeper.

```
Points: (3,6), (17,15), (13,15), (6,12), (9,1), (2,7)

Depth 0 splits on x, depth 1 splits on y, depth 2 back to x, etc:

                (9,1)               <- split on x here
               /       \
          (3,6)       (13,15)       <- split on y here
          /    \             \
       (2,7)  (6,12)        (17,15)
```

This lets you prune huge chunks of space during a nearest-neighbor search instead of checking every point.

**Rust:** `Box<Node>` tree, each node stores a point, comparator alternates dimension by depth. Name-drop the `kiddo` crate for real usage.

**Honest caveat to mention:** K-D trees degrade toward brute-force search in very high dimensions (the "curse of dimensionality") — at large scale, people switch to approximate methods like LSH or HNSW instead.

---

## 33. Cartesian Tree

**The problem it solves:** it's less a standalone tool and more a clever *bridge* between two classic problems: Range Minimum Query (RMQ) on an array, and Lowest Common Ancestor (LCA) on a tree. A Cartesian tree built from an array is simultaneously a valid BST on *index* (walking it in-order gives back the original array) and a valid min-heap on *value* (the smallest value is always the root).

```
Array: [9, 3, 7, 1, 8, 12, 10, 20, 15, 18, 5]

The minimum (1) becomes the root; everything to its left becomes the
left subtree (recursively built the same way), everything to its
right becomes the right subtree:

                     (1)
                   /      \
              (3)          (5)
             /    \        /
          (9)    (7)    (8)...
```

**Why interviewers ask this:** it's a clean way to check if you understand *reductions* — "the min of range [l,r] in the array is the value at the LCA of nodes l and r in the Cartesian tree." Turning one hard problem into a different, already-solved problem is a core skill, not just knowing structures individually.

**Rust:** buildable in O(n) using a monotonic stack, producing a normal `Box<Node>` tree.

---

## 34. Van Emde Boas Tree (vEB Tree)

**The problem it solves:** extremely fast operations — O(log log U), where U is the size of the *universe* of possible keys (e.g., U = 2^32 if keys are `u32`s) — by recursively splitting the universe into √U-sized chunks, each handled by a smaller vEB tree, plus a "summary" structure that lets you skip entirely empty chunks.

```
Universe of size 16 splits into 4 "clusters" of size 4 each,
plus a summary tree (itself a smaller vEB) tracking which
clusters are non-empty:

Universe [0..16)
 ├── cluster 0: [0,1,2,3]
 ├── cluster 1: [4,5,6,7]
 ├── cluster 2: [8,9,10,11]
 └── cluster 3: [12,13,14,15]
 summary: tracks which of the 4 clusters have ANY element
```

**Honest interview framing:** this is a "do you know it exists and roughly why" question, not a "code it live" question. It's rarely used in real production code because it needs O(U) memory in its simple form (huge for large key universes) — it's more of a theoretical existence-proof that O(log log U) is achievable. Describing the recursive universe-splitting idea, and honestly noting it's mostly seen in competitive programming/theory, is the correct-depth answer.

---

## 35. Merkle Tree

**The problem it solves:** "how do I verify a huge dataset hasn't been tampered with, without re-checking every byte?" A Merkle tree hashes each small chunk of data at the leaves, then hashes pairs of hashes going up, until you get a single root hash that's a fingerprint of the *entire* dataset — and lets you prove any one piece is included using only O(log n) extra hashes (a "Merkle proof"), not the whole dataset.

```
        Root = hash(H_AB + H_CD)
           /              \
   H_AB=hash(A+B)      H_CD=hash(C+D)
     /       \            /       \
  hash(A)  hash(B)    hash(C)   hash(D)
    |        |          |         |
  data A   data B     data C    data D

To prove "data B is really in this dataset," you only need to reveal
hash(A), hash(C+D), and let the verifier recompute the root — no need
to send data A, C, or D at all.
```

**Used for:** Git (every commit is essentially a Merkle tree of the repo state), blockchains (verifying transactions in a block), peer-to-peer file sharing (BitTorrent), verifying that two copies of a distributed database actually match.

**Rust:** a plain `Box<Node>` binary tree where each node stores a hash (`[u8; 32]` for SHA-256), built bottom-up; `sha2` crate for the hashing itself. This one's genuinely fun to sketch on a whiteboard — simple structure, but demonstrates real systems/crypto awareness.

---

## 36. Persistent Data Structures

**The problem it solves:** "I want to modify this structure, but keep the old version around too" (undo/redo, version history, safe concurrent reads while a write is happening) — without paying the cost of copying the entire structure on every change.

**The trick — structural sharing:** only copy the *path* from the root down to the part you actually changed. Everything else is shared, unchanged, between the old and new versions.

```
Original list: A -> B -> C

Prepend X (functional style, doesn't mutate the original):

New list:  X -> A -> B -> C
Old list:       A -> B -> C     (still valid, still usable!)

X's "next" pointer just points at the *same* A -> B -> C chain that
already existed — nothing was copied, just one new node was added.
```

**Why Rust specifically makes this natural (not a workaround, the actual right tool):** this is one of the rare cases where `Rc` (reference counting) is exactly the right fit rather than a way to dodge the borrow checker — persistent structures are *built* around the idea of many things sharing one read-only piece of data, and cloning an `Rc` is a cheap O(1) refcount bump, not a deep copy.

```rust
#[derive(Clone)]
enum PersistentList<T> { Nil, Cons(Rc<T>, Rc<PersistentList<T>>) }

fn prepend<T>(list: &Rc<PersistentList<T>>, val: T) -> Rc<PersistentList<T>> {
    Rc::new(PersistentList::Cons(Rc::new(val), Rc::clone(list))) // O(1), old list untouched
}
```

**Used for:** functional languages' default collections (Clojure, Haskell), undo/redo systems, Git-like version control, lock-free reads in concurrent systems (readers keep using an old version while a writer builds a new one — no locking needed). Rust crate to know: `im` (immutable/persistent collections, ready to use).

---

## Quick "which structure when" cheat sheet

| Need | Structure |
|---|---|
| Ordered data, guaranteed O(log n) | AVL (read-heavy) / Red-Black (write-heavy) / B-Tree (disk/cache-friendly — what Rust's `BTreeMap` actually uses) |
| Fast unordered lookup | HashSet / HashMap |
| Priority queue | Heap (`BinaryHeap`) |
| Autocomplete / prefix search | Trie |
| Range sum/min queries | Segment Tree (general) / Fenwick Tree (prefix-sum only, less code) |
| "Same group?" merging, no splitting | Union-Find |
| Membership test at huge scale, OK with false positives | Bloom Filter |
| Lock-free concurrent ordered structure | Skip List |
| Multi-dimensional nearest neighbor | K-D Tree |
| Editable huge text | Rope |
| Immutable/versioned data, cheap sharing | Persistent structure via `Rc` / `im` crate |
| Tamper-proofing / data integrity | Merkle Tree |
| Booking/calendar overlap checks | Interval Tree |

**The meta-answer for any "how would you build this in Rust" question:**
- Strictly tree-shaped, no cycles, no need to go "up"? → `Box`.
- Need parent pointers or the structure can have cycles? → arena (`Vec` + indices) is the clean, idiomatic choice; `Rc<RefCell<>>` + `Weak` if you specifically need shared mutable ownership.
- Need cheap, shared, *immutable* subtrees across versions? → `Rc` — the one case where it's the *correct* tool, not a workaround.
