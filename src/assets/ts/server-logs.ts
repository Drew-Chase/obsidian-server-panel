import Server from "./servers.ts";

export async function getServerLogFiles(id: string): Promise<string[]>
{
    // Fetch the server instance by its identifier
    let server = await Server.get(id);

// If the server instance is not found, return an empty array
    if (!server) return [];

// Retrieve the list of files in the "/logs/" directory from the server's filesystem
    const logFiles = await server.filesystem().files("/logs/");

// Filter the retrieved files to include only those of type "Log File",
// Then map the filtered entries to return an array of log file names
    return logFiles.entries
        .filter(log => log.type === "Log File") // Keep only log files
        .map(log => log.name); // Extract and return the names of the log files
}