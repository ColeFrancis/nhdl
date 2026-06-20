# TODO

- Impulse (Spiking) entity type
      - Takes "true" or "false" like Bool but only is true if it is
      the event that triggered the call
      - Thus simulator needs to handle all events at the same time step
      at the same time so if two spikes arrive at a relation simultaneously
      the simulation will be able to know

# Possible future extensions

- Nest networks inside of networks
- Tuples (chisen bundles) and arrays of entites in networks
- Complex numbers
- Lambda expressons (somehow)
- way to randomly place and connect entities and relations in nets
- Dynamic or self-rewriting networks
 
# Analytical tools to add

- Graph structure analysis
- Information theoretic quantities
      - entropy, mutual information, transformation entropy, complexity
- State space analysis
      - Detect memory and state
- Graph rewriting
