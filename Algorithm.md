# 120 Algorithms, From First Principles

For each algorithm: **the problem that made it necessary**, and **the core idea** that solves it.

---

## 1. Searching
Problem class: given a collection, find whether/where a target value exists — as fast as possible.

1. **Linear Search** — No structure to exploit? Just check every element one by one. Exists because it's the only option when data is unsorted/unindexed. O(n), the baseline everything else tries to beat.
2. **Binary Search** — If data is *sorted*, checking every element is wasteful. Exists to exploit order: compare to the middle, discard half the search space each time. O(log n).
3. **Ternary Search** — For unimodal functions (one peak/valley), binary search's "left or right" doesn't directly apply to finding an extremum. Splits into three parts to locate a maximum/minimum instead of an exact value.
4. **Jump Search** — Binary search needs random access and log n comparisons; on media where each "jump back" is costly, jump search moves forward in fixed blocks, then linearly scans within a block — fewer bidirectional jumps, O(√n).
5. **Interpolation Search** — Binary search always looks at the middle, even when data is uniformly distributed and you could guess the position better (like flipping to the "S" section of a phone book by name, not always the middle page). Estimates position based on value distribution — O(log log n) on uniform data.
6. **Exponential Search** — Binary search needs to know the array's bounds. For unbounded/streamed sorted data, exponential search first finds a range (1,2,4,8...) containing the target, then binary searches within it.

---

## 2. Sorting
Problem class: reorder elements into a defined order. Exists because almost every other algorithm (searching, deduplication, scheduling) becomes cheaper/easier on sorted data.

7. **Bubble Sort** — Simplest possible idea: repeatedly swap adjacent out-of-order pairs until nothing needs swapping. Exists as the conceptual entry point to sorting, not for practical speed (O(n²)).
8. **Selection Sort** — Repeatedly find the minimum of the unsorted part and place it at the front. Minimizes the *number of swaps* (useful when writes are expensive), at the cost of always O(n²) comparisons.
9. **Insertion Sort** — Mimics how people sort playing cards: take each new element and insert it into the already-sorted portion. Very fast on nearly-sorted data (adaptive), used as the base case inside faster hybrid sorts.
10. **Merge Sort** — Comparison sorting is bounded below by O(n log n); merge sort achieves it by splitting the array recursively and merging sorted halves. Exists for guaranteed O(n log n) and stability, crucial when data can't fit in memory (external sorting).
11. **Quick Sort** — Merge sort needs extra memory to merge; quicksort sorts in-place by picking a pivot, partitioning around it, and recursing. Exists for speed in practice (good cache behavior) despite worst-case O(n²).
12. **Heap Sort** — Quicksort's worst case is bad, merge sort needs extra space; heap sort builds a heap (structure enabling O(log n) max-extraction) and repeatedly extracts the max. Guarantees O(n log n) in-place.
13. **Counting Sort** — Comparison sorts can't beat O(n log n) — but if you're not comparing, that bound doesn't apply. Counts occurrences of each value (needs a bounded range of integers) and places them directly. O(n+k).
14. **Radix Sort** — Counting sort needs the value range k to be small; radix sort sorts numbers digit-by-digit (using counting sort as a subroutine per digit) so huge integers can be sorted in linear time without comparing them directly.
15. **Bucket Sort** — For data spread uniformly over a range (e.g., floats in [0,1)), scatter elements into buckets, sort each small bucket, concatenate. Exists to exploit uniform distribution for near-linear time.
16. **Shell Sort** — Insertion sort is slow because elements move one step at a time; shell sort first sorts elements far apart (by a gap sequence), shrinking the gap, so elements travel farther in fewer moves before a final insertion sort pass.

---

## 3. Array / Two-Pointer Techniques
Problem class: process arrays in less than O(n²) time by avoiding redundant re-scanning.

