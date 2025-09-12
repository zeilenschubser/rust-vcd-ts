import initSync, { load_vcd_by_filename, VCDFile, VCDFileError } from "rust-vcd-ts";

const loadFromVcdFast = async (vcdFile: string | undefined): Promise<string[]> => {
  if (vcdFile) {
    try {
      const trace: VCDFile | VCDFileError = load_vcd_by_filename(vcdFile) || { filename: vcdFile, value_map: {}, variable_map: {} };
      console.log(trace.filename, trace.variable_map, trace.value_map);
      return ["ok"]
    } catch (err) {
      const errx = err as VCDFileError;
      console.error("FAIL: ", errx)
      return [];
    }
  }
  return [];
}

interface CMDLineArgs {
  mode: "fast" | "slow";
  vcdFile: string | undefined;
}

function main(args: CMDLineArgs) {
  initSync().then(loaded => {
    if (args.mode === "fast") {
      loadFromVcdFast(args.vcdFile).then(ret => {
        console.log("ret from fast mode:", ret);
      });
    } else {
      console.log("todo")
    }
  });
}
// check if running single file
if (require.main === module) {
  const workspaceFolder = process.argv[1].split("/").slice(0, -1).join("/"); // returns .../test
  console.log(workspaceFolder)
  const args = process.argv.slice(2); // Remove 'bun' and script path
  const fast = args[0] === '--fast';
  main({
    mode: fast ? "fast" : "slow",
    vcdFile: `${workspaceFolder}/test.vcd`
  });
}