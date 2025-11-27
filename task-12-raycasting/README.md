# DD1338 Week 12

Author: Benjamin Widman

Will the Ray Sorcerers finally be tested on their powers handling the rays??

## Getting familiar with raycasting

Raycasting is essentially about sending out rays for every pixel on the width of your screen (each column), marching them forward in our 2D scene and detecting if they hit a wall. If a ray hits a wall, a column with it's height inversely proportionate to the marched distance from the camera is drawn on that x-position. This way we get the illusion of 3D since parts of walls further away appear smaller.

The scene can be represented represented as a 2D array indicating what cells are and aren't walls. For example, using `1` representing a wall and `0` as air:

```
{
    {1,1,1,1,1},
    {1,0,1,0,1},
    {1,0,0,0,1},
    {1,1,0,0,1},
    {1,1,1,1,1}
}
```

We could march in decimal numbers but round down to the closest integer to index in this game map and determine if a ray position is inside a wall or not.

The player/camera can be represented using only x and y coordinates, as well as an angle to signify in which direction it's pointing.

Pseudo code for a simple unshaded but fisheye-corrected implementation using vectors instead of linear :

```
while true
    clear screen
    for x in 0 to SCREEN_WIDTH
        y = tan(FOV / 2) * SCREEN_WIDTH/2
        direction = (x - SCREEN_WIDTH/2, y)
        marching_position = camera_position
        for distance = 1; distance < MAX_DISTANCE; distance += STEP_SIZE
            marching_position += (distance^2, distance^2)
            if map[floor(marching_position.y)][floor(marching_position.x)] == WALL
                wall_height = SCREEN_HEIGHT / perpendicular_distance
                draw line from (x, SCREEN_HEIGHT/2 - wall_height/2) to (x, SCREEN_HEIGHT/2 + wall_height/2)
```

Keep in mind this simplified pseudo code is only for inspiration and understanding the algorithm. I.e. don't expect it to work if you directly translate it to Python.

Here's an example of what the rendering could look like on the left and an overview of the map and the sent out rays on the right:

![raycasting example](https://upload.wikimedia.org/wikipedia/commons/e/e7/Simple_raycasting_with_fisheye_correction.gif)

## Learning material

To get a more in-depth explanation I recommend to either:
- Read [this article](https://lodev.org/cgtutor/raycasting.html) (very detailed, can recommend)
- Watch [OneLoneCoder's implementation in the command line](https://www.youtube.com/watch?v=xW8skO7MFYw)
- Watch [3DSage's video](https://www.youtube.com/watch?v=gYRrGTC7GtA)

## Assignment

1) Create a repository named `<KTH_ID>-raycasting` and clone it.
2) Write a raycaster in an optional language.
   - Implement controls so you can move around (moving with WASD and rotating with LEFT & RIGHT)
   - _Optional extra_: textured walls by sampling images instead of drawing single-colored lines
