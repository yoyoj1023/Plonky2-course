# Module 2: The Language of Computation - From Execution Trace to Polynomial Constraints
## The Language of Computation - From Execution Trace to Polynomial Constraints

**Course Objective:** Master Plonky2's (and STARKs') arithmetization approach and deeply compare its differences from standard PLONK.

**Mental Model:** If standard PLONK is freely designing circuits graphically, then AIR is like using Excel spreadsheets to describe a step-by-step computational process.

---

## 1. Execution Trace

### 1.1 Core Concept

**Execution trace** represents a computational process as a 2D table:
- **Each row** represents a time step (state) of computation
- **Each column** represents a register or variable

### 1.2 Fibonacci Example

Let's use the Fibonacci sequence to understand concretely:

```
F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2)
```

**Execution Trace Table:**

| Step | a | b | c |
|------|---|---|---|
| 0    | 0 | 1 | 1 |
| 1    | 1 | 1 | 2 |
| 2    | 1 | 2 | 3 |
| 3    | 2 | 3 | 5 |
| 4    | 3 | 5 | 8 |
| ... | ... | ... | ... |

**Meaning Explanation:**
- `a`: Previous-previous Fibonacci number
- `b`: Previous Fibonacci number  
- `c`: Current Fibonacci number
- Each row satisfies: `c = a + b`

### 1.3 Comparison with Traditional Circuits

**Traditional Circuit Thinking:**
```
[a] ——————————\
              ADD —— [c]
[b] ——————————/
```

**Execution Trace Thinking:**
```
State sequence on timeline:
State 0: {a=0, b=1, c=1}
State 1: {a=1, b=1, c=2}  
State 2: {a=1, b=2, c=3}
...
```

---

## 2. AIR (Algebraic Intermediate Representation)

### 2.1 What is AIR?

AIR is an **execution trace-based arithmetization method** that describes using polynomial constraints:
1. Relationships within each row
2. Relationships between adjacent rows
3. Constraints at special positions (start/end)

### 2.2 Types of Polynomial Constraints

#### A. Transition Constraints

**Definition:** Relationship constraints between adjacent rows

**Fibonacci Transition Constraints:**
```
// For each row i (except the last row)
c[i] = a[i] + b[i]           // Internal constraint
a[i+1] = b[i]                // Transition constraint 1  
b[i+1] = c[i]                // Transition constraint 2
```

**Polynomial Form:**
```
// Let row_i = (a[i], b[i], c[i])
// Let row_{i+1} = (a[i+1], b[i+1], c[i+1])

Constraint 1: c[i] - a[i] - b[i] = 0
Constraint 2: a[i+1] - b[i] = 0  
Constraint 3: b[i+1] - c[i] = 0
```

#### B. Boundary Constraints

**Definition:** Constraints on specific rows (usually first or last row)

**Fibonacci Boundary Constraints:**
```
// Initial state
a[0] = 0
b[0] = 1

// If verifying specific result
c[n] = expected_result
```

### 2.3 Complete AIR Definition

For Fibonacci sequence, complete AIR contains:

```rust
// Boundary constraints
fn boundary_constraints(trace: &Trace) -> Vec<Constraint> {
    vec![
        trace[0][0] - F::ZERO,      // a[0] = 0
        trace[0][1] - F::ONE,       // b[0] = 1  
    ]
}

// Transition constraints
fn transition_constraints(current: &[F], next: &[F]) -> Vec<F> {
    let (a_curr, b_curr, c_curr) = (current[0], current[1], current[2]);
    let (a_next, b_next, c_next) = (next[0], next[1], next[2]);
    
    vec![
        c_curr - a_curr - b_curr,   // c = a + b
        a_next - b_curr,            // a[i+1] = b[i]
        b_next - c_curr,            // b[i+1] = c[i]
    ]
}
```

---

## 3. The Great Arithmetization Comparison

### 3.1 Design Philosophy Differences

| Feature | PLONK | AIR |
|---------|-------|-----|
| **Thinking Mode** | Circuit Graph | Time Series |
| **Constraint Source** | Gate Logic Relations | State Transition Rules |
| **Flexibility** | Extremely High (arbitrary connections) | Medium (structured) |
| **Application Scenarios** | General computation | Repetitive/regular computation |

### 3.2 Constraint Expression Methods

#### PLONK Constraint Example
```rust
// Each gate is an independent constraint
gate1: q_L·w_a + q_R·w_b + q_O·w_c + q_M·w_a·w_b + q_C = 0
gate2: q_L·w_d + q_R·w_e + q_O·w_f + q_M·w_d·w_e + q_C = 0

// Connected through copy constraints
copy_constraint: w_c = w_d
```

#### AIR Constraint Example  
```rust
// Transition constraints directly express state changes
transition: next_state = f(current_state)
// Boundary constraints define initial/final states  
boundary: initial_state = init_values
```

