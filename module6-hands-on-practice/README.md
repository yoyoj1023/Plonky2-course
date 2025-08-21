# Module 6: Hands-On Practice - The Fibonacci Example & API
## Hands-On Practice - The Fibonacci Example & API

**Course Objective:** Apply all theoretical knowledge to actual code, solidifying understanding.

**Mental Model:** After reading all automotive engineering manuals, personally assemble a Fibonacci engine.

---

## 1. Plonky2 API Overview

### 1.1 Core Component Architecture

```rust
// Main type definitions
use plonky2::field::types::Field;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData};
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::iop::target::Target;

// Common type aliases
const D: usize = 2; // Extension degree
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F; // GoldilocksField
```

### 1.2 Key API Components

#### A. CircuitBuilder - Circuit Builder
```rust
impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilder<F, D> {
    /// Create new circuit builder
    pub fn new(config: CircuitConfig) -> Self;
    
    /// Add virtual variable (circuit input)
    pub fn add_virtual_target(&mut self) -> Target;
    
    /// Add constant
    pub fn constant(&mut self, value: F) -> Target;
    
    /// Basic operation gates
    pub fn add(&mut self, a: Target, b: Target) -> Target;
    pub fn mul(&mut self, a: Target, b: Target) -> Target;
    pub fn sub(&mut self, a: Target, b: Target) -> Target;
    
    /// Constrain equality
    pub fn connect(&mut self, a: Target, b: Target);
    
    /// Register public input
    pub fn register_public_input(&mut self, target: Target);
    
    /// Build final circuit
    pub fn build<C: GenericConfig<D, F>>(self) -> CircuitData<F, C, D>;
}
```

#### B. PartialWitness - Witness Assignment
```rust
impl<F: Field> PartialWitness<F> {
    /// Create new witness
    pub fn new() -> Self;
    
    /// Assign value to variable
    pub fn set_target(&mut self, target: Target, value: F) -> Result<()>;
    
    /// Assign values to multiple variables
    pub fn set_targets(&mut self, targets: &[Target], values: &[F]) -> Result<()>;
}
```

#### C. CircuitData - Circuit Data
```rust
impl<F: RichField + Extendable<D>, C: GenericConfig<D, F>, const D: usize> CircuitData<F, C, D> {
    /// Generate proof
    pub fn prove(&self, pw: PartialWitness<F>) -> Result<ProofWithPublicInputs<F, C, D>>;
    
    /// Verify proof
    pub fn verify(&self, proof: ProofWithPublicInputs<F, C, D>) -> Result<()>;
}
```

---

## 2. Fibonacci Classic Example Detailed

### 2.1 Problem Definition

**Goal:** Prove that we know the nth Fibonacci number and that the computation process is correct.

**Fibonacci sequence definition:**
```
F(0) = 0
F(1) = 1  
F(n) = F(n-1) + F(n-2) for n ≥ 2
```

### 2.2 Arithmetization Strategy

We will use the **execution trace** approach to represent Fibonacci computation:

```
| Step | prev | curr | next |
|------|------|------|------|
|  0   |  0   |  1   |  1   |
|  1   |  1   |  1   |  2   |
|  2   |  1   |  2   |  3   |
|  3   |  2   |  3   |  5   |
|  4   |  3   |  5   |  8   |
| ...  | ...  | ...  | ...  |
```

**Constraints:**
1. `next = prev + curr` (transition constraint for each step)
2. `prev[i+1] = curr[i]` (state transfer constraint)
3. `curr[i+1] = next[i]` (state transfer constraint)

### 2.3 Complete Implementation

