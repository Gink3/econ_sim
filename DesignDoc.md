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
### Class interactions
Sim acts as a controller for Trader, Producer, Consumer, Market
Trader owns Producer
Consumer looks at market and buys from trader
### Sim
* Simulation Controller Class
* Land is a randomly generated number that seeds how many consumers(population) are set
* Population is a % of land
* Needs are a usage rate of products per day
    * Calculated as (usage rate per day * population) 
#### Methods
* End sim - runs all of the end of simulation metrics calculations
* create_trader - generates a new trader
* cal_avg_age - calculates average age of all traders
### Trader
* trader id - unique trader id
* age - tracks time alive
* first name - string
* last name - string
* bank - hash map that tracks currency name and holdings
* holdings - hashmap that tracks goods and holdings
#### Methods

### Producer
A producer looks to have enough inventory for a week and throughout the week it will try and refill the inventory
* Has a daily upkeep cost
* Produces a defined amount of a good each day
* Owner tid **Note** producer tid = 0 if owner is consumer at start of simulation
* Production - hashmap pair
* Needs - hashmap pair
* Holdings - current inventory for producer
#### Methods
* Initialization - determines the % of each type of production
    * Select production type (material harvest or processor)
    * Set needs according to production selected
* Owner change
* Added inventory
* Check inventory


### Trader Behavior
#### Buying a product
Looks at a given commodity, determines if the price is above or below an internal price prediction

#### Nerual Network 
Inputs
* Current Price
* 
Outputs
* Buy commodity
* Sell commondity

### Market behavior
Market transactions are solved on a per time step basis resolving any trades utilizing a bid system that will execute upon having intersecting buy and sell bids. Editing and resolving quantities in the 
Market Entry
* Commodity name
* Current Price = Highest bid - Lowest sell price
* List of bids ordered by price given as a tuple (bid price per unit, tid, number of units)
* Quantity of units outstanding
* Quantity of units looking to sell
* Quantitiy of units looking to be bought

## Daily Process
The Consumer and all traders are looking to fill their needs each day.
1. Generate random order for traders to take their actions in
2. Iterate over traders 1 by 1
3. Trader uses an initially random set of weights to determine the action taken that day
    * Bid
    * Sell
    * Wait
4. Execute choosen action
5. Adjust prices accordingly

## Simulation Initialization
Every Trader gets 10000 dollars
Commdity Prices will be randomized
N - number of traders, goal is 10-50 traders

## TODO Priorities
1. Good Functionality
    * Unit Tests
2. Market functionality
    * Placing bids
    * Cancelling bids
    * Setting sell prices
    * Executing trades
3. Trader Internal Thoughts
    * Internal Pricing
    * Nerual Network for deciding actions
    * 
4. Time Step functionality
5. Bank Function


## Prior Works
[An artificial stock market](https://www.researchgate.net/profile/R-Palmer-2/publication/225471692_An_artificial_stock_market/links/0fcfd513767593647d000000/An-artificial-stock-market.pdf)
[Manually creating a nerual network](https://pwy.io/en/posts/learning-to-fly-pt2/)