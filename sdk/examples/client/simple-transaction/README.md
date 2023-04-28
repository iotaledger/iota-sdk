# Send a Value Transaction

To transfer funds in Stardust, a transaction must include:

* **Inputs**: unspent transaction outputs holding funds
* **Unlock** blocks: signatures authorizing the consumption of inputs
* **Outputs**: Newly created outputs holding transferred funds. 

You can use basic Outputs to define the base token funds held in the `Amount` field. These outputs must include at
least one Unlock Condition, specifically the `Address Unlock Condition`, which requires a valid signature for the
recipient's address to unlock the output in a transaction. The signature authorizes the consumption of inputs and
declares the intention to create new outputs.