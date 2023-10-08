#! /usr/bin/env node
var exec = require("child_process").execSync;

/* 
npx tron create
npx tron build
npx tron start
*/

const args = process.argv.slice(2);
if (args.length < 1) {
  console.log("run default");
} else {
  switch (args[0]) {
    case "create":
      console.log("create default");
      exec(
        `npm install tron-lang & echo 'name = \"TronProject\"\nentry = \"main\"\nversion = \"0.0.1\"\nauthors = \"YOU\"\nlicense = \"MIT\"\ndecor = \"default\"\npointer = \"default\"\nenv = \"prod\"\nexperimental = \"false\"\ncredits = \"false\"\nwarnings = \"true\"' > tron.toml & echo 'print "Hello, World!";' > main.tron`,
        (error, stdout, stderr) => {
          if (error) {
            console.log(`error: ${error.message}`);
            return;
          }
          if (stderr) {
            console.log(`stderr: ${stderr}`);
            return;
          }
          console.log(`stdout: ${stdout}`);
        }
      );

      break;
    case "start":
      exec(
        "cd node_modules/tron-lang \n cargo run",
        (error, stdout, stderr) => {
          if (error) {
            return;
          }
          if (stderr) {
            return;
          }
          console.log(`stdout: ${stdout}`);
        }
      );
      break;
    case "build":
      exec(
        `cd node_modules/tron-lang \n cargo build`,
        (error, stdout, stderr) => {
          if (error) {
            console.log(`error: ${error.message}`);
            return;
          }
          if (stderr) {
            console.log(`stderr: ${stderr}`);
            return;
          }
          console.log(`stdout: ${stdout}`);
        }
      );
      break;
    case "help":
      console.log("help");
      break;
    default:
      console.log("help");
      break;
  }
}

process.exit(0);
