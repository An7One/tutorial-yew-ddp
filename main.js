import init, { run_app } from "./pkg/tutorial_yew_ddp.js";
async function main() {
  await init("/pkg/tutorial_yew_ddp_bg.wasm");
  run_app();
}
main();
