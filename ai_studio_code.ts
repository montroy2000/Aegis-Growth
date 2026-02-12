// Calculation for the State Machine UI
const getVaultState = (peg, oracleStale, hf) => {
  if (oracleStale || peg > 50) return "PANIC";
  if (peg > 25) return "EXIT";
  if (peg > 10) return "CONTRACT";
  if (hf < 2.40) return "CONTRACT"; // Protect Health Factor
  return "LOOP";
};

// Equity Calculation
const equity = totalSupplied - totalBorrowed;
const leverage = totalSupplied / equity;