```rust
use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

fn fibonacci_proof() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    // 1. Configuration and initialization
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // 2. Define circuit variables
    let initial_a = builder.add_virtual_target(); // F(0) = 0
    let initial_b = builder.add_virtual_target(); // F(1) = 1
    
    // 3. Build Fibonacci computation chain
    let mut prev_target = initial_a;
    let mut curr_target = initial_b;
    
    for i in 0..99 { // Calculate to F(100)
        // next = prev + curr
        let next_target = builder.add(prev_target, curr_target);
        
        // Update state: prepare for next round
        prev_target = curr_target;
        curr_target = next_target;
        
        // Optional: add intermediate values as private witness (for debugging)
        if i < 5 { // Only record first few values
            println!("Step {}: F({}) circuit construction complete", i + 2, i + 2);
        }
    }

    // 4. Define public inputs
    builder.register_public_input(initial_a);    // F(0)
    builder.register_public_input(initial_b);    // F(1)  
    builder.register_public_input(curr_target);  // F(100)

    // 5. Build circuit
    let data = builder.build::<C>();
    println!("Circuit built, total constraints: {}", data.common.degree());

    // 6. Prepare witness (private inputs)
    let mut pw = PartialWitness::new();
    pw.set_target(initial_a, F::ZERO)?;  // F(0) = 0
    pw.set_target(initial_b, F::ONE)?;   // F(1) = 1

    // 7. Generate proof
    println!("Starting proof generation...");
    let start_time = std::time::Instant::now();
    let proof = data.prove(pw)?;
    let prove_time = start_time.elapsed();
    
    println!("Proof generation complete, time: {:?}", prove_time);
    println!("Proof size: {} bytes", proof.to_bytes().len());

    // 8. Extract and display results
    println!("Public input verification:");
    println!("F(0) = {}", proof.public_inputs[0]);
    println!("F(1) = {}", proof.public_inputs[1]); 
    println!("F(100) = {}", proof.public_inputs[2]);

    // 9. Verify proof
    println!("Starting proof verification...");
    let start_time = std::time::Instant::now();
    data.verify(proof)?;
    let verify_time = start_time.elapsed();
    
    println!("Proof verification successful, time: {:?}", verify_time);

    Ok(())
}

fn main() -> Result<()> {
    fibonacci_proof()
}
```

### 2.4 Result Analysis

**Expected output:**
```
Circuit built, total constraints: 32768
Starting proof generation...
Proof generation complete, time: 1.2s
Proof size: 45032 bytes
Public input verification:
F(0) = 0
F(1) = 1
F(100) = 792070839848372253127 (mod 2^64-2^32+1)
Starting proof verification...
Proof verification successful, time: 12ms
```

**Key observations:**
1. **Constraint count**: ~32K, relatively small
2. **Proof size**: ~45KB, typical FRI size
3. **Generation time**: ~1.2 seconds, very fast
4. **Verification time**: ~12ms, extremely fast

---

## 3. Advanced API Usage

### 3.1 Advanced Constraint Operations

#### A. Conditional Constraints
```rust
/// Implement conditional logic: if condition then a else b
fn conditional_select(
    builder: &mut CircuitBuilder<F, D>,
    condition: Target,
    a: Target,
    b: Target,
) -> Target {
    // result = condition * a + (1 - condition) * b
    let not_condition = builder.sub(builder.one(), condition);
    let term1 = builder.mul(condition, a);
    let term2 = builder.mul(not_condition, b);
    builder.add(term1, term2)
}
```

#### B. Range Checks
```rust
/// Ensure a value is in range [0, 2^bits)
fn range_check(
    builder: &mut CircuitBuilder<F, D>,
    value: Target,
    bits: usize,
) {
    let mut sum = builder.zero();
    let mut power_of_two = builder.one();
    
    for _ in 0..bits {
        // Add a binary variable
        let bit = builder.add_virtual_target();
        
        // Constrain bit ∈ {0, 1}
        let bit_squared = builder.mul(bit, bit);
        builder.connect(bit, bit_squared);
        
        // Add to sum
        let term = builder.mul(bit, power_of_two);
        sum = builder.add(sum, term);
        
        // Update power of two
        power_of_two = builder.add(power_of_two, power_of_two);
    }
    
    // Constrain sum equals original value
    builder.connect(value, sum);
}
```

