
import random
import math
import statistics

# --- Configuration Constants ---
NUM_SIMULATIONS = 10000
INITIAL_POSITIONS = 10
# 1.4M CU limit / 400k CU per unwind = 3.5 -> floor(3)
UNWINDS_PER_TX = 3 
BLOCK_TIME_SEC = 0.4

# Market Parameters
# Annual volatility of 100% (high stress scenario)
VOLATILITY_ANNUAL = 1.0 
# Drift (assume neutral or slightly negative during crash, let's use 0 for pure volatility impact)
DRIFT_ANNUAL = 0.0

# Network Parameters
# Probability that a transaction fails to land in a block due to congestion
FAIL_RATE_CONGESTION = 0.15 # 15% failure rate
# Probability of total missed block (rpc/network/leader issues)
FAIL_RATE_NETWORK = 0.05 

# Slippage Parameters
# Base slippage per unwind unit (e.g. 0.1% per position)
BASE_SLIPPAGE = 0.001 
# Impact slippage: increases quadratically with batch size? 
# Lets keep it linear for now: Slippage = Base * Quantity
# But if market is crashing, liquidity is thinner.
# Effective Price = Oracle Price * (1 - Slippage)

# Initial Asset Price
INITIAL_PRICE = 100.0

# --- Helper Functions ---

def run_simulation_step():
    """
    Runs a single simulation of unwinding 10 positions.
    Returns:
        - total_loss (float): Loss relative to initial value
        - blocks_taken (int): Number of blocks to fully unwind
        - min_price_execution (float): Lowest price executed at
        - max_slippage (float): Maximum slippage encountered in a single batch
    """
    
    positions_remaining = INITIAL_POSITIONS
    current_price = INITIAL_PRICE
    
    total_value_realized = 0.0
    blocks_passed = 0
    
    # Track stats
    min_execution_price = float('inf')
    max_batch_slippage = 0.0
    
    dt = BLOCK_TIME_SEC / (365 * 24 * 3600) # Time step in years
    
    while positions_remaining > 0:
        blocks_passed += 1
        
        # 1. Price limit process (GBM)
        # S_t = S_{t-1} * exp((mu - 0.5*sigma^2)*dt + sigma*sqrt(dt)*Z)
        shock = random.gauss(0, 1)
        drift_factor = (DRIFT_ANNUAL - 0.5 * VOLATILITY_ANNUAL**2) * dt
        vol_factor = VOLATILITY_ANNUAL * math.sqrt(dt) * shock
        
        current_price *= math.exp(drift_factor + vol_factor)
        
        # 2. Attempt Unwind
        # Check network success
        if random.random() < (FAIL_RATE_CONGESTION + FAIL_RATE_NETWORK):
            # Transaction failed to land
            continue
            
        # Determine batch size
        batch_size = min(positions_remaining, UNWINDS_PER_TX)
        
        # 3. Calculate Execution
        # Slippage increases with batch size in a constrained market
        # Simple linear model: 0.5% per unit in crash? Let's say 0.2% per unit base
        current_slippage = BASE_SLIPPAGE * batch_size 
        
        # Additional "panic" penalty if price drops fast? 
        # For now, keep it simple.
        
        executed_price = current_price * (1.0 - current_slippage)
        
        total_value_realized += batch_size * executed_price
        positions_remaining -= batch_size
        
        # Update stats
        if executed_price < min_execution_price:
            min_execution_price = executed_price
        if current_slippage > max_batch_slippage:
            max_batch_slippage = current_slippage
            
    # Calculate Results
    initial_value = INITIAL_POSITIONS * INITIAL_PRICE
    total_loss = initial_value - total_value_realized
    
    return {
        "loss": total_loss,
        "blocks": blocks_passed,
        "min_price": min_execution_price,
        "max_slippage": max_batch_slippage,
        "percentage_loss": (total_loss / initial_value) * 100
    }

