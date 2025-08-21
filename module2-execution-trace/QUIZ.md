# Module 2 Quiz: The Language of Computation - From Execution Trace to Polynomial Constraints

**Quiz Time:** 35 minutes  
**Total Score:** 100 points  
**Passing Score:** 70 points

---

## 📝 Part 1: Multiple Choice Questions (8 points each, 40 points total)

### 1. What is the basic structure of an Execution Trace?
A. One-dimensional array, each element represents a computation step
B. Two-dimensional table, rows represent time steps, columns represent variables/registers
C. Tree structure, each node represents an operation
D. Graph structure, edges represent data dependencies

### 2. What do Transition Constraints in AIR primarily describe?
A. Relationships between variables within each row
B. State change relationships between adjacent rows
C. Special conditions for first and last rows
D. Data type constraints between different columns

### 3. In the Fibonacci execution trace, if the current row is `[a=2, b=3, c=5]`, what should the next row be?
A. `[a=3, b=5, c=8]`
B. `[a=5, b=8, c=13]`
C. `[a=1, b=2, c=3]`
D. `[a=2, b=3, c=5]`

### 4. What is the core innovation of Plonky2's hybrid model?
A. Completely replace PLONK with AIR
B. Use PLONK's underlying architecture to implement AIR-style constraints
C. Directly combine AIR and STARK
D. Create entirely new arithmetization methods

### 5. Compared to PLONK constraints, what is the main characteristic of AIR constraints?
A. More complex but more flexible
B. Simpler but more restrictive
C. Higher degree of structure, suitable for repetitive computations
D. Completely equivalent, just different expressions

---

## ✍️ Part 2: Short Answer Questions (12 points each, 36 points total)

### 6. Compare the arithmetization thinking differences between PLONK and AIR
Please compare from the following three perspectives:
a) Thinking mode (circuit graph vs time series)
b) Constraint source (gate logic vs state transition)
c) Application scenarios (general computation vs repetitive computation)

### 7. Explain the role and importance of boundary constraints in AIR
Please explain:
a) What are boundary constraints
b) Why are boundary constraints needed  
c) Give an example of boundary constraints for Fibonacci sequence

### 8. Analyze the advantages of Plonky2's hybrid model
Please explain how Plonky2 uses copy constraints to implement transition constraints, and the benefits this design brings.

---

## 🧠 Part 3: Application Questions (24 points)

### 9. Design Execution Trace (12 points)
Design a complete execution trace for computing `x^8` (using repeated squaring: x → x² → x⁴ → x⁸), including:
a) Trace table structure
b) Transition constraint definitions
c) Boundary constraint settings

### 10. Hybrid Model Implementation (12 points)
Explain how to implement the `x^8` computation circuit from the previous question in Plonky2:
a) Main steps of CircuitBuilder
b) How to use copy constraints to connect each step
c) Design of public inputs and private witnesses

---

# 📊 Quiz Solutions

## Part 1: Multiple Choice Answers

### 1. Answer: B
**Explanation:** Execution trace is a two-dimensional table structure where each row represents a time step (state) in the computation process, and each column represents a variable, register, or computation result. This structure clearly shows the temporality and state evolution of computation.

### 2. Answer: B
**Explanation:** Transition constraints describe state change relationships between adjacent rows (time steps), i.e., `state[i+1] = f(state[i])`. They ensure each computation step follows correct state transition rules.

### 3. Answer: A
**Explanation:** According to Fibonacci transition rules:
- `a[i+1] = b[i]` → `a = 3`
- `b[i+1] = c[i]` → `b = 5` 
- `c[i+1] = a[i+1] + b[i+1]` → `c = 3 + 5 = 8`

### 4. Answer: B  
**Explanation:** Plonky2's innovation lies in maintaining PLONK's underlying architecture (gate constraints and copy constraints) while using this mechanism to efficiently implement AIR-style structured constraints, combining advantages of both.

### 5. Answer: C
**Explanation:** AIR constraints have highly structured characteristics, usually only involving relationships between adjacent rows, making them particularly suitable for describing repetitive, regular computation processes like virtual machine execution.

---

## Part 2: Short Answer Solutions

### 6. PLONK and AIR Arithmetization Thinking Differences (12 points)

**a) Thinking Mode (4 points)**
- **PLONK**: Circuit graph thinking, represents computation as connection graphs of gates and wires, focuses on logical relationships
- **AIR**: Time series thinking, represents computation as state evolution process on timeline