### 3.2 Optimization Techniques

#### A. Batch Operations
```rust
/// Efficiently compute sum of multiple values
fn sum_targets(
    builder: &mut CircuitBuilder<F, D>,
    targets: &[Target],
) -> Target {
    targets.iter().fold(builder.zero(), |acc, &target| {
        builder.add(acc, target)
    })
}

/// Efficiently compute product of multiple values
fn product_targets(
    builder: &mut CircuitBuilder<F, D>,
    targets: &[Target],
) -> Target {
    targets.iter().fold(builder.one(), |acc, &target| {
        builder.mul(acc, target)
    })
}
```

#### B. Constant Optimization
```rust
/// Fast multiplication by constant (avoid general multiplication gates)
fn mul_const(
    builder: &mut CircuitBuilder<F, D>,
    target: Target,
    constant: u64,
) -> Target {
    let const_target = builder.constant(F::from_canonical_u64(constant));
    builder.mul(target, const_target)
}

/// Fast power computation (using repeated squaring)
fn pow_const(
    builder: &mut CircuitBuilder<F, D>,
    base: Target,
    exponent: u64,
) -> Target {
    if exponent == 0 {
        return builder.one();
    }
    
    let mut result = builder.one();
    let mut base_power = base;
    let mut exp = exponent;
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = builder.mul(result, base_power);
        }
        base_power = builder.mul(base_power, base_power);
        exp >>= 1;
    }
    
    result
}
```

---

## 4. Practical Exercises

### Exercise 1: Extend Fibonacci Example

Modify the Fibonacci program to prove that the nth Fibonacci number is divisible by some value.

<details>
<summary>Solution Framework</summary>

```rust
fn fibonacci_divisibility_proof(n: usize, divisor: u64) -> Result<()> {
    // ... basic setup ...
    
    // Calculate F(n)
    let fib_n = build_fibonacci_circuit(&mut builder, n);
    
    // Add divisibility check
    let quotient = builder.add_virtual_target();
    let divisor_target = builder.constant(F::from_canonical_u64(divisor));
    let product = builder.mul(quotient, divisor_target);
    builder.connect(fib_n, product);
    
    // Make quotient a public input
    builder.register_public_input(quotient);
    
    // ... remaining implementation ...
}
```

</details>

### Exercise 2: Implement Counter Circuit

Create a circuit that proves the process of counting from 0 to n.

<details>
<summary>Solution Framework</summary>

```rust
fn counter_circuit(n: usize) -> Result<()> {
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    let mut current = builder.zero();
    let one = builder.one();
    
    for _ in 0..n {
        current = builder.add(current, one);
    }
    
    let expected = builder.constant(F::from_canonical_u64(n as u64));
    builder.connect(current, expected);
    
    // ... complete implementation ...
}
```

</details>

### Exercise 3: Polynomial Evaluation Circuit

Implement a circuit that proves polynomial evaluation at a point is correct.

<details>
<summary>Solution Framework</summary>

```rust
fn polynomial_evaluation(coeffs: &[u64], x_value: u64) -> Result<()> {
    let mut builder = CircuitBuilder::<F, D>::new(config);
    
    let x = builder.constant(F::from_canonical_u64(x_value));
    let mut result = builder.zero();
    let mut x_power = builder.one();
    
    for &coeff in coeffs {
        let coeff_target = builder.constant(F::from_canonical_u64(coeff));
        let term = builder.mul(coeff_target, x_power);
        result = builder.add(result, term);
        x_power = builder.mul(x_power, x);
    }
    
    builder.register_public_input(result);
    
    // ... complete implementation ...
}
```

</details>

---

## 5. Error Handling & Debugging

### 5.1 Common Errors

#### A. Unsatisfied Constraints
```rust
// Error example: forgetting to set witness value
let target = builder.add_virtual_target();
builder.register_public_input(target);
// pw.set_target(target, value); // forgot this line

// Solution: ensure all variables have values
let mut pw = PartialWitness::new();
pw.set_target(target, F::from_canonical_u64(42))?;
```

