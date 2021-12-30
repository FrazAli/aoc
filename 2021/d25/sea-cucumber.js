const fs = require('fs')

let cmap = fs.readFileSync('input.txt', 'utf8').split('\n').filter(x=>x !== '').map(x=>x.split(''));

h = cmap.length;
w = cmap[0].length;

const printMap = function () {
  for (let y=0; y<h; y++) {
    for (let x=0; x<w; x++) {
      process.stdout.write(cmap[y][x]);
    }
    console.log();
  }
  console.log("-----------------");
}

printMap()

let step = 0;
let moved = true;
while (true) {
  moved = false;
  for (let y=0; y<h; y++) {
    let wrap = cmap[y][0] === '.' && cmap[y][w-1] === '>';
    for (let x=0; x<w-1; x++) {
      let c = cmap[y][x]
      if (c === '>') {
        if (cmap[y][x+1] === '.') {
          cmap[y][x] = ".";
          cmap[y][x+1] = c;
          x++;
          moved = true;
        }
      }
    }

    if (wrap === true) {
        cmap[y][w-1] = '.';
        cmap[y][0] = '>';
        moved = true;
    }
  }

  for (let x=0; x<w; x++) {
    let wrap = cmap[0][x] === '.' && cmap[h-1][x] === 'v';
    for (let y=0; y<h-1; y++) {
      let c = cmap[y][x]
      if (c === 'v') {
        if (cmap[y+1][x] === '.') {
          cmap[y][x] = '.';
          cmap[y+1][x] = c;
          y++;
          moved = true;
        }
      }
    }

    if (wrap === true) {
        cmap[h-1][x] = '.';
        cmap[0][x] = 'v';
        moved = true;
    }
  }

  step++;
  if (moved === false) {
    console.log("No Moves!\n Steps: ", step)
    break;
  }
}
