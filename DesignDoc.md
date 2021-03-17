# Economic Simulation
## End Goals
* Output Trader Metrics
    * Average Age
    * Average Bank balance
    * Most Profitable Trader Statistics
        * Export to JSON automatically
* Track methodology versus results in a competitive market place


## Terms
* Trader - Trading entities that control goods and make decisions
* Producers - Farm or Mine that produces raw materials
* Factories - Turns raw materials into a intermediate or finished good
* Consumer - The entity that acts as all consumers in the market
* Goods - Any product or material
* Retail goods - Products that are only needed by the consumer

## Computations
* **Note** Each trader has a float from -20 to 20 that controls what they believe it is worth called attitude
* Producer Value - ((products/day) * price) * attitude

## Classes
* Sim
    * Simulation Controller Class
    * Land is a randomly generated number that seeds how many consumers(population) are set
    * Population is a % of land
    * Needs are a usage rate of products per day
        * Calculated as (usage rate per day * population) 
* Trader
    * trader id - unique trader id
    * age - tracks time alive
    * first name - string
    * last name - string
    * bank - hash map that tracks currency name and holdings
    * holdings - hashmap that tracks goods and holdings
* Producer
    * Has a daily upkeep cost
    * Produces a defined amount of a good each day

### Trader Behavior
#### Selling a product
* Setting a price
    * if not being sold retail = [cost / (100 - markup %)] * 100
    * if being sold already and cost is less than current price
        * Undercut new_price = (current price - cost) / 4 + current price
    * if being sold already and cost is above 
#### Buying a product
Consumer acts as a product sink
Manages needs based on a variable factor of land size

## Algorithms
1. Randomly Choose land size as square kilometers
2. Generate X number of Merchant where X is a % of land
    * Set percentage in sim file
3. Generate Y number of Producers where Y is a % of land
    * Set percentage in sim file
4. Merchants bid on Producers, where the value is determined by a
5. 

## Daily Process
The Consumer and all traders are looking to fill their needs each day.


## Simulation Initialization