### 3.3 Efficiency Comparison

| Aspect | PLONK | AIR |
|--------|-------|-----|
| **Setup Complexity** | High (need to design gates and connections) | Low (directly describe state transitions) |
| **Number of Constraints** | Many (one constraint per gate) | Few (one constraint per transition type) |
| **Repetitive Computation Efficiency** | Low | High |
| **Parallelization Friendliness** | Medium | High |

---

## 4. Plonky2's Hybrid Model

### 4.1 Core Innovation: Implementing AIR with PLONK

Plonky2's genius lies in: **Using PLONK's underlying architecture to efficiently implement AIR-style constraints**.

### 4.2 Implementation Strategy

#### A. State Representation
```rust
// In Plonky2, each row of execution trace corresponds to multiple PLONK variables
struct FibonacciRow {
    a: Target,      // PLONK target
    b: Target,      // PLONK target  
    c: Target,      // PLONK target
}
```

#### B. Transition Constraint Implementation
```rust
// Using PLONK gates to implement AIR transition constraints
impl CircuitBuilder {
    fn add_fibonacci_transition(&mut self, 
                               current: &FibonacciRow, 
                               next: &FibonacciRow) {
        // Internal constraint: c = a + b
        let sum = self.add(current.a, current.b);
        self.connect(sum, current.c);
        
        // Transition constraints: use copy constraints for state passing
        self.connect(current.b, next.a);  // a[i+1] = b[i]
        self.connect(current.c, next.b);  // b[i+1] = c[i]
    }
}
```

#### C. Clever Use of Copy Constraints

**Key Insight:** Plonky2 uses PLONK's copy constraints to efficiently express "the value in row i+1 equals the value in row i", perfectly simulating transition constraints.

```rust
// Traditional AIR needs special transition constraint mechanisms
// Plonky2 converts it to PLONK copy constraints
self.connect(trace[i].output, trace[i+1].input);
```

### 4.3 Advantages of Hybrid Model

1. **Maintain Flexibility**: Can still use PLONK's arbitrary gate constraints
2. **Structured Efficiency**: AIR-level efficiency for repetitive computations  
3. **Developer Friendly**: Can mix both constraint styles
4. **Recursion Friendly**: Copy constraints are easier to handle in recursive verification

---

## 5. Practical Exercises

### Exercise 1: Design Execution Trace

Design an execution trace for the following computation: calculate `x^4` (using repeated squaring method)

<details>
<summary>Solution</summary>

```
| Step | x | result |
|------|---|--------|
| 0    | x | x      |
| 1    | x | x²     |
| 2    | x | x⁴     |

Transition constraint:
result[i+1] = result[i] × result[i]

Boundary constraint:
result[0] = x
```

</details>

### Exercise 2: AIR Constraint Design

Design AIR for a counter circuit: increment counter by 1 each step, counting from 0 to n.

<details>
<summary>Solution</summary>

```rust
// Execution trace
| Step | counter |
|------|---------|
| 0    | 0       |
| 1    | 1       |
| 2    | 2       |
| ...  | ...     |
| n    | n       |

// Transition constraint
counter[i+1] = counter[i] + 1

// Boundary constraints  
counter[0] = 0
counter[n] = n
```

</details>

### Exercise 3: Hybrid Model Implementation

Think about how to implement the counter from Exercise 2 in Plonky2.

<details>
<summary>Solution</summary>

```rust
struct CounterRow {
    counter: Target,
}

impl CircuitBuilder {
    fn add_counter_step(&mut self, current: Target) -> Target {
        let one = self.one();
        let next = self.add(current, one);
        next
    }
    
    fn build_counter_circuit(&mut self, n: usize) -> Target {
        let mut current = self.zero(); // Initial value 0
        
        for _ in 0..n {
            current = self.add_counter_step(current);
        }
        
        current // Return final value
    }
}
```

</details>

---

## 6. Deep Thinking

### Thinking Question 1
Why is AIR more suitable for describing virtual machine (VM) execution processes?

### Thinking Question 2
In what situations would Plonky2's hybrid model have advantages over pure AIR or pure PLONK?

### Thinking Question 3  
If proving the correctness of a sorting algorithm, would you choose PLONK-style or AIR-style constraints? Why?

---

## 7. Next Module Preview

In the next module, we will explore:
- How **FRI commitment scheme** replaces KZG to achieve transparency
- The mathematical beauty of **commit-fold-repeat**
- Why FRI is more suitable for Plonky2's recursion goals

---

**Key Takeaways:**
1. **Execution trace** provides a time series perspective for describing computation
2. **AIR** naturally expresses structured computation through transition and boundary constraints
3. **Plonky2's hybrid model** cleverly uses PLONK mechanisms to achieve AIR efficiency
4. Different arithmetization methods are suitable for different types of computational tasks
5. Understanding these differences is key to choosing the right tools
