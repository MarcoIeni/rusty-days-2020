# Cellular Game of Life

![gif](/assets/test_run.gif)

# Rules

![states](/assets/species.png)

A cell may be in one of those four states: male, female, tired (female), child or hunter.

## Behaviour

- When a male and a female are near enough, they make a child.
- A child after a certain period of time will count the number of males and females around him:
  - if there is noone around, it will become a hunter
  - if there are more males than females, it will become a female
  - else it will become a male
- When an hunter is near enough to a cell, it will kill the cell.
- If the hunter does not find a cell in a given period of time, it will die.

## Movements

All the cells normally goes at a constant speed towards a given direction.

- Males try to follow females that are on their field of view.
- Hunters try to hunt everything except for hunters that are on their fields of view.
- When a male and a female make a child, the female slows down for a while.

## Fields of view

**Hunters**: their field of view is a circular sector of the circle centered on them with a
given radius and angle, pointed towards the direction they are moving

**Other Cells**: their field of view is a circle centered on them with a give radius

## Inital state

The inital state and all the properties of the simulation can be modified inside [config.json](/config.json)

# Hackaton rules

> The topic of this year's hackathon is "Emergent phenomena", or if you prefer - "Amaze us with simple rules".
> Create an amazing result with simple ruleset.
> If you heard about cellular automaton, fractals, or similar constructs - this is what we are talking about.
