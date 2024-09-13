import {buildBackend, buildFrontend} from "./app.js";
import {execSync} from "node:child_process";

run()

function run() {
    buildFrontend();
    buildBackend();
    // eslint-disable-next-line no-undef
    execSync(`${process.cwd()}/dist/stacked.exe`, {cwd: `${process.cwd()}/dist`, stdio: "inherit"});
}