Example 1\
`cargo run -- -x 1000 -y 1000 -p 100 800 800 100 800 800 -d 2 -i 100000`\
<!-- ![3 points with a proportional distance of half](./assets/image1.png){width=250; height=250; } -->
<img src="./assets/image1.png" alt="3 points with a proportional distance of half" width="250" height=250/>

Example 2\
`cargo run -- -x 1000 -y 1000 --points 100 100 100 800 800 100 800 800 -d 3 --iter 100000`\
<img src="./assets/image2.png" alt="4 points with a proportional distance of a 1/3" width="250" height=250/>

Example 3\
`cargo run -- -x 2000 -y 2000 -e 10 -d 3 -i 500000`\

<img src="./assets/image3.png" alt="10 points equidistant proporional distance of a third iterated 500_000 times" width="250" height=250/>

Example 4\
`cargo run -- -x 10000 -y 10000 -e 10 -n 4 -d 5 -i 10000000`\
<img src="./assets/image4.png" alt="10 points equidistant proporional distance of a 4/5 iterated 10_000_000 times" width="250" height=250/>

 ``Usage: chaos_game.exe [OPTIONS]``

 ```
 Options:
  -x <X>                         [default: 1000]
  -y <Y>                         [default: 1000]
  -e, --equidistant <PTS>        [default: 3]
  -p, --points <COORDINATES>...
  -n, --numerator <NUM>          [default: 1]
  -d, --denominator <DENO>       [default: 2]
  -i, --iter <ITERS>             [default: 10000]
  -h, --help                     Print help
  ```