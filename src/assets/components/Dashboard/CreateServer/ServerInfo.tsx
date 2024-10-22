import OInput from "../../Extends/OInput.tsx";

interface ServerInfoProps
{
    serverName: string;
    setServerName: (value: string) => void;
    serverPort: string;
    setServerPort: (value: string) => void;
    portError: string | null;
    setPortError: (value: string | null) => void;
}

export default function ServerInfo({serverName, setServerName, serverPort, setServerPort, portError, setPortError}: ServerInfoProps)
{
    return (
        <>
            <OInput
                label="Server Name"
                placeholder="Enter a name for your server"
                value={serverName}
                onValueChange={setServerName}
                isRequired
            />
            <OInput
                label="Server Port"
                placeholder="Enter a port for your server"
                value={serverPort}
                onValueChange={e =>
                {
                    const portString = e.replace(/\D/g, "");
                    const minPort = 1;
                    const maxPort = 65535;
                    setServerPort(portString);
                    if (portString === "" || (parseInt(portString) >= minPort && parseInt(portString) <= maxPort))
                    {
                        setPortError(null);
                    } else
                    {
                        setPortError("Invalid port number, please enter a valid port number between 1 and 65535");
                    }
                }}
                errorMessage={portError}
                isInvalid={portError !== null}
                isRequired
            />
        </>
    );
}