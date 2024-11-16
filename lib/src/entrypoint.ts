/**
 * File used as entrypoint for the application
 * Made primarily for the purpose of fixing the dll issue on windows
 * -> Sets the root path as the one with the dlls
 */

import path from "path";

// Get the current path and a path to the main file
const project_path = process.cwd();
const file_path = path.join(project_path, process.argv[2]);

// Get the path to the lib folder (where the dlls are)
const c_path = import.meta.dirname;
const lib_path = path.join(c_path, "lib");

// Run the file with the dlls folder as the root path
Bun.spawnSync({
  cmd: ["bun", "run", file_path],
  cwd: lib_path,
  stdio: ["inherit", "inherit", "inherit"],
  env: {
    // Set the environment variables
    ...process.env,
    //GTK_CSD: "0",
  },
});
