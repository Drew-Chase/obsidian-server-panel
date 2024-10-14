import {spawn} from "node:child_process";

/**
 * Tracks if any data has been received from the process.
 * @type {boolean}
 */
let first_data = false;

/**
 * Spawns the API server process using npm to run the 'watch-api' script.
 * @type {ChildProcessWithoutNullStreams}
 */
let apiServerProcess = spawn('npm', ['run', 'watch-api'], {shell: true});

/**
 * Handles the 'data' event for the standard output stream of the API server process.
 * @param {Buffer} data - The data received from the standard output stream.
 */
apiServerProcess.stdout.on('data', (data) => {
    if (data && !first_data) first_data = true;
    process.stdout.write(data);
    apiServerProcess.stdout.on('data', (data) => {
        process.stdout.write(data);
        debug(data.toString());
    });
});

/**
 * Waits until the first data is received from the API server process.
 * If the first data is not received, it logs 'waiting for first data' every second.
 */
while (!first_data) {
    console.log('waiting for first data');
    // add a delay to allow the process to start
    await new Promise(resolve => setTimeout(resolve, 1000));
}

/**
 * Handles the 'data' event for the standard error stream of the API server process.
 * @param {Buffer} data - The data received from the standard error stream.
 */
apiServerProcess.stderr.on('data', (data) => {
    process.stderr.write(data);
    debug(data.toString());
});

/**
 * Handles the 'close' event for the API server process, logging and exiting if it fails.
 * @param {number} code - The exit code of the process.
 */
apiServerProcess.on('close', (code) => {
    if (code !== 0) {
        console.error(`API server process exited with code ${code}`);
        process.exit(1);
    }
});

/**
 * Spawns the Vite server process.
 * @type {ChildProcessWithoutNullStreams}
 */
let viteServerProcess = spawn('vite', ['.'], {shell: true});

/**
 * Handles the 'close' event for the Vite server process, logging and exiting if it fails.
 * @param {number} code - The exit code of the process.
 */
viteServerProcess.on('close', (code) => {
    if (code !== 0) {
        console.error(`Vite server process exited with code ${code}`);
        process.exit(1);
    }
});

/**
 * Logs formatted debug information based on the log pattern provided.
 * @param {string} data - The data to be logged.
 */
function debug(data) {
    const logData = data.toString();
    const logPattern = /\[(.*?)\s(\w+)\s+(.*?)\] (.*)/;
    const match = logData.match(logPattern);

    if (match) {
        const [, timestamp, logLevel, source, message] = match;
        let style = 'color: blue; font-weight: bold;';

        switch (logLevel) {
            case 'TRACE':
                style = 'color: gray;';
                break;
            case 'DEBUG':
                style = 'color: green;';
                break;
            case 'INFO':
                style = 'color: blue;';
                break;
            case 'WARN':
                style = 'color: orange;';
                break;
            case 'ERROR':
                style = 'color: red;';
                break;
        }

        console.log(`%c[${timestamp} ${logLevel} ${source}]%c ${message}`, style, 'color: white;');
    }
}