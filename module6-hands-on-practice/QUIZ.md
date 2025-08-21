# Module 6 Quiz: Hands-On Practice - Fibonacci Classic Example & API

**Quiz Time:** 40 minutes  
**Total Score:** 100 points  
**Passing Score:** 70 points

---

## 📝 Part 1: Multiple Choice Questions (8 points each, 32 points total)

### 1. What is the main role of CircuitBuilder in Plonky2?
A. Compile Rust code
B. Tool for defining circuit constraints and variables
C. Generate zero-knowledge proofs
D. Verify proof correctness

### 2. What does the `add_virtual_target()` method do?
A. Add a constant value
B. Add a variable (wire) in the circuit
C. Execute addition operation
D. Connect two variables

### 3. In the Fibonacci example, which values does `register_public_input()` register?
A. All intermediate computation results
B. Only the final Fibonacci number
C. F(0), F(1), and F(100)
D. The entire computation process

### 4. What is the role of PartialWitness in zero-knowledge proofs?
A. Define circuit structure
B. Provide assignment of private input values
C. Verify proof correctness
D. Generate public parameters

---

## ✍️ Part 2: Short Answer Questions (10 points each, 30 points total)

### 5. Explain the complete workflow of Plonky2 development
Please describe in order the 8 main steps from circuit design to proof verification.

### 6. Analyze constraint design in the Fibonacci example
Please explain:
a) How to implement the recurrence relation F(n) = F(n-1) + F(n-2)
b) The role of copy constraints in state transitions
c) The difference between public inputs and private witnesses

### 7. Compare costs of different constraint operations
Please analyze the constraint costs of the following operations in circuits:
a) Addition operation (`builder.add()`)
b) Multiplication operation (`builder.mul()`)  
c) Constant multiplication (`mul_const()`)
d) Copy constraint (`connect()`)

---

## 🧠 Part 3: Programming Application Questions (28 points)

### 8. Circuit Design Exercise (14 points)
Please design a circuit to prove knowledge of a secret number x satisfying:
- x² + 3x + 2 = 0
- x is an integer

Requirements:
a) Design circuit structure and constraints
b) Write key CircuitBuilder code
c) Design witness assignment strategy

### 9. Performance Optimization Exercise (14 points)
Original code (inefficient):
```rust
let mut result = builder.zero();
for i in 0..1000 {
    let temp = builder.mul(x, x);
    result = builder.add(result, temp);
}
```

Please:
a) Identify performance issues
b) Provide optimized version
c) Estimate constraint count improvement

---

## 💡 Part 4: Debugging & Analysis Questions (10 points)

### 10. Error Analysis & Fix
What's wrong with the following code? Please analyze and provide a fix:

```rust
fn broken_circuit() -> Result<()> {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    let x = builder.add_virtual_target();
    let y = builder.add_virtual_target();
    let z = builder.add(x, y);
    
    builder.register_public_input(x);
    builder.register_public_input(z);
    
    let data = builder.build::<C>();
    let proof = data.prove(pw)?;  // Compilation error here
    
    data.verify(proof)
}
```

---

# 📊 Quiz Solutions

## Part 1: Multiple Choice Answers

### 1. Answer: B
**Explanation:** CircuitBuilder is the core tool for defining circuit constraints and variables, providing methods like `add_virtual_target()`, `add()`, `mul()` to build circuit structure and constraint relationships.

### 2. Answer: B  
**Explanation:** `add_virtual_target()` adds a variable in the circuit (called Target in Plonky2 terminology), which can be used in subsequent constraints and assigned concrete values during proving.

### 3. Answer: C
**Explanation:** In the Fibonacci example, public inputs include initial values F(0)=0, F(1)=1, and the final result F(100). Intermediate computation steps are private and not visible to external verifiers.

### 4. Answer: B
**Explanation:** PartialWitness is responsible for providing concrete value assignments to circuit variables. These values are typically private, known only to the prover, and are key inputs for generating zero-knowledge proofs.

---

## Part 2: Short Answer Solutions

### 5. Complete Plonky2 Development Workflow (10 points)

**Complete 8 steps (1.25 points each):**

1. **Circuit Configuration**: Create `CircuitConfig` and `CircuitBuilder`
2. **Define Variables**: Use `add_virtual_target()` to add circuit variables
3. **Build Constraints**: Use `add()`, `mul()` etc. to define computation logic
4. **Register Public Inputs**: Use `register_public_input()` to specify verifiable values
5. **Compile Circuit**: Call `build()` to generate `CircuitData`
6. **Prepare Witness**: Create `PartialWitness` and use `set_target()` to assign values
7. **Generate Proof**: Call `data.prove(pw)` to generate zero-knowledge proof
8. **Verify Proof**: Call `data.verify(proof)` to verify proof correctness

### 6. Fibonacci Constraint Design Analysis (10 points)

**a) Recurrence Relation Implementation (3.5 points)**
```rust
// In each iteration:
let next_target = builder.add(prev_target, curr_target);
// This creates constraint: next = prev + curr

// State update:
prev_target = curr_target;
curr_target = next_target;
```

**b) Role of Copy Constraints (3.5 points)**
- Copy constraints are implicitly implemented in variable assignments for state transitions
- `prev_target = curr_target` ensures next step's prev equals current step's curr
- `curr_target = next_target` ensures next step's curr equals current step's next
- These constraints guarantee continuity of Fibonacci sequence

**c) Public Inputs vs Private Witnesses (3 points)**
```
Public Inputs: F(0)=0, F(1)=1, F(100)=result
- Anyone can see and verify
- Used to confirm computation start and end points

Private Witnesses: Intermediate F(2), F(3), ..., F(99)
- Only prover knows
- Achieves zero-knowledge property: proves computation correctness without revealing process
```

