use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

/// Course Example: Using Plonky2 to prove the computation of the 100th Fibonacci number
/// This example demonstrates the complete Plonky2 development workflow:
/// 1. Circuit design
/// 2. Constraint construction  
/// 3. Witness assignment
/// 4. Proof generation and verification
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    println!("=== Plonky2 Course Example: Fibonacci Proof ===\n");

    // 1. Circuit configuration
    println!("1. Configuring circuit...");
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // 2. Define circuit variables
    println!("2. Defining circuit variables...");
    let initial_a = builder.add_virtual_target(); // F(0) = 0
    let initial_b = builder.add_virtual_target(); // F(1) = 1
    
    // 3. Build Fibonacci computation chain
    println!("3. Building Fibonacci computation circuit...");
    let mut prev_target = initial_a;
    let mut curr_target = initial_b;
    
    // Compute F(2) to F(100), requiring 99 iterations
    for i in 0..99 {
        // next = prev + curr (Fibonacci recurrence relation)
        let next_target = builder.add(prev_target, curr_target);
        
        // State update: prepare for next iteration
        prev_target = curr_target;
        curr_target = next_target;
        
        // Progress display (only show first few steps)
        if i < 5 {
            println!("   Building constraint F({}) = F({}) + F({})", i + 2, i + 1, i);
        } else if i == 5 {
            println!("   ... (continuing to build up to F(100))");
        }
    }

    // 4. Define public inputs (values that can be verified externally)
    println!("4. Registering public inputs...");
    builder.register_public_input(initial_a);    // F(0) = 0
    builder.register_public_input(initial_b);    // F(1) = 1  
    builder.register_public_input(curr_target);  // F(100)

    // 5. Build circuit data structure
    println!("5. Compiling circuit...");
    let data = builder.build::<C>();
    println!("   Circuit size: {} constraints", data.common.degree());
    println!("   Number of public inputs: {}", data.common.num_public_inputs);

    // 6. Prepare witness (private input values)
    println!("6. Preparing witness data...");
    let mut pw = PartialWitness::new();
    pw.set_target(initial_a, F::ZERO)?;  // F(0) = 0
    pw.set_target(initial_b, F::ONE)?;   // F(1) = 1
    println!("   Set F(0) = 0");
    println!("   Set F(1) = 1");

    // 7. Generate zero-knowledge proof
    println!("7. Generating zero-knowledge proof...");
    let start_time = std::time::Instant::now();
    let proof = data.prove(pw)?;
    let prove_time = start_time.elapsed();
    
    println!("   Proof generation complete!");
    println!("   Generation time: {:?}", prove_time);
    println!("   Proof size: {} bytes", proof.to_bytes().len());

    // 8. Display computation results
    println!("8. Verifying public inputs...");
    println!("   F(0) = {}", proof.public_inputs[0]);
    println!("   F(1) = {}", proof.public_inputs[1]); 
    println!("   F(100) = {} (in Goldilocks field)", proof.public_inputs[2]);
    
    // Calculate actual F(100) for comparison (using standard arithmetic)
    let mut a = 0u128;
    let mut b = 1u128;
    for _ in 0..99 {
        let next = a + b;
        a = b;
        b = next;
    }
    println!("   F(100) = {} (standard arithmetic)", b);
    println!("   Note: Due to Goldilocks field modular arithmetic, the two values will differ");

    // 9. Verify zero-knowledge proof
    println!("9. Verifying zero-knowledge proof...");
    let start_time = std::time::Instant::now();
    data.verify(proof.clone())?;
    let verify_time = start_time.elapsed();
    
    println!("   Proof verification successful!");
    println!("   Verification time: {:?}", verify_time);

    // 10. Performance summary
    println!("\n=== Performance Summary ===");
    println!("Constraint count: {}", data.common.degree());
    println!("Proof size: {} KB", proof.to_bytes().len() / 1024);
    println!("Generation time: {:?}", prove_time);
    println!("Verification time: {:?}", verify_time);
    println!("Performance ratio: Generation/Verification = {:.1}x", 
             prove_time.as_millis() as f64 / verify_time.as_millis() as f64);

    // 11. Key learning points
    println!("\n=== Key Learning Points ===");
    println!("1. Circuit design: Using execution trace thinking model");
    println!("2. Constraint construction: add() operation automatically creates addition constraints");
    println!("3. Public/Private: F(0),F(1),F(100) are public, intermediate steps are private");
    println!("4. Zero-knowledge property: Verifier only knows results, not computation process");
    println!("5. Succinctness: Proof size is fixed, independent of computation scale");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_circuit() -> Result<()> {
        // Test smaller Fibonacci sequence
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // Circuit to compute F(5) = 5
        let initial_a = builder.add_virtual_target();
        let initial_b = builder.add_virtual_target();
        
        let mut prev = initial_a;
        let mut curr = initial_b;
        
        // F(2) = 1, F(3) = 2, F(4) = 3, F(5) = 5
        for _ in 0..4 {
            let next = builder.add(prev, curr);
            prev = curr;
            curr = next;
        }

        builder.register_public_input(initial_a);
        builder.register_public_input(initial_b);
        builder.register_public_input(curr);

        let data = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_target(initial_a, F::ZERO)?;
        pw.set_target(initial_b, F::ONE)?;

        let proof = data.prove(pw)?;
        
        // Verify results
        assert_eq!(proof.public_inputs[0], F::ZERO); // F(0) = 0
        assert_eq!(proof.public_inputs[1], F::ONE);  // F(1) = 1
        assert_eq!(proof.public_inputs[2], F::from_canonical_u64(5)); // F(5) = 5

        data.verify(proof)?;
        
        Ok(())
    }

    #[test]
    fn test_fibonacci_values() {
        // Verify correctness of standard Fibonacci sequence
        fn fib(n: usize) -> u64 {
            if n <= 1 { return n as u64; }
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }

        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(10), 55);
    }
}
