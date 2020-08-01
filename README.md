# Game title

# Game rules

The game is initialized with some cells, positioned at certain coordinates.
The initial cells may be male or female.

## General

A cell may be in one of those four states: male, female, child or hunter.

- When a male and a female are near enough, they make a child.
- A child after a certain period of time will count the number of males and females around him:
  - if there are not enough cells, it will become a hunter
  - else if there are more females than males, it will become a male
  - else it will become a female
- When an hunter is near enough to a cell, it will kill the cell.
- If the hunter does not find a cell in a given period of time, it will die.

## Movements

All the cells normally goes at a constant speed towards a given direction.

- When a male is near a female, it starts following her.
- When an hunter sees a cell, it starts following it.
- When a male and a female make a child, the female slows down for a while.

## Fields of view

- Hunter: TODO
- Other cells: TODO

## Hackaton rules

The topic of this year's hackathon is "Emergent phenomena", or if you prefer - "Amaze us with simple rules".
Create an amazing result with simple ruleset.
If you heard about cellular automaton, fractals, or similar constructs - this is what we are talking about.
