import {buildBackend, buildFrontend, cleanup, deploy, incrementVersion} from "./app.js";

console.log("Packaging...");
cleanup();
buildFrontend();
buildBackend();
await deploy();
cleanup();