# Module 1: Foundation Moment - Plonky2's Design Origins
## The Foundation - Plonky2's Design Origins

**Course Objective:** Review PLONK's core ideas to pave the way for understanding Plonky2's evolution and differentiated design.

**Mental Model:** Before studying an F1 race car, make sure we have a clear understanding of a standard family car's engine and transmission system.

---

## 1. PLONK Background Review

### 1.1 Core Constraint System

PLONK's charm lies in its **Universal Gate Constraints**:

```
q_L · w_a + q_R · w_b + q_O · w_c + q_M · w_a · w_b + q_C = 0
```

This seemingly simple equation embodies a "configurable logic unit":

#### Addition Gate
```
q_L = 1, q_R = 1, q_O = -1, q_M = 0, q_C = 0
=> w_a + w_b - w_c = 0
=> w_c = w_a + w_b
```

#### Multiplication Gate  
```
q_L = 0, q_R = 0, q_O = -1, q_M = 1, q_C = 0
=> w_a · w_b - w_c = 0
=> w_c = w_a · w_b
```

#### Constant Gate
```
q_L = 0, q_R = 0, q_O = -1, q_M = 0, q_C = k
=> -w_c + k = 0
=> w_c = k
```

### 1.2 Permutation Arguments: Universal "Glue"

**Core Concept:** Permutation Arguments are PLONK's innovative core, solving a fundamental problem: **How to prove that values at different positions are equal?**

#### Problem Scenario
Suppose we have a circuit:
- Row 1: `w_c = w_a + w_b` 
- Row 2: `w_c = w_a × w_b`
- We need `w_c` from row 1 to equal `w_a` from row 2

#### PLONK's Solution: Copy Constraints

PLONK uses permutation arguments to encode "wire connections":

```
σ(i,j) = (k,l) means the value at position (i,j) equals the value at position (k,l)
```

**Mathematical Principle:** If two sets A and B are equal, then their polynomial interpolations are also equal:
```
∏(x + βσ(i) + γ) = ∏(x + βπ(i) + γ)
```

### 1.3 PLONK's Architectural Features: Circuit-Centric

**Design Philosophy:** PLONK is a "circuit-centric" system.

- **Advantage:** Extremely flexible, like a free circuit board where any wire can connect anywhere
- **Cost:** For structured computations (like VM execution), this flexibility might be over-engineering

**Graphical Understanding:**
```
Gate 1: [w_a] + [w_b] = [w_c]
         |       |       |
         |       |    Copy Constraint
         |       |       ↓
Gate 2: [w_a] × [w_b] = [w_c]
```

---

## 2. Why Do We Need Plonky2?

### 2.1 PLONK's Limitations

1. **Trusted Setup Dependency:** KZG commitments require trusted setup, posing centralization risks
2. **Recursion Efficiency:** Elliptic curve pairing operations are costly to simulate in circuits
3. **Structured Computation:** For repetitive, regular computations, PLONK's flexibility becomes a burden

### 2.2 Plonky2's Design Goals

1. **Transparency:** Completely eliminate trusted setup
2. **Efficient Recursion:** Make verifier simulation in circuits extremely cheap
3. **Hybrid Architecture:** Maintain PLONK's flexibility while supporting AIR-style structured constraints

---

## 3. Core Concept Comparison

### 3.1 Trust Model

| Feature | PLONK | Plonky2 |
|---------|-------|---------|
| Trusted Setup | Required | Not Required |
| Commitment Scheme | KZG | FRI |
| Transparency | Partial | Complete |

### 3.2 Efficiency Comparison

| Feature | PLONK | Plonky2 |
|---------|-------|---------|
| Proof Size | Small (~400B) | Medium (~45KB) |
| Verification Time | Fast | Fast |
| Recursion Efficiency | Low | Extremely High |
| Generation Speed | Medium | Fast |

---

## 4. Practical Exercises

### Exercise 1: PLONK Gate Constraint Design

Design a gate constraint to implement: `w_c = w_a² + 2w_b + 3`

<details>
<summary>Solution</summary>

Two gates needed:
1. Multiplication gate: `w_d = w_a × w_a`
2. Linear combination gate: `w_d + 2w_b + 3 - w_c = 0`

Parameter settings:
- Gate 1: `q_M = 1, q_O = -1, others = 0`
- Gate 2: `q_L = 1, q_R = 2, q_O = -1, q_C = 3`

</details>

### Exercise 2: Copy Constraint Scenarios

In a 3-row circuit:
- Row 1: `a + b = c`
- Row 2: `c × d = e` 
- Row 3: `e + f = g`

Design copy constraints to connect these computations.

<details>
<summary>Solution</summary>

Copy constraints:
- `σ(1,c) = (2,c)` // Connect c from row 1 to c in row 2
- `σ(2,e) = (3,e)` // Connect e from row 2 to e in row 3

</details>

---

## 5. Deep Thinking

### Thinking Question 1
Why is PLONK's flexibility potentially "over-engineering" for applications like zkEVM?

### Thinking Question 2  
If we were to design a zero-knowledge proof system specifically for matrix multiplication, which features of PLONK would be necessary and which would be redundant?

---

## 6. Next Module Preview

In the next module, we will explore:
- How **Execution Trace** changes the mindset of arithmetization
- How **AIR (Algebraic Intermediate Representation)** more naturally expresses structured computations
- How **Plonky2's hybrid model** combines the best of both worlds

---

**Key Takeaways:**
1. PLONK provides ultimate flexibility through its universal gate constraints and permutation arguments
2. This flexibility comes at a cost: trusted setup, recursion efficiency, and complexity for structured computations
3. Plonky2 was born to solve these fundamental problems while maintaining core advantages
4. Understanding PLONK is a necessary foundation for grasping Plonky2's innovations
