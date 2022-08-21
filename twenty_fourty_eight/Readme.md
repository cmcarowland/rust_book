## 2048 Game
```
---------------------------------
|   4   |   8   |  32   |   2   |
---------------------------------
|   8   |  128  |   4   |   8   |
---------------------------------
|   2   |   4   |  16   |   4   |
---------------------------------
|   4   |   8   |   4   |   2   |
---------------------------------

Score : 1032
Moves : 117
```

#### I am currently learning rust and made this version of 2048 to get some practice.
- Game is traditional 2048
- Move using w(up),a(left),s(down),d(right) followed by a new line
    - Would like to move on key down but have not figured that out in rust yet.
- Try to combine like numbers to get to 2048. 
- Move towards a similar number to merge with it. 

<pre>
```
---------------------------------
|       |       |       |   2   |
---------------------------------
|       |       |       |  64   |
---------------------------------
|   2   |       |  <b>16</b>   |  <b>16</b>   |
---------------------------------
|   2   |   4   |  32   |   2   |
---------------------------------

Move LEFT was pressed <-----

Resulting in ...
---------------------------------
|   2   |       |   <b><i>2</i></b>   |       |
---------------------------------
|  64   |       |       |       |
---------------------------------
|   2   |  <b>32</b>   |       |       |
---------------------------------
|   2   |   4   |  32   |   2   |
---------------------------------

All numbers moved to the left.
The two 16s combined to make the <b>32</b> cell.  And a new <b><i>2</i></b> was also added to the board.
```
</pre>