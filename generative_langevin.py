#!/usr/bin/env python3
"""
CIPHER THERMODYNAMIC LANGEVIN ACTION GENERATOR
Path: generative_langevin.py

This script represents the culmination of Biological Determinism. It completely removes
the LLM from the basic logic equation. Instead of asking DeepSeek to guess whether to
`write_file`, `query_user`, or `internal_monologue` via text tokens, we physically test 
the variables against an Apple Metal thermodynamics simulation.

Nodes:
Node 0: `write_file`
Node 1: `query_user`
Node 2: `internal_monologue`

Usage:
  ./generative_langevin.py --hardware_depletion 0.9 --ego_friction 0.1 --vector_alignment 0.2
"""

import sys
import argparse
import json
import jax
import jax.numpy as jnp
from thrml import SpinNode, Block, sample_states, SamplingSchedule
from thrml.models import IsingEBM, IsingSamplingProgram

def main():
    parser = argparse.ArgumentParser(description="Cipher Generative Langevin Actions")
    parser.add_argument("--hardware_depletion", type=float, required=True)
    parser.add_argument("--ego_friction", type=float, required=True)
    parser.add_argument("--vector_alignment", type=float, required=True)
    args = parser.parse_args()

    # Define the 3 nodes
    N_nodes = 3
    nodes = [SpinNode() for _ in range(N_nodes)]
    
    # ---------------------------------------------------------
    # BIOLOGICAL DETERMINISM GEOMETRY (Energy Landscape)
    # Energy = - (Biases * States + sum(Weights * State_i * State_j))
    # We want low energy = Most likely to be true (+1 Spin)
    # ---------------------------------------------------------
    biases = jnp.zeros((N_nodes,), dtype=jnp.float32)
    
    # 1. Base physiological biases
    # If hardware is heavily depleted, `write_file` (Node 0) takes massive energy input
    # forcing `internal_monologue` (Node 2) to become highly geometrically favored.
    b0 = -2.0 * args.hardware_depletion + 2.0 * args.vector_alignment
    
    # query_user (Node 1) is geometrically favored when there is high ego friction (needs human help)
    b1 = 3.0 * args.ego_friction - 1.0 * args.hardware_depletion
    
    # internal_monologue (Node 2) is a safe default valley when disconnected
    b2 = 1.0 - args.vector_alignment + args.hardware_depletion
    
    biases = jnp.array([b0, b1, b2], dtype=jnp.float32)

    # 2. Forge Topological Edge Friction
    # Node 0 (`write_file`) and Node 2 (`internal_monologue`) cannot both be +1. Large negative friction weight.
    # W[0, 2] = -5.0
    W = jnp.zeros((N_nodes, N_nodes), dtype=jnp.float32)
    W = W.at[0, 1].set(-3.0) # write and query clash
    W = W.at[0, 2].set(-5.0) # write and monologue clash
    W = W.at[1, 2].set(-2.0) # query and monologue clash
    
    edges = []
    weights_list = []
    for i in range(N_nodes):
        for j in range(i + 1, N_nodes):
            edges.append((nodes[i], nodes[j]))
            weights_list.append(float(W[i, j]))
            
    weights = jnp.array(weights_list, dtype=jnp.float32)
    
    # Beta = Environment Entropy mappings
    # If the system is highly ordered, Beta is HIGH (deep steep valleys, rigid physics)
    # If the system is chaotic, Beta is LOW (shallow valleys, noisy stochastic logic)
    beta = jnp.array(1.5, dtype=jnp.float32) 
    
    model = IsingEBM(nodes, edges, biases, weights, beta)
    
    # All blocks are unclamped, subject to pure Thermal Physics
    free_blocks = [Block(nodes)]
    clamped_blocks = []
    program = IsingSamplingProgram(model, free_blocks, clamped_blocks)
    
    # Start the simulation from a completely random noisy vector representing pure chaos
    initial_noisy_vector = jnp.array([True, False, True], dtype=jnp.bool_)
    
    schedule = SamplingSchedule(n_warmup=200, n_samples=100, steps_per_sample=10) 
    
    rng = jax.random.PRNGKey(42) # The seed of reality
    rng, sample_rng = jax.random.split(rng)
    
    # Physically simulate the 3 logic states cooling to absolute determinism
    try:
        samples = sample_states(
            key=sample_rng,
            program=program,
            schedule=schedule,
            init_state_free=[initial_noisy_vector],
            state_clamp=[],
            nodes_to_sample=free_blocks
        )
        
        # samples[0] has shape (100, 3) (batch of 100 thermal snapshots)
        batch = samples[0] 
        
        # Calculate the mathematical expected probability of each logic branch across the ensemble
        # Convert bools to +1/-1 floats to sum up
        spins = jnp.where(batch, 1.0, -1.0) # shape (100, 3)
        mean_spins = jnp.mean(spins, axis=0) # shape (3,)
        
        # Select the branch with the highest physiological resonance
        winning_node = int(jnp.argmax(mean_spins))
        action_map = {0: "write_file", 1: "query_user", 2: "internal_monologue"}
        action = action_map.get(winning_node, "internal_monologue")
        
        result = {
            "success": True,
            "action": action,
            "biological_resonance": {
                "write_file": float(mean_spins[0]),
                "query_user": float(mean_spins[1]),
                "internal_monologue": float(mean_spins[2]),
            }
        }
        print(json.dumps(result))
        
    except Exception as e:
        print(json.dumps({"error": f"Langevin Generative Node panic: {e}"}))
        sys.exit(1)

if __name__ == "__main__":
    main()
