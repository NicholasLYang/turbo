const { cpSync } = require("fs");
const args = process.argv.slice(2);
const dest = args.pop();
args.forEach((src) => cpSync(src, dest));
