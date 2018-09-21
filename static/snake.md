# How to make Snake

In Snake, you have a 2 dimensional board, unless you're making 3 dimensional snake, which is out of scope of this document.

The board state usually has three types of objects: A snake part, a piece of food that you can eat and grow with, and emptiness.

You can represent this with enums or different values, i.e. 0 = empty, 1 = food, 2 = snake.

The food will be eaten when your snake head touches it.

The food spawns randomly around the board on empty squares.

The snake moves in one of the four cardinal directions.

When the snake moves into a wall or into itself, the player loses.

When the snake touches food, the snake grows.

When the snake moves, the whole snake moves as if its one entity.

One way to implement that, is to think about it kind of like a linked list. Each snake position is a tuple of (x, y), and you have a list of [(x, y), (x, y), ...].

When the snake moves, you can prepend the new position as the head of the list, and pop off the position at the end of the list.

Trick for growing: when you've eaten an apple, do the same thing, but don't pop from the end of the list!

## Logic

Setup:
    Initialize the board to all empty.
    Spawn the snake head somewhere.
    Spawn some food.

Game loop:
    If there is not enough food on the board:
        Spawn food randomly at an empty tile.
    Move the snake in whatever direction it is moving in.
        Before moving it, check if the position it'll be in overlaps with food. If so, grow the snake.
        If it will overlap with itself or a wall, game over.

Take player input when you can, and adjust snake direction. If text based, you can do it turn-based, or you animate the game at a set interval.
