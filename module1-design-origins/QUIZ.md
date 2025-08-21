# Module 1 Quiz: Foundation Moment - Plonky2's Design Origins

**Quiz Time:** 30 minutes  
**Total Score:** 100 points  
**Passing Score:** 70 points

---

## 📝 Part 1: Multiple Choice Questions (10 points each, 40 points total)

### 1. What is PLONK's universal gate constraint equation?
A. `q_L · w_a + q_R · w_b + q_O · w_c = 0`  
B. `q_L · w_a + q_R · w_b + q_O · w_c + q_M · w_a · w_b + q_C = 0`  
C. `w_a + w_b = w_c`  
D. `q_L · w_a · w_b + q_R · w_c = 0`

### 2. What is the main role of Permutation Arguments in PLONK?
A. Accelerate polynomial operations  
B. Implement copy constraints to prove values at different positions are equal  
C. Reduce proof size  
D. Improve verification speed

### 3. Compared to PLONK, which of the following is NOT a main design goal of Plonky2?
A. Completely eliminate trusted setup  
B. Achieve efficient recursive verification  
C. Reduce proof generation time to milliseconds  
D. Support hybrid arithmetization model

### 4. PLONK's design philosophy can best be described as:
A. Execution trace-centric  
B. Circuit-centric  
C. Polynomial-centric  
D. Hash function-centric

---

## ✍️ Part 2: Short Answer Questions (15 points each, 30 points total)

### 5. Explain the mathematical principle of copy constraints in PLONK
Please use mathematical formulas to explain how polynomial interpolation proves equality when two sets A and B are equal.

### 6. Analyze the three main motivations for Plonky2's design
Please list and briefly explain the three core problems that Plonky2 aims to solve compared to traditional PLONK.

---

## 🧠 Part 3: Application Questions (15 points each, 30 points total)

### 7. Gate Constraint Design Exercise
Design PLONK gate constraints to implement the following computation: `w_c = 3w_a² + 2w_b + 5`
Please provide:
a) Number and types of gates needed  
b) Parameter settings for each gate (q_L, q_R, q_O, q_M, q_C)  
c) Necessary copy constraints

### 8. Scenario Analysis Question
A zkEVM project is choosing a zero-knowledge proof backend. The project requirements are:
- Need to process large amounts of repetitive EVM instruction execution
- Not particularly sensitive to proof size
- Want complete trustlessness
- Need to support proof aggregation

Please analyze the pros and cons of PLONK and Plonky2 in this scenario, and provide your recommended choice with reasoning.

---

# 📊 Quiz Solutions

## Part 1: Multiple Choice Answers

### 1. Answer: B
**Explanation:** PLONK's complete universal gate constraint includes five terms: linear terms q_L·w_a, q_R·w_b, q_O·w_c, multiplication term q_M·w_a·w_b, and constant term q_C. The flexibility of this equation comes from being able to implement various logic gates by setting different q parameters.

### 2. Answer: B  
**Explanation:** Permutation arguments are PLONK's core innovation, used to implement copy constraints. They mathematically prove that variables at different positions in the circuit have the same value, which is crucial for connecting inputs and outputs of different gates.

### 3. Answer: C
**Explanation:** Plonky2's proof generation time is typically in seconds, not milliseconds. Milliseconds is the verification time. Plonky2's main goals are transparency (no trusted setup), efficient recursion, and hybrid arithmetization.

### 4. Answer: B
**Explanation:** PLONK is "circuit-centric" in design, providing extremely flexible gate constraints and arbitrary connections, like a free circuit board. This contrasts with AIR's "execution trace-centric" approach.

---

## Part 2: Short Answer Solutions

### 5. Mathematical Principle of Copy Constraints (15 points)

**Core Principle:** 
If two multisets A = {a₁, a₂, ..., aₙ} and B = {b₁, b₂, ..., bₙ} are equal, then any of their symmetric polynomials are also equal.

**Mathematical Expression:**
```
For any β, γ, if A = B, then:
∏ᵢ(β·aᵢ + γ) = ∏ᵢ(β·bᵢ + γ)
```

**Application in PLONK:**
- Original position set: {w(σ(1)), w(σ(2)), ..., w(σ(n))}
- Permuted set: {w(π(1)), w(π(2)), ..., w(π(n))}
- Generate random challenges β, γ through Fiat-Shamir
- Prove equality of two polynomial products

