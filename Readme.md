Example 1\
`cargo run -- -x 1000 -y 1000 -p 100 800 800 100 800 800 -d 2 -i 100000`\
![3 points with a proportional distance of half](./assets/image1.png)

Example 2\
`cargo run -- -x 1000 -y 1000 --points 100 100 100 800 800 100 800 800 --dist 3 --iter 100000`\
![4 points with a proportional distance of a third](./assets/image2.png)

Example 3\
`cargo run -- -x 2000 -y 2000 -e 10 -d 3 -i 500000`\
![10 points equidistant proporional distance of a third iterated 500_000 times](./assets/image3.png)

 ``Usage: chaos_game.exe [OPTIONS]``

 ``Options:
  -x <X>                         [default: 1000]\
  -y <Y>                         [default: 1000]\
  -e, --equidistant <PTS>        [default: 3]\
  -p, --points <COORDINATES>...\
  -d, --dist <PROP>              [default: 2]\
  -i, --iter <ITERS>             [default: 10000]\
  -h, --help                     Print help``