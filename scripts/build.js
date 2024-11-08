import { spawn } from "node:child_process";
import { performance } from "perf_hooks";

/**
 * Formats the duration in "HH:MM:SS.FFFF" format, showing only the necessary parts.
 * @param {number} duration - The duration in milliseconds.
 * @returns {string} - The formatted duration.
 */
function formatDuration(duration) {
    const milliseconds = duration % 1000;
    const totalSeconds = Math.floor(duration / 1000);
    const seconds = totalSeconds % 60;
    const totalMinutes = Math.floor(totalSeconds / 60);
    const minutes = totalMinutes % 60;
    const hours = Math.floor(totalMinutes / 60);

    const parts = [
        hours ? `${String(hours).padStart(2, '0')}hr ` : "",
        minutes || hours ? `${String(minutes).padStart(2, '0')}m ` : "",
        `${String(seconds).padStart(2, '0')}s `,
        `${String(milliseconds).padStart(4, '0')}ms`
    ];

    return parts.join('');
}

/**
 * Runs a given npm script and returns a promise that resolves when the script completes.
 * @param {string} scriptName - The name of the npm script to run.
 * @returns {Promise<void>}
 */
function runScript(scriptName) {
    return new Promise((resolve, reject) => {
        const process = spawn('npm', ['run', scriptName], { shell: true });

        process.stdout.on('data', (data) => {
            process.stdout.write(data);
        });

        process.stderr.on('data', (data) => {
            process.stderr.write(data);
        });

        process.on('close', (code) => {
            if (code === 0) {
                resolve();
            } else {
                reject(new Error(`Script "${scriptName}" exited with code ${code}`));
            }
        });
    });
}

async function build() {
    const start = performance.now();

    try {
        await runScript('build-frontend');
        await runScript('build-api');
        const end = performance.now();
        const buildTime = end - start;
        const formattedDuration = formatDuration(buildTime);
        console.log(`Build completed successfully in ${formattedDuration}`);
    } catch (error) {
        const end = performance.now();
        const buildTime = end - start;
        const formattedDuration = formatDuration(buildTime);
        console.error(`Build process failed: ${error.message} after ${formattedDuration}`);
        process.exit(1);
    }
}

build();