import {buildBackend, buildFrontend, cleanup, deploy, incrementVersion} from "./app.js";

console.log("Publishing...");
cleanup();
incrementVersion();
buildFrontend();
buildBackend();
await deploy();
cleanup();