**b) Constraint Source (4 points)**  
- **PLONK**: Constraints come from gate logical relationships, each gate defines mathematical relationships between inputs and outputs
- **AIR**: Constraints come from state transition rules, defining how system changes from one state to the next

**c) Application Scenarios (4 points)**
- **PLONK**: General computation, particularly suitable for scenarios requiring complex arbitrary connections
- **AIR**: Repetitive/regular computation, particularly suitable for structured scenarios like VM execution, cryptographic operations

### 7. Role of Boundary Constraints in AIR (12 points)

**a) What are Boundary Constraints (4 points)**
Boundary constraints are constraint conditions on specific rows in the execution trace (usually first or last row), defining the initial state and/or final state of computation.

**b) Why Boundary Constraints are Needed (4 points)**
- **Initialization**: Ensure computation starts from correct initial state
- **Result Verification**: Ensure computation achieves expected final result  
- **Completeness**: Work with transition constraints to completely define entire computation process

**c) Fibonacci Sequence Boundary Constraint Example (4 points)**
```
Initial boundary constraints:
- a[0] = 0  (F(0) = 0)
- b[0] = 1  (F(1) = 1)

Terminal boundary constraints (optional):
- c[n] = expected_result  (verify nth Fibonacci number)
```

### 8. Advantages of Plonky2 Hybrid Model (12 points)

**Mechanism of Copy Constraints Implementing Transition Constraints (6 points)**
- Plonky2 maps each row of execution trace to multiple PLONK variables
- Uses copy constraint `connect(trace[i].output, trace[i+1].input)` to implement state passing
- Converts AIR transition constraints into PLONKY copy constraints

**Benefits Brought (6 points)**
1. **Maintain Flexibility**: Can still use PLONK's arbitrary gate constraints
2. **Structured Efficiency**: AIR-level efficiency for repetitive computations
3. **Developer Friendly**: Can mix both constraint styles  
4. **Recursion Friendly**: Copy constraints are easier to handle in recursive verification

---

## Part 3: Application Solutions

### 9. Design x^8 Execution Trace (12 points)

**a) Trace Table Structure (4 points)**
```
| Step | base | result |
|------|------|--------|
|  0   |  x   |   x    |
|  1   |  x   |   x²   |
|  2   |  x   |   x⁴   |
|  3   |  x   |   x⁸   |
```

**b) Transition Constraint Definitions (4 points)**
```
For each row i (i = 0, 1, 2):
result[i+1] = result[i] × result[i]
base[i+1] = base[i]  (base remains unchanged)
```

**c) Boundary Constraint Settings (4 points)**
```
Initial constraints:
- base[0] = x
- result[0] = x

Terminal constraints:
- result[3] = x^8
```

### 10. Plonky2 Hybrid Model Implementation (12 points)

**a) CircuitBuilder Main Steps (4 points)**
```rust
let mut builder = CircuitBuilder::new(config);

// Define input
let x = builder.add_virtual_target();

// Compute each power
let x_squared = builder.mul(x, x);           // x²
let x_fourth = builder.mul(x_squared, x_squared);   // x⁴  
let x_eighth = builder.mul(x_fourth, x_fourth);     // x⁸
```

**b) Copy Constraint Connections (4 points)**
In this specific example, since each step is a squaring operation, copy constraints are implicit in the `mul` operations:
- Both inputs of `x_squared` connect to `x`
- Both inputs of `x_fourth` connect to `x_squared`  
- Both inputs of `x_eighth` connect to `x_fourth`

**c) Public Input and Private Witness Design (4 points)**
```rust
// Public inputs
builder.register_public_input(x);        // Input value x
builder.register_public_input(x_eighth); // Result x^8

// Private witness
let mut pw = PartialWitness::new();
pw.set_target(x, F::from_canonical_u64(input_value));

// Intermediate values x², x⁴ are private, no external provision needed
```

---

## 🎯 Grading Scale

- **90-100 points:** Excellent - Deep understanding of execution trace and AIR concepts, flexible application
- **80-89 points:** Good - Good grasp of basic concepts with design capability
- **70-79 points:** Pass - Basic understanding of main concepts  
- **60-69 points:** Fail - Need to re-learn AIR related content
- **Below 60 points:** Fail - Recommend complete re-study of this module

## 📚 Review Recommendations

If scores are not ideal, focus on reviewing:
1. **Two-dimensional table structure and meaning of execution trace**
2. **Definition and role of transition and boundary constraints**
3. **Fundamental differences in thinking modes between PLONK and AIR**
4. **How Plonky2's hybrid model combines advantages of both methods**
5. **Practical trace design and constraint definition practice**