### 7. Constraint Operation Cost Analysis (10 points)

**a) Addition Operation (2.5 points)**
```rust
builder.add(a, b)  // Cost: 1 constraint
```
- Creates linear constraint: `a + b - result = 0`
- Lowest cost basic operation

**b) Multiplication Operation (2.5 points)**  
```rust
builder.mul(a, b)  // Cost: 1 constraint
```
- Creates quadratic constraint: `a × b - result = 0`
- Slightly more complex than addition but still efficient

**c) Constant Multiplication (2.5 points)**
```rust
mul_const(target, constant)  // Cost: 1 constraint
```
- Optimized constant multiplication, avoids general multiplication gates
- Same cost as variable multiplication but simpler implementation

**d) Copy Constraint (2.5 points)**
```rust
builder.connect(a, b)  // Cost: 0 additional constraints
```
- Implemented through permutation arguments, doesn't increase constraint count
- Most efficient constraint type

---

## Part 3: Programming Application Solutions

### 8. Circuit Design Exercise (14 points)

**a) Circuit Structure Design (5 points)**
```
Goal: Prove x² + 3x + 2 = 0, where x is integer

Analysis: x² + 3x + 2 = (x+1)(x+2) = 0
So x = -1 or x = -2

Circuit constraints:
1. y = x²
2. z = 3x  
3. result = y + z + 2
4. result = 0
```

**b) CircuitBuilder Code (5 points)**
```rust
fn quadratic_proof() -> Result<()> {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    // Define secret input x
    let x = builder.add_virtual_target();
    
    // Calculate x²
    let x_squared = builder.mul(x, x);
    
    // Calculate 3x
    let three = builder.constant(F::from_canonical_u64(3));
    let three_x = builder.mul(three, x);
    
    // Calculate x² + 3x
    let partial_sum = builder.add(x_squared, three_x);
    
    // Calculate x² + 3x + 2
    let two = builder.constant(F::from_canonical_u64(2));
    let result = builder.add(partial_sum, two);
    
    // Constraint result to be 0
    let zero = builder.zero();
    builder.connect(result, zero);
    
    // Public input: only result (should be 0)
    builder.register_public_input(result);
    
    let data = builder.build::<C>();
    // ... witness and proof generation
}
```

**c) Witness Assignment Strategy (4 points)**
```rust
// Witness assignment
let mut pw = PartialWitness::new();

// Choose x = -1 (represented in Goldilocks field)
let x_value = F::from_canonical_u64(p - 1); // -1 mod p
pw.set_target(x, x_value)?;

// Verification: (-1)² + 3(-1) + 2 = 1 - 3 + 2 = 0 ✓
```

### 9. Performance Optimization Exercise (14 points)

**a) Performance Issue Identification (5 points)**
```
Issue 1: Repeated computation of x²
- Computing same x² 1000 times in loop
- Creating new constraint each time

Issue 2: Constraint count explosion  
- Created 1000 multiplication constraints
- Created 1000 addition constraints
- Total: 2000 constraints

Issue 3: Not utilizing constant optimization
- Could precompute 1000 × x²
```

**b) Optimized Version (5 points)**
```rust
// Optimized version
let x_squared = builder.mul(x, x);           // 1 constraint
let thousand = builder.constant(F::from_canonical_u64(1000));
let result = builder.mul(thousand, x_squared); // 1 constraint

// Or further optimization
let result = mul_const(&mut builder, x_squared, 1000); // 1 constraint
```

**c) Constraint Count Improvement (4 points)**
```
Original version:
- Multiplication constraints: 1000
- Addition constraints: 1000  
- Total: 2000 constraints

Optimized version:
- Multiplication constraints: 2 (x² and 1000×x²)
- Addition constraints: 0
- Total: 2 constraints

Improvement ratio: 2000 → 2, reduced by 99.9%
```

---

## Part 4: Debugging & Analysis Solutions

### 10. Error Analysis & Fix (10 points)

**Error Analysis (5 points)**
```
Main issues:
1. PartialWitness pw used without being defined
2. No concrete values set for variables x, y
3. Variable y defined but unused, may cause incomplete constraints

Compilation error reason:
- pw variable undeclared
- Compiler cannot infer pw type
```

**Fix Solution (5 points)**
```rust
fn fixed_circuit() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    let x = builder.add_virtual_target();
    let y = builder.add_virtual_target();
    let z = builder.add(x, y);
    
    builder.register_public_input(x);
    builder.register_public_input(y);  // Add y as public input
    builder.register_public_input(z);
    
    let data = builder.build::<C>();
    
    // Correctly create and set PartialWitness
    let mut pw = PartialWitness::new();
    pw.set_target(x, F::from_canonical_u64(10))?;
    pw.set_target(y, F::from_canonical_u64(20))?;
    
    let proof = data.prove(pw)?;
    
    data.verify(proof)
}
```

---

## 🎯 Grading Scale

- **90-100 points:** Excellent - Proficient mastery of Plonky2 API, capable of independent circuit design and optimization
- **80-89 points:** Good - Good understanding of API usage, capable of basic circuit design
- **70-79 points:** Pass - Basic mastery of main APIs and development workflow
- **60-69 points:** Fail - Need to re-learn API usage methods
- **Below 60 points:** Fail - Recommend complete re-study of this module and hands-on practice

## 📚 Review Recommendations

If scores are not ideal, recommend:
1. **Re-run fibonacci_course.rs**, understand every line of code
2. **Master main APIs**: CircuitBuilder, Target, PartialWitness
3. **Practice circuit design**: Start with simple examples, gradually increase complexity
4. **Learn constraint optimization**: Understand cost differences of different operations
5. **Master debugging techniques**: Able to identify and fix common errors
