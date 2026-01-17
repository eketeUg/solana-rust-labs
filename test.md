Devlog – January DD, 2026

Date & Total time spent today
• 2026-01-DD
• ~X hours

What I worked on today (concrete achievements)
• Tried to deploy my Anchor program to localnet
• Deployment got stuck at finalizing transaction...
• Diagnosed that the local validator state was likely corrupted or had pending transactions
• Restarted the validator using:

solana-test-validator -r

    •	After restarting, deployment worked successfully

Technical deep-dive / most interesting thing I learned

Anchor’s localnet deployment can hang if the local validator has a stuck state or pending transactions. Even when the program build is fine, Anchor waits for the transaction to finalize. Restarting the validator with --reset clears the ledger and validator state, allowing the deployment to proceed normally.

This reinforced the workflow of running the validator manually and using --skip-local-validator during repeated builds for faster iteration.

Key takeaway
• If localnet deployment hangs at finalizing transactions, always try:

solana-test-validator -r

    •	Keep the validator running manually for faster iteration and avoid unnecessary restarts during development.