**Scoring Criteria:**
- Correct understanding of polynomial properties of set equality (5 points)
- Correct mathematical formula (5 points)  
- Explanation of specific application in PLONK (5 points)

### 6. Three Design Motivations of Plonky2 (15 points)

**1. Eliminate Trusted Setup (5 points)**
- Problem: KZG commitments require "powers of tau" ceremony
- Risk: Centralization risk, if setup is compromised, entire system fails
- Solution: Adopt FRI commitment, completely transparent

**2. Achieve Efficient Recursion (5 points)**  
- Problem: Elliptic curve pairings extremely expensive to implement in circuits (~1M+ constraints)
- Limitation: Difficult to build recursive verification circuits
- Solution: FRI + Goldilocks field, recursive verification ~100K constraints

**3. Support Structured Computation (5 points)**
- Problem: PLONK's flexibility becomes a burden for repetitive, regular computations
- Scenario: zkEVM, zkVM requiring large amounts of repetitive instructions
- Solution: Hybrid model, use PLONK to implement AIR-style constraints

**Scoring Criteria:**
- Problem identification and solution for each motivation: 2.5 points each
- Clear logic and accurate expression

---

## Part 3: Application Solutions

### 7. Gate Constraint Design Exercise (15 points)

**Goal:** `w_c = 3w_a² + 2w_b + 5`

**Solution:**

**Step 1: Decompose Computation**
- Need to compute `w_a²` first
- Then compute linear combination

**Step 2: Gate Design**

**Gate 1 (Multiplication Gate):** Compute `w_a² → temp`
```
q_L = 0, q_R = 0, q_O = -1, q_M = 1, q_C = 0
Constraint: w_a · w_a - temp = 0
```

**Gate 2 (Linear Combination Gate):** Compute `3temp + 2w_b + 5 → w_c`
```
q_L = 3, q_R = 2, q_O = -1, q_M = 0, q_C = 5  
Constraint: 3·temp + 2·w_b - w_c + 5 = 0
```

**Step 3: Copy Constraints**
- Connect Gate 1's output temp to Gate 2's first input

**Scoring Criteria:**
- Correctly identify need for 2 gates (3 points)
- Gate 1 parameter settings correct (4 points)
- Gate 2 parameter settings correct (4 points)  
- Copy constraint design correct (4 points)

### 8. Scenario Analysis Question (15 points)

**Scenario Analysis:**
- Large amounts of repetitive EVM instruction execution
- Not sensitive to proof size
- Complete trustlessness requirement
- Need proof aggregation

**PLONK Analysis:**
Advantages:
- Mature technology, widely used
- Extremely small proof size (~400B)

Disadvantages:
- KZG requires trusted setup (doesn't meet trustlessness requirement)
- Low recursion efficiency, difficult aggregation
- Excessive flexibility for repetitive instructions

**Plonky2 Analysis:**
Advantages:
- FRI completely transparent, no trusted setup ✓
- Efficient recursion, strong aggregation capability ✓  
- Hybrid model suitable for repetitive instructions ✓
- Acceptable proof size (~45KB) ✓

Disadvantages:
- Relatively new technology
- Larger proof size than KZG

**Recommended Choice: Plonky2**

**Reasoning:**
1. **Meets Key Requirements**: Trustlessness is a hard requirement, only Plonky2 satisfies this
2. **Technical Match**: Repetitive instruction execution is Plonky2's hybrid model's advantage scenario
3. **Aggregation Capability**: Efficient recursion makes large-scale aggregation possible
4. **Acceptable Trade-off**: Increased proof size for complete transparency is worthwhile

**Scoring Criteria:**
- PLONK pros and cons analysis (4 points)
- Plonky2 pros and cons analysis (4 points)
- Requirements matching analysis (4 points)
- Recommendation reasoning logic (3 points)

---

## 🎯 Grading Scale

- **90-100 points:** Excellent - Deep understanding of PLONK and Plonky2 core concepts
- **80-89 points:** Good - Good grasp of basic concepts with analytical ability  
- **70-79 points:** Pass - Basic understanding of main concepts
- **60-69 points:** Fail - Need to re-learn some content
- **Below 60 points:** Fail - Recommend complete re-study of this module

## 📚 Review Recommendations

If scores are not ideal, focus on reviewing:
1. **The five components of PLONK's universal gate constraints**
2. **Mathematical principles and mechanisms of permutation arguments**  
3. **Plonky2's three major design goals compared to PLONK**
4. **Trade-off considerations in technology selection for different scenarios**
