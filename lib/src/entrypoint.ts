import path from "path";

const project_path = process.cwd();
const file_path = path.join(project_path, process.argv[2]);

const c_path = import.meta.dirname;
const lib_path = path.join(c_path, "lib");

const proc = Bun.spawnSync({
  cmd: ["bun", "run", file_path],
  cwd: lib_path,
  stdio: ["inherit", "inherit", "inherit"],
  env: {
    ...process.env,
    GTK_CSD: "0",
  },
});
