function depth_scan(depths) {
  let count = 0;
  for (let i=1; i<depths.length; i++) {
    if (depths[i] > depths[i-1]) {
      count++;
    }
  }

  return count;
}

function sliding_window(depths) {
  const result = [];
  for (let i=1; i<depths.length-1; i++) {
    result.push(depths[i-1] + depths[i] + depths[i+1])
  }

  return result;
}

const fs = require('fs');
const text = fs.readFileSync("input.txt").toString('utf-8');
let lines = text.split('\n');
lines = lines.filter((x) => {return !isNaN(parseInt(x))});
const depths = lines.map(Number)

r1 = depth_scan(depths)
r2 = depth_scan(sliding_window(depths))
console.log(`Result part-1: ${r1}`);
console.log(`Result part-2: ${r2}`);