#### B. Type Mismatches
```rust
// Error example: using wrong configuration type
type WrongConfig = KeccakGoldilocksConfig; // doesn't match
let data = builder.build::<WrongConfig>(); // compilation error

// Solution: keep types consistent
type C = PoseidonGoldilocksConfig;
let data = builder.build::<C>();
```

### 5.2 Debugging Techniques

#### A. Intermediate Value Checking
```rust
fn debug_fibonacci() -> Result<()> {
    // ... basic setup ...
    
    let mut values = vec![F::ZERO, F::ONE];
    let mut prev = initial_a;
    let mut curr = initial_b;
    
    for i in 0..10 {
        let next = builder.add(prev, curr);
        
        // Calculate expected value (for debugging)
        let expected = values[i] + values[i + 1];
        values.push(expected);
        
        println!("F({}) expected value: {}", i + 2, expected);
        
        prev = curr;
        curr = next;
    }
    
    // ... remaining implementation ...
}
```

#### B. Circuit Statistics
```rust
fn circuit_statistics(data: &CircuitData<F, C, D>) {
    println!("Circuit statistics:");
    println!("  Total constraints: {}", data.common.degree());
    println!("  Public inputs: {}", data.common.num_public_inputs);
    println!("  Constraint polynomials: {}", data.common.num_constants);
    println!("  FRI config: {:?}", data.common.config.fri_config);
}
```

---

## 6. Performance Optimization Guide

### 6.1 Circuit Size Optimization

#### A. Reduce Constraint Count
```rust
// Inefficient: multiple uses of intermediate variables
let temp1 = builder.add(a, b);
let temp2 = builder.add(temp1, c);
let result = builder.add(temp2, d);

// Efficient: direct chained computation
let result = builder.add(builder.add(builder.add(a, b), c), d);
```

#### B. Reuse Computation Results
```rust
// Inefficient: repeated computation
let x_squared_1 = builder.mul(x, x);
let x_squared_2 = builder.mul(x, x); // repeated computation

// Efficient: reuse results
let x_squared = builder.mul(x, x);
// Use x_squared in all subsequent operations
```

### 6.2 Memory Optimization

#### A. Batch Witness Setting
```rust
// Inefficient: individual setting
for (target, value) in targets.iter().zip(values.iter()) {
    pw.set_target(*target, *value)?;
}

// Efficient: batch setting
pw.set_targets(&targets, &values)?;
```

#### B. Pre-allocate Space
```rust
// Efficient: pre-allocate sufficient space
let mut targets = Vec::with_capacity(n);
let mut values = Vec::with_capacity(n);
```

---

## 7. Real Project Recommendations

### 7.1 Project Structure

```
fibonacci_project/
├── Cargo.toml
├── src/
│   ├── lib.rs          // Core implementation
│   ├── fibonacci.rs    // Fibonacci circuit  
│   ├── utils.rs        // Utility functions
│   └── main.rs         // Main program
├── examples/           // Example programs
├── tests/             // Test files
└── benches/           // Performance tests
```

### 7.2 Cargo.toml Configuration

```toml
[package]
name = "fibonacci-plonky2"
version = "0.1.0"
edition = "2021"

[dependencies]
plonky2 = { path = "../plonky2/plonky2" }
anyhow = "1.0"

[dev-dependencies]
criterion = "0.4"

[[bench]]
name = "fibonacci_bench"
harness = false
```

---

## 8. Next Module Preview

In the final module, we will explore:
- **Plonky3's modular design**
- **Migration from Plonky2 to Plonky3**  
- **Future development directions of ZKP systems**

---

**Key Takeaways:**
1. **CircuitBuilder API** provides an intuitive circuit building interface
2. **PartialWitness** manages private input assignments
3. **Fibonacci example** demonstrates complete development workflow
4. **Performance optimization** needs consideration from circuit design stage
5. **Debugging techniques** are crucial for complex circuit development