def main():
    print(f"Starting Monte Carlo Simulation ({NUM_SIMULATIONS} iterations)...")
    print(f"Parameters: Vol={VOLATILITY_ANNUAL*100}%, FailRate={(FAIL_RATE_CONGESTION+FAIL_RATE_NETWORK)*100}%, Unwinds/Tx={UNWINDS_PER_TX}")
    
    results = []
    for _ in range(NUM_SIMULATIONS):
        results.append(run_simulation_step())
        
    # Aggregate Data
    losses = [r['loss'] for r in results]
    percent_losses = [r['percentage_loss'] for r in results]
    blocks = [r['blocks'] for r in results]
    
    avg_loss = statistics.mean(losses)
    max_loss = max(losses)
    p99_loss = sorted(percent_losses)[int(NUM_SIMULATIONS * 0.99)]
    p95_loss = sorted(percent_losses)[int(NUM_SIMULATIONS * 0.95)]
    
    avg_blocks = statistics.mean(blocks)
    max_blocks = max(blocks)
    p99_blocks = sorted(blocks)[int(NUM_SIMULATIONS * 0.99)]
    
    print("\n--- Results ---")
    print(f"Initial Portfolio Value: ${INITIAL_POSITIONS * INITIAL_PRICE:,.2f}")
    print(f"Average Loss: ${avg_loss:,.2f} ({statistics.mean(percent_losses):.4f}%)")
    print(f"95th Percentile Loss: {p95_loss:.4f}%")
    print(f"99th Percentile Loss: {p99_loss:.4f}%")
    print(f"Max Loss Observed: {max(percent_losses):.4f}%")
    
    print(f"\n--- Timing (Blocks) ---")
    print(f"Average Unwind Time: {avg_blocks:.2f} blocks (~{avg_blocks*BLOCK_TIME_SEC:.2f} sec)")
    print(f"99th Percentile Time: {p99_blocks} blocks (~{p99_blocks*BLOCK_TIME_SEC:.2f} sec)")

    print(f"Max Time Observed: {max_blocks} blocks (~{max_blocks*BLOCK_TIME_SEC:.2f} sec)")

    print("\n--- Sensitivity Analysis: Max Slippage-per-Block Survival ---")
    print("Testing what constant price drop per block leads to > 10% and > 30% portfolio loss...")
    
    thresholds = [0.10, 0.30]
    results_sensitivity = {}
    
    # Test slippage per block from 0.1% to 20% in 0.1% increments
    for slip_bps in range(1, 200, 5): 
        slip_per_block = slip_bps / 10000.0
        
        # Run a small batch to estimate average loss at this decay rate
        batch_losses = []
        for _ in range(200): # 200 runs per point
             # Re-implement step logic with forced decay
            positions_remaining = INITIAL_POSITIONS
            current_price = INITIAL_PRICE
            total_value_realized = 0.0
            
            while positions_remaining > 0:
                # Forced decay
                current_price *= (1.0 - slip_per_block)
                
                # Network fail check
                if random.random() < (FAIL_RATE_CONGESTION + FAIL_RATE_NETWORK):
                    continue
                    
                batch_size = min(positions_remaining, UNWINDS_PER_TX)
                # Execution slippage (market depth)
                current_slippage = BASE_SLIPPAGE * batch_size
                executed_price = current_price * (1.0 - current_slippage)
                
                total_value_realized += batch_size * executed_price
                positions_remaining -= batch_size
                
            loss_pct = (INITIAL_POSITIONS * INITIAL_PRICE - total_value_realized) / (INITIAL_POSITIONS * INITIAL_PRICE)
            batch_losses.append(loss_pct)
            
        avg_loss = statistics.mean(batch_losses)
        
        # Check thresholds
        for t in thresholds:
            if t not in results_sensitivity and avg_loss > t:
                results_sensitivity[t] = slip_per_block
                
    for t in thresholds:
        if t in results_sensitivity:
            print(f"Threshold limit (> {t*100}% loss): {results_sensitivity[t]*100:.2f}% price drop per block")
        else:
            print(f"Threshold limit (> {t*100}% loss): > 2.00% price drop per block")

if __name__ == "__main__":
    main()