17. **Two Pointers** — Many problems (pair sums in sorted arrays, container problems) get solved with nested loops in O(n²). Using two indices that move toward/away from each other based on a condition collapses this to O(n), because each pointer only moves forward.
18. **Sliding Window** — Computing something (sum, max, distinct count) over every subarray/substring of size k or satisfying a condition naively costs O(n·k). A window that expands/contracts and updates incrementally reuses previous work, hitting O(n).
19. **Prefix Sum** — Answering many "sum of range [i,j]" queries by re-adding each time is O(n) per query. Precomputing running sums lets any range sum be answered in O(1) via subtraction.
20. **Difference Array** — The inverse problem: applying many range *updates* (add v to [i,j]) is O(range) each if done directly. Storing only the +v/-v at boundaries and prefix-summing at the end reduces total update cost to O(1) per update.
21. **Kadane's Algorithm** — Finding the maximum-sum contiguous subarray by checking all O(n²) subarrays is wasteful. Kadane's tracks "best sum ending here," resetting when it turns negative, solving it in O(n).
22. **Dutch National Flag Algorithm** — Sorting an array of only 3 distinct values (e.g., 0s,1s,2s) with a general sort is overkill (O(n log n)). Three pointers partition the array into three regions in one O(n) pass.
23. **Boyer-Moore Majority Vote** — Finding the element occurring more than n/2 times normally needs a hashmap (O(n) space). This algorithm cancels out one occurrence of two different elements at a time, using the fact that a true majority element survives cancellation — O(n) time, O(1) space.

---

## 4. Recursion & Backtracking
Problem class: exploring all valid configurations/combinations of a solution space where brute-force enumeration is the only correct approach, but naive enumeration explodes.

24. **Basic Recursion** — Some problems (factorial, tree traversal, divide-and-conquer) are naturally defined in terms of smaller versions of themselves. Recursion exists because it lets code directly mirror that self-referential structure instead of manually managing a stack.
25. **Generate Subsets** — Enumerate all 2^n subsets of a set (power set) — needed for exhaustive search problems. Solved by, for each element, recursively choosing to include or exclude it.
26. **Generate Permutations** — Enumerate all n! orderings — needed wherever order matters (scheduling, TSP brute force). Solved by fixing one element at a time and recursing on the rest.
27. **Combination Sum** — Find all combinations of numbers (reuse allowed or not) summing to a target. Solved by recursively picking numbers and backtracking when the sum overshoots — a template for constrained subset search.
28. **N-Queens** — Place N queens on an N×N board so none attack another. Represents "place items with pairwise constraints" problems; backtracking places one queen per row and abandons a branch the moment a conflict appears, avoiding checking all N^N placements.
29. **Sudoku Solver** — A constraint-satisfaction problem: fill cells so row/column/box constraints hold. Backtracking tries a candidate digit, recurses, and undoes the choice if it leads to a dead end — a general pattern for CSPs.
30. **Rat in a Maze** — Find a path through a grid with obstacles. Represents pathfinding-by-exhaustive-exploration; backtrack out of dead ends rather than exploring the entire grid blindly.
31. **Backtracking / Constraint Search** — The umbrella technique behind 27–30: build a solution incrementally, and the instant a partial solution violates a constraint, abandon it (prune) rather than completing it — turning exponential brute force into something often much faster in practice.

---

## 5. Linked List
Problem class: linked lists give O(1) insertion/deletion but no random access, so array tricks (indexing) don't work — a distinct toolkit is needed.

