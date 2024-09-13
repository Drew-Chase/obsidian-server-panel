import {execSync} from "node:child_process";
import {copyFileSync, createWriteStream, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import archiver from "archiver";

const outputDirectory = "dist";
let version = "0.0.0";


export function buildFrontend() {
    console.log("Building frontend...");
    execSync("npm run \"build frontend\"", {stdio: "inherit"});
    mkdirSync(`${outputDirectory}/wwwroot`, {recursive: true});
}

export function buildBackend() {
    console.log("Building backend...");
    execSync("cargo build --release", {stdio: "inherit"});
    copyFileSync("target/release/obsidian-server-panel.exe", `${outputDirectory}/obsidian-server-panel.exe`);
}

export function cleanup() {
    console.log("Cleaning up...");
    if(!existsSync(outputDirectory))return;
    rmSync(outputDirectory, {recursive: true});
    mkdirSync(outputDirectory, {recursive: true});
}

export function incrementVersion() {
    console.log("Incrementing version...");
    const pkg = JSON.parse(readFileSync("./package.json", {encoding: "utf-8"}));
    const cargo = readFileSync("./Cargo.toml", {encoding: "utf-8"});
    version = pkg.version;
    // Increment version
    const [major, minor, patch] = version.split(".").map(Number);
    if (patch < 9) {
        version = `${major}.${minor}.${patch + 1}`;
    } else if (minor < 9) {
        version = `${major}.${minor + 1}.0`;
    } else {
        version = `${major + 1}.0.0`;
    }
    console.log(`New version: ${version}`);
    pkg.version = version;

    writeFileSync("./Cargo.toml", cargo.replace(/\nversion = ".*"\n/, `\nversion = "${version}"\n`));
    writeFileSync("./package.json", JSON.stringify(pkg, null, 4));
}

export async function deploy() {
    console.log("Deploying...");
    // Specify the output file
    const output = createWriteStream(`obsidian-server-panel-${version}.zip`);
    const archive = archiver('zip', {
        zlib: {level: 9}  // Set the compression level
    });

    // Listen for the 'close' event
    output.on('close', () => {
        console.log(`Archive created successfully: ${archive.pointer()} total bytes`);
    });

    // Catch errors to avoid crashes
    archive.on('error', (err) => {
        throw err;
    });

    // Pipe the archive data to the file
    archive.pipe(output);

    archive.directory(outputDirectory, '', {name: 'stacked'});

    // Finalize the archive (this is necessary)
    await archive.finalize();
}