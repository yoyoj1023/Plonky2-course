# Module 3 Quiz: The Heart of Transparency - FRI Commitment Scheme

**Quiz Time:** 40 minutes  
**Total Score:** 100 points  
**Passing Score:** 70 points

---

## 📝 Part 1: Multiple Choice Questions (8 points each, 40 points total)

### 1. What is the main flaw of the KZG commitment scheme?
A. Proof size is too large
B. Verification time is too long
C. Requires trusted setup
D. Does not support polynomial commitments

### 2. What is FRI's core idea for proving polynomial low-degree property?
A. Directly compute all coefficients of the polynomial
B. Commit-fold-repeat recursive process, gradually reducing degree
C. Compare polynomial values at multiple points
D. Use elliptic curve pairing operations

### 3. What is the mathematical foundation of FRI folding operation?
A. Additive properties of polynomials
B. Odd-even decomposition of polynomials: P(x) = P_even(x²) + x·P_odd(x²)
C. Derivative properties of polynomials
D. Integral properties of polynomials

### 4. Compared to KZG, what is FRI's main advantage in recursive verification?
A. Smaller proofs
B. Only relies on hash functions, avoiding complexity of elliptic curve pairings
C. Faster generation speed
D. Simpler mathematical theory

### 5. What level of security bits can Plonky2's typical FRI security parameter settings provide?
A. 128 bits
B. 256 bits  
C. 80 bits
D. 64 bits

---

## ✍️ Part 2: Short Answer Questions (12 points each, 36 points total)

### 6. Explain FRI's high-level working principle
Please explain how FRI transforms complex low-degree testing problems into simple constant checks through the "commit-fold-repeat" process.

### 7. Compare performance characteristics of FRI and KZG
Please compare from the following four dimensions:
a) Proof size
b) Verification time  
c) Trusted setup requirements
d) Recursion friendliness

### 8. Analyze why FRI is more suitable for Plonky2's recursion goals
Please explain FRI's cost advantages in circuit implementation and how this supports efficient recursion.

---

## 🧠 Part 3: Computation Questions (12 points)

### 9. FRI Folding Calculation Exercise
Given polynomial `P(x) = 2x³ + 3x² + x + 4`, calculate one FRI folding operation:
a) Decompose P(x) into odd and even terms
b) Using random challenge α = 7, compute the folded polynomial
c) Verify the degree of the folded polynomial

---

## 💡 Part 4: Analysis Questions (12 points)

### 10. Scenario Selection Analysis
A DeFi protocol needs to choose a polynomial commitment scheme with the following requirements:
- Deployed on Ethereum mainnet, gas cost sensitive
- Need on-chain proof verification
- Users have strong decentralization demands
- May need Layer 2 scaling in the future

Please analyze the applicability of FRI and KZG in this scenario and provide recommendations.

---

# 📊 Quiz Solutions

## Part 1: Multiple Choice Answers

### 1. Answer: C
**Explanation:** KZG requires trusted setup ("powers of tau" ceremony), which brings centralization risks. If the setup process is compromised or participants collude, the security of the entire system is threatened, contradicting blockchain's decentralization philosophy.

### 2. Answer: B
**Explanation:** FRI works through recursive "commit-fold-repeat" process: each folding round halves the polynomial degree, ultimately transforming a complex low-degree testing problem into simple checking of constant polynomials.

### 3. Answer: B  
**Explanation:** FRI folding is based on odd-even decomposition of polynomials. Any polynomial can be written as P(x) = P_even(x²) + x·P_odd(x²), where P_even contains even terms and P_odd contains odd terms. This decomposition is the mathematical foundation enabling FRI to gradually reduce degree.

### 4. Answer: B
**Explanation:** FRI only relies on hash functions and field operations, while KZG requires elliptic curve pairings. When implementing in circuits, hash function constraint costs are far lower than pairing operations (~1000 vs 1,000,000+ constraints), giving FRI huge advantages in recursive verification.

### 5. Answer: C
**Explanation:** Plonky2's FRI configuration (28 query rounds, specific parameter settings) provides approximately 80 bits of security, which is sufficient for most applications while maintaining good performance balance.

---

## Part 2: Short Answer Solutions

### 6. FRI High-Level Working Principle (12 points)

**Commitment Phase (4 points)**
- Prover commits to polynomial P(x), claiming its degree ≤ d
- Use Merkle tree to commit all evaluations of polynomial on some domain

**Folding Phase (4 points)**  
- Verifier sends random challenge α
- Prover computes folded polynomial: P_folded(y) = P_even(y) + α·P_odd(y)
- New polynomial's degree is approximately half of original

**Repeat Process (2 points)**
- Repeat folding process for k rounds until obtaining constant polynomial
- Each round halves degree: d → d/2 → d/4 → ... → constant

**Final Verification (2 points)**
- Perform simple check on constant polynomial
- Recursively verify consistency of all folding steps

### 7. FRI and KZG Performance Comparison (12 points)

**a) Proof Size (3 points)**
- **KZG**: Extremely small (~32 bytes), only needs one group element
- **FRI**: Medium (~45 KB), includes multiple rounds of Merkle paths and queries

**b) Verification Time (3 points)**
- **KZG**: Fast (~2ms), only needs one pairing operation
- **FRI**: Fast (~0.5ms), O(log d) hash operations

**c) Trusted Setup Requirements (3 points)**  
- **KZG**: Required, "powers of tau" ceremony
- **FRI**: Not required, completely transparent

**d) Recursion Friendliness (3 points)**
- **KZG**: Difficult, pairing implementation in circuits extremely expensive (~1M+ constraints)
- **FRI**: Easy, only needs hash and field operations (~100K constraints)

### 8. Analysis of Why FRI Suits Recursion (12 points)

**Circuit Implementation Cost Analysis (6 points)**
- **Elliptic curve pairing**: Requires complex elliptic curve operations, modular arithmetic, inversions, etc., enormous constraint count
- **Hash functions**: Poseidon hash designed for ZK-friendliness, only needs field addition and multiplication
- **Cost comparison**: FRI recursive verification constraint count is 10-100x less than KZG

**Mechanisms Supporting Efficient Recursion (6 points)**
1. **Field operation friendly**: FRI verification only needs basic Goldilocks field operations
2. **Avoid complex cryptography**: No need for elliptic curves, pairings, etc.
3. **Parallelization potential**: Hash verification can be highly parallelized
4. **Recursive composability**: Multiple FRI proofs easily aggregate into single proof

---

## Part 3: Computation Solutions

### 9. FRI Folding Calculation (12 points)

**Given:** P(x) = 2x³ + 3x² + x + 4, α = 7

**a) Odd-even decomposition (4 points)**
```
P_even(x²) = 3x² + 4 = 3y + 4  (where y = x²)
P_odd(x²) = 2x² + 1 = 2y + 1   (extract coefficients of x)

Verification: P(x) = (3x² + 4) + x(2x² + 1) = 3x² + 4 + 2x³ + x = 2x³ + 3x² + x + 4 ✓
```

**b) Folding calculation (4 points)**
```
P_folded(y) = P_even(y) + α · P_odd(y)
            = (3y + 4) + 7 · (2y + 1)
            = 3y + 4 + 14y + 7  
            = 17y + 11
```

**c) Degree verification (4 points)**
```
Original polynomial P(x) degree: 3
Folded P_folded(y) degree: 1

Degree reduction: 3 → 1, conforms to FRI folding's degree halving principle
(In the context of variable substitution y = x², actual degree reduces from 3 to 1)
```

---

## Part 4: Analysis Solutions

### 10. DeFi Protocol Scenario Analysis (12 points)

**Requirements Analysis (3 points)**
- Gas cost sensitive → Favor small proofs
- On-chain verification → Need efficient verification
- Decentralization demand → Favor no trusted setup
- Future Layer 2 → May need recursion capability

**KZG Analysis (3 points)**
Advantages:
- Extremely small proof (32 bytes), lowest gas cost
- Efficient on-chain verification, mature technology

Disadvantages:
- Requires trusted setup, violates decentralization demand
- Limited Layer 2 recursion capability

**FRI Analysis (3 points)**  
Advantages:
- Completely transparent, no trusted setup
- Ready for future Layer 2 recursion
- Actually faster verification time

Disadvantages:
- Larger proof size (45KB), higher gas cost

**Recommended Solution (3 points)**
**Short-term recommendation: KZG**
- Current gas cost is main bottleneck
- Trusted setup risk controllable in short term
- High technology maturity

**Long-term recommendation: FRI**  
- Decentralization is fundamental value
- Layer 2 migration is inevitable trend
- Gas cost issues will be alleviated with technological development

**Suggested strategy:** Adopt modular design supporting smooth future migration from KZG to FRI.

---

## 🎯 Grading Scale

- **90-100 points:** Excellent - Deep understanding of FRI principles and application scenarios
- **80-89 points:** Good - Good grasp of basic concepts with simple analysis capability
- **70-79 points:** Pass - Basic understanding of main concepts
- **60-69 points:** Fail - Need to re-learn FRI related content  
- **Below 60 points:** Fail - Recommend complete re-study of this module

## 📚 Review Recommendations

If scores are not ideal, focus on reviewing:
1. **Mathematical foundation of FRI: polynomial odd-even decomposition**
2. **Complete process of commit-fold-repeat**
3. **Pros and cons comparison between FRI and KZG**
4. **Cost analysis in recursive verification**  
5. **Technology selection considerations in real application scenarios**