32. **Reverse Linked List** — Reversing an array is trivial with indices; a linked list has only forward pointers. Solved by iterating and flipping each `next` pointer to point backward.
33. **Fast & Slow Pointers** — Without indices, you can't jump to "the middle" or detect a "loop" directly. Two pointers moving at different speeds exploit relative motion to answer these structural questions in O(n) time, O(1) space.
34. **Floyd's Cycle Detection** — Detecting a cycle in a list can't use a "visited" array without extra memory. If a fast pointer (2 steps) ever laps a slow pointer (1 step) inside a loop, they must meet — proving a cycle exists in O(1) space.
35. **Merge Two Sorted Lists** — Merging two sorted arrays is easy with indices; lists need pointer rewiring instead. Walk both lists, always attach the smaller head, and relink — O(n+m), no extra array.
36. **Find Middle of Linked List** — No length field, no indexing, so "give me the middle" needs a trick: slow pointer moves 1 step, fast moves 2; when fast reaches the end, slow is at the middle.
37. **Detect/Remove Cycle** — After detecting a cycle (Floyd's), removing it requires finding exactly where it begins — done by resetting one pointer to the head and advancing both at equal speed until they meet.
38. **Merge Sort on Linked List** — Quicksort/array merge sort rely on random access or extra arrays; on linked lists, merge sort adapts naturally since splitting/merging is just pointer rewiring — no extra array needed, so it's the preferred sort for lists.

---

## 6. Stack & Queue
Problem class: many array problems need to know "the nearest bigger/smaller element" or a running extremum — brute force is O(n²); stacks/queues that maintain useful order collapse it to O(n).

39. **Monotonic Stack** — Keep a stack that's always increasing or decreasing; when a new element violates that order, pop until it doesn't. Exists to answer "next greater/smaller element" style queries in O(n) instead of O(n²).
40. **Monotonic Queue** — Same idea as the monotonic stack but supports removing from both ends, needed to maintain a "max/min of the last k elements" as the window slides.
41. **Next Greater Element** — For each element, find the first larger element to its right. Brute force is O(n²); a monotonic (decreasing) stack solves it in O(n) by only ever comparing each element a constant number of times.
42. **Previous Greater Element** — Mirror of 41, scanning leftward instead — same monotonic stack idea, different direction, used in histogram and stock-span type problems.
43. **Largest Rectangle in Histogram** — Finding the biggest rectangle under a skyline naively checks all O(n²) width×height pairs. A monotonic stack tracks bars still "active" as a potential rectangle boundary, solving it in O(n).
44. **Sliding Window Maximum** — Getting the max of every window of size k naively is O(n·k). A monotonic (decreasing) deque keeps only candidates that could still be the max, giving O(n) total.

---

## 7. Tree Algorithms
Problem class: hierarchical (non-linear) data needs traversal and query strategies that arrays/lists don't have.

45. **DFS (tree)** — Explore as deep as possible before backtracking, needed for tasks like path-finding, tree serialization, or subtree computations. Natural fit for recursion/stack.
46. **BFS (tree)** — Explore level-by-level, needed when "shortest path" or "distance from root" matters. Uses a queue to process nodes in order of depth.
47. **Preorder Traversal** — Visit node, then left, then right. Exists for tasks like copying a tree or prefix-expression generation, where you need the "root first" order.
48. **Inorder Traversal** — Visit left, node, right. For binary search trees, this always yields sorted order — exists specifically to extract sorted data from a BST in O(n).
49. **Postorder Traversal** — Visit left, right, node. Needed when children must be fully processed before the parent (e.g., deleting a tree, computing subtree sizes/heights).
50. **Level Order Traversal** — Same as tree BFS; needed for printing a tree level-by-level or computing width/serialization by depth.
51. **Binary Search Tree Operations** — Maintaining an always-sorted, efficiently searchable structure under insertions/deletions. BST property (left < node < right) allows O(log n) search/insert/delete on average, unlike O(n) for lists.
52. **Lowest Common Ancestor (LCA)** — Given two nodes, find their deepest shared ancestor — needed for tasks like relationship queries or path computation between nodes. Solved via recursive search, binary lifting, or Euler tour + RMQ, depending on constraints.
53. **Tree Diameter** — Find the longest path between any two nodes in a tree. Solved by two DFS/BFS passes (or DP), because the diameter always passes through specific farthest-node pairs — checking all O(n²) pairs directly is unnecessary.
54. **Tree Height/Depth** — Basic structural measurement needed for balancing decisions and recursion bounds; computed recursively as 1 + max(height of children).
55. **Binary Lifting** — Answering "k-th ancestor" or repeated LCA queries by walking up one node at a time is O(n) per query. Precomputing 2^i-th ancestors lets any jump be done in O(log n) via binary decomposition of k.

---

## 8. Graph Algorithms
Problem class: relationships between entities (not just hierarchy) — reachability, shortest paths, connectivity, ordering with dependencies.

56. **BFS (graph)** — Find shortest paths in *unweighted* graphs, or explore level-by-level. Exists because DFS doesn't guarantee shortest path; BFS's queue-based layer expansion does.
57. **DFS (graph)** — Explore as far as possible along each branch; foundational for connectivity checks, cycle detection, and as a subroutine in more complex algorithms (Tarjan's, topological sort).
58. **Topological Sort** — Given tasks with dependencies (a DAG), find a valid execution order. Necessary anywhere "do A before B" constraints exist (build systems, course prerequisites) — solved via DFS finish-order or Kahn's algorithm.
59. **Kahn's Algorithm** — An alternative to DFS-based topological sort using in-degree counting: repeatedly remove nodes with no remaining dependencies. Exists because it naturally also detects cycles (if not all nodes get removed, a cycle exists).
60. **Dijkstra's Algorithm** — Find shortest paths from a source in a graph with non-negative weights. BFS fails once edges have weights; Dijkstra's greedily expands the nearest unvisited node using a priority queue, exists because greedy choice is provably optimal when weights are non-negative.
61. **Bellman-Ford** — Dijkstra's breaks with negative edge weights. Bellman-Ford relaxes all edges repeatedly (n-1 times), tolerating negative weights and detecting negative cycles, at the cost of O(VE) instead of Dijkstra's speed.
62. **Floyd-Warshall** — Need shortest paths between *all pairs* of nodes, not just from one source. Running Dijkstra/Bellman-Ford from every node is possible but Floyd-Warshall's DP (try every node as an intermediate) does it directly in O(V³), handling negative weights (no negative cycles).
63. **Prim's Algorithm** — Build a Minimum Spanning Tree (connect all nodes with minimum total edge weight, no cycles) by growing a tree one cheapest edge at a time from the current tree — good on dense graphs.
64. **Kruskal's Algorithm** — Same MST goal as Prim's, different strategy: sort all edges by weight, add the smallest edge that doesn't form a cycle (checked via Union-Find) — good on sparse graphs.
65. **A\* Search** — Dijkstra's explores blindly outward; when you have a heuristic estimate of distance to the goal (e.g., straight-line distance on a map), A* uses it to prioritize promising directions, reaching the goal faster.
66. **Tarjan's Algorithm** — Find Strongly Connected Components (SCCs) — groups of nodes all mutually reachable — in a single DFS pass using discovery times and low-link values, avoiding multiple graph passes.
67. **Kosaraju's Algorithm** — An alternative SCC algorithm: DFS to get finish-order, reverse the graph, DFS again in that order. Conceptually simpler than Tarjan's at the cost of two passes and a reversed graph.
68. **Union-Find / DSU** — Repeatedly need to ask "are these two elements in the same group?" and "merge these two groups" (used in Kruskal's, cycle detection, connectivity). Naive checking is slow; union-find with path compression and union by rank makes both operations nearly O(1).

---

## 9. Dynamic Programming
Problem class: problems with overlapping subproblems and optimal substructure, where naive recursion recomputes the same states exponentially many times. DP exists to store and reuse those results.

69. **0/1 Knapsack** — Choose items (each usable once) to maximize value under a weight limit. Brute force checks 2^n subsets; DP builds a table of best value per (item, remaining capacity), reusing overlapping states.
70. **Unbounded Knapsack** — Same as 0/1 but items can be reused unlimited times (e.g., coin-like problems) — the DP transition allows reusing the current item, not just moving to the next.
71. **Coin Change** — Find minimum coins (or number of ways) to make a target amount. A specific unbounded-knapsack-style DP; exists because greedy coin selection fails for arbitrary denominations.
72. **Longest Common Subsequence (LCS)** — Find the longest sequence common to two strings (used in diff tools, DNA comparison). DP compares characters and builds up from smaller prefixes, avoiding recomputation across the exponential number of subsequences.
73. **Longest Increasing Subsequence (LIS)** — Find the longest strictly increasing subsequence. Naive check is O(2^n); DP tracks the best LIS ending at each index in O(n²), improvable to O(n log n) with binary search.
74. **Edit Distance** — Minimum operations (insert/delete/replace) to transform one string into another — foundational for spell-checkers and diff tools. DP builds the answer from smaller prefixes of both strings.
75. **Matrix Chain Multiplication** — Multiplying a chain of matrices in different orders gives the same result but wildly different costs. DP finds the optimal parenthesization by trying all split points on subchains, avoiding the exponential number of full orderings.
76. **House Robber** — Maximize sum of non-adjacent elements. A simple linear DP (rob current + best-two-back, or skip) illustrating the core "choose or skip" DP pattern.
77. **Maximum Subarray** — Same problem Kadane's solves; included here as the canonical example of 1D DP (best-ending-here recurrence).
78. **Grid DP** — Count paths / find min-cost path through a grid with movement restrictions (e.g., only right/down). Exists because each cell's answer depends only on a few neighbors — reused instead of recomputed.
79. **Interval DP** — Problems where you must choose how to split/combine an interval (like matrix chain, or palindrome partitioning). DP considers every split point of every subinterval, avoiding exponential recursive splitting.
80. **Bitmask DP** — When "state" involves a subset of items (e.g., which cities visited in TSP), a bitmask compactly encodes the subset as an integer, letting DP index by (subset, current position) — turns 2^n subset possibilities into indexable table cells.
81. **Digit DP** — Count numbers in a range satisfying digit-based constraints (e.g., "sum of digits = k") without enumerating every number. DP processes digit-by-digit, tracking constraints (tight bound, sum so far), covering ranges up to 10^18 efficiently.
82. **Tree DP** — DP where the "subproblems" are subtrees (e.g., max independent set in a tree). Exists because tree structure gives a natural recursive decomposition — compute each subtree's answer from its children's answers.

---

## 10. Greedy Algorithms
Problem class: problems where locally optimal choices, made step by step, provably lead to a globally optimal solution — no need to explore all combinations like DP/backtracking.

83. **Activity Selection** — Choose the max number of non-overlapping activities from a schedule. Provably optimal to always pick the activity that finishes earliest — greedy avoids the exponential exploration of all subsets.
84. **Fractional Knapsack** — Like 0/1 knapsack but items can be split. Since fractions are allowed, always taking the highest value/weight ratio first is provably optimal — no DP needed, unlike the 0/1 version.
85. **Huffman Coding** — Build a minimum-redundancy prefix code for data compression. Exists because assigning shorter codes to more frequent symbols greedily (via a min-heap merging the two least frequent nodes repeatedly) minimizes expected code length.
86. **Job Sequencing** — Maximize profit scheduling jobs with deadlines, each taking one unit of time. Greedily schedule the highest-profit job as late as possible before its deadline — a scheduling variant of the general "greedy with deadlines" pattern.
87. **Interval Scheduling** — General family covering 83/86: choosing/ordering intervals under constraints to optimize a target — greedy works because of the exchange-argument property of intervals.
88. **Minimum Platforms** — Given arrival/departure times of trains, find the minimum platforms needed simultaneously. Solved by sorting arrivals and departures separately and sweeping — a greedy/two-pointer hybrid, avoiding simulating every minute.
89. **Gas Station** — Determine if a circular trip is completable given gas/cost at each station. A greedy observation (if total gas ≥ total cost, a valid start exists, found by tracking running deficit) avoids checking all n starting points individually.
90. **Jump Game** — Determine if you can reach the end of an array via jumps of varying max length. Greedily track the farthest reachable index while scanning — avoids exponential branching over jump choices.

---

## 11. Divide and Conquer
Problem class: problems solvable by splitting into independent subproblems, solving each, and combining results — exists to turn expensive brute force into recursively reduced work.

91. **Merge Sort** *(see #10)* — Divide-and-conquer applied to sorting: split, sort halves independently, merge.
92. **Quick Sort** *(see #11)* — Divide-and-conquer via partitioning around a pivot rather than a fixed midpoint.
93. **Binary Search** *(see #2)* — Divide-and-conquer applied to search: eliminate half the space each step.
94. **Closest Pair of Points** — Naively finding the closest pair among n points is O(n²). Divide-and-conquer splits points by x-coordinate, solves each half, and cleverly checks only a thin strip near the dividing line for cross-pairs — O(n log n).
95. **Strassen's Matrix Multiplication** — Standard matrix multiplication is O(n³). Strassen's splits matrices into quadrants and uses 7 (not 8) recursive multiplications with clever additions/subtractions, achieving O(n^2.81) — exists to prove/exploit that fewer multiplications are possible via smarter combination.

---

## 12. String Algorithms
Problem class: exact/approximate pattern matching and structural string queries where brute-force character comparison is too slow.

96. **KMP (Knuth-Morris-Pratt)** — Naive substring search re-checks characters after a mismatch, costing O(n·m). KMP precomputes a "failure function" (longest prefix-suffix overlap) so on a mismatch it never re-examines already-matched characters — O(n+m).
97. **Rabin-Karp** — Instead of comparing characters, hash the pattern and each window of text, comparing hashes (with a rolling hash update in O(1) per shift) — good for multiple pattern search and plagiarism-style matching.
98. **Z Algorithm** — Computes, for every position, the length of the longest substring starting there that matches the string's prefix — a single linear-time precomputation that answers many pattern-matching questions (including KMP-like search) uniformly.
99. **Manacher's Algorithm** — Finding the longest palindromic substring naively is O(n²) or O(n³). Manacher's exploits palindrome symmetry (mirroring already-computed results) to find it in O(n).
100. **Aho-Corasick** — Searching for *many* patterns individually with KMP costs O(k·n) for k patterns. Aho-Corasick builds a single automaton (trie + failure links) over all patterns, finding all matches in one O(n) pass over the text.
101. **Suffix Array** — A sorted array of all suffixes of a string, enabling binary-search-based substring queries and many string problems (longest repeated substring, etc.) in a compact O(n log n)-built structure, cheaper in memory than a suffix tree.
102. **Suffix Tree** — A compressed trie of all suffixes, enabling many string queries (substring search, longest common substring, repeats) in linear time after O(n) construction — the most powerful but most memory-heavy string structure.

---

## 13. Mathematical / Number Theory
Problem class: number-theoretic computations (divisibility, primality, modular arithmetic) that appear constantly in cryptography, combinatorics, and competitive programming — needing sub-brute-force algorithms.

103. **Euclidean Algorithm** — Computing GCD by checking all divisors is slow. Euclid's insight: gcd(a,b) = gcd(b, a mod b), shrinking the problem fast (related to Fibonacci-rate convergence) — O(log(min(a,b))).
104. **Extended Euclidean Algorithm** — Beyond just the GCD, many problems (modular inverse, Diophantine equations) need integers x,y such that ax+by=gcd(a,b). The extended version tracks these coefficients alongside the standard Euclidean steps.
105. **Sieve of Eratosthenes** — Checking primality of each number up to n individually (trial division) is slow in aggregate. The sieve marks off multiples of each prime starting from 2, finding all primes up to n in O(n log log n).
106. **Fast Exponentiation** — Computing a^b via repeated multiplication is O(b). Exploiting a^b = (a^(b/2))², squaring and halving the exponent gets it to O(log b).
107. **Modular Exponentiation** — Same idea as 106 but taking mod at every step to keep numbers from overflowing — essential for cryptography (RSA) where exponents/bases are huge.
108. **Modular Inverse** — Division doesn't exist directly in modular arithmetic; the modular inverse (via extended Euclid or Fermat's little theorem) lets you "divide" by multiplying by this inverse instead — needed for modular fractions in combinatorics/crypto.
109. **Chinese Remainder Theorem** — Given several modular congruences (x ≡ a mod m, x ≡ b mod n, ...), CRT reconstructs a unique x mod (m·n...) directly rather than brute-force searching — exists for combining independent modular constraints efficiently.
110. **Prime Factorization** — Decompose a number into prime factors — foundational for GCD/LCM, cryptography, and divisor problems. Trial division up to √n is the base method; sieve-based precomputation speeds up repeated queries.
111. **GCD / LCM** — GCD (largest shared divisor) and LCM (smallest shared multiple) are needed constantly for fractions, scheduling (cycles), and modular arithmetic; computed via the Euclidean algorithm and the identity LCM(a,b) = a·b / GCD(a,b).

---

## 14. Bit Manipulation
Problem class: operations exploiting the binary representation of numbers directly, often turning O(n) or O(2^n) problems into O(1)/O(log n) via hardware-level operations.

112. **Bitwise AND/OR/XOR** — The foundational operations for masking, toggling, and combining binary flags directly — exists because manipulating individual bits is far cheaper than arithmetic equivalents for flag-style problems.
113. **Brian Kernighan's Algorithm** — Counting set bits naively checks all 32/64 bits. This trick repeatedly does `n & (n-1)` to clear the lowest set bit, looping only as many times as there are set bits — faster when bits are sparse.
114. **XOR-based algorithms** — XOR's property (a^a=0, a^0=a) enables elegant tricks: finding a single non-duplicate in a list of pairs, swapping without a temp variable — exists to exploit self-cancellation for problems that seem to need extra memory.
115. **Bitmasking** — Representing a set of boolean choices/membership as bits of an integer, enabling fast set operations (union, intersection, subset check) and compact DP states (see Bitmask DP) — exists because bit operations on an int are far faster than manipulating actual set structures.
116. **Subset Enumeration** — Iterating all 2^n subsets of a set, or all submasks of a given bitmask, needed for exhaustive search over combinations — done efficiently via bit tricks (`for (s=mask; s; s=(s-1)&mask)`).

---

## 15. Computational Geometry
Problem class: spatial problems (points, lines, shapes) where naive pairwise checking is O(n²) or worse, and geometric structure can be exploited for speed.

117. **Convex Hull** — Find the smallest convex polygon enclosing a set of points (used in collision detection, shape analysis). Naive check of all point combinations is exponential; algorithms like Graham scan/Jarvis march sort/rotate points to build the hull in O(n log n).
118. **Line Intersection** — Determine whether/where two line segments cross — a basic primitive needed by almost every higher geometry algorithm (convex hull edge cases, polygon clipping), solved via cross-product orientation tests rather than solving raw equations naively each time.
119. **Sweep Line** — Many geometry problems (segment intersections, rectangle union area) become easier by conceptually sweeping a vertical line across the plane and maintaining an ordered structure of "active" objects — turns O(n²) pairwise checks into O(n log n).
120. **Closest Pair of Points** *(see #94)* — Included again as computational geometry's flagship divide-and-conquer + sweep-adjacent example: reducing brute-force O(n²) to O(n log n).

---

### The meta-pattern across all 120
Nearly every algorithm above exists to escape a brute-force cost (usually O(n²) or O(2^n)) by exploiting some structure in the problem: **sortedness** (binary search, merge sort), **overlapping subproblems** (DP), **monotonicity** (stacks/queues), **spatial/positional locality** (sweep line, two pointers), **graph structure** (BFS/DFS-based algorithms), or **number-theoretic identities** (GCD, modular arithmetic). Learning "why" an algorithm exists is really learning which structural shortcut it's exploiting.
