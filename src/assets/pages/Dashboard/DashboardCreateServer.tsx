import {Button} from "@nextui-org/react";
import {useEffect, useState} from "react";
import {setTitle} from "../../../main.tsx";
import ServerInfo from "../../components/Dashboard/CreateServer/ServerInfo.tsx";
import ServerSettings from "../../components/Dashboard/CreateServer/ServerSettings.tsx";
import AdvancedSettings from "../../components/Dashboard/CreateServer/AdvancedSettings.tsx";
import VersionSettings from "../../components/Dashboard/CreateServer/VersionSettings.tsx";
import UploadIcon from "../../components/Dashboard/CreateServer/UploadIcon.tsx";
import Server from "../../ts/servers.ts";
import FileSystem from "../../ts/file-system.ts";
import {useAlertModal} from "../../providers/AlertModalProvider.tsx";
import Java from "../../ts/java.ts";
import {useNavigate} from "react-router-dom";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";

export default function DashboardCreateServer()
{
    setTitle("Create Server");
    const [serverName, setServerName] = useState<string>("");
    const [serverPort, setServerPort] = useState<string>("25565");
    const [serverDifficulty, setServerDifficulty] = useState<string>("easy");
    const [serverGamemode, setServerGamemode] = useState<string>("survival");
    const [serverMaxPlayers, setServerMaxPlayers] = useState<number>(20);
    const [hardcoreMode, setHardcoreMode] = useState<boolean>(false);
    const [minecraftVersion, setMinecraftVersion] = useState<string>("");
    const [loader, setLoader] = useState<string>("vanilla");
    const [loaderVersion, setLoaderVersion] = useState<string>("");
    const [portError, setPortError] = useState<string | null>(null);
    const [serverIcon, setServerIcon] = useState<File | null>(null);
    const [isValid, setIsValid] = useState<boolean>(false);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const {alert} = useAlertModal();
    const {setSelectedServerId} = useSelectedServer();
    const navigate = useNavigate();


    useEffect(() =>
    {
        const validName = serverName.length > 0;
        const validPort = !isNaN(+serverPort) && +serverPort >= 1 && +serverPort <= 65535;
        const validVersion = minecraftVersion.length > 0;
        const validLoader = loader.length > 0;
        const validLoaderVersion = loader.toLowerCase() === "vanilla" || loaderVersion.length > 0;
        setIsValid(validName && validPort && validVersion && validLoader && validLoaderVersion);
    }, [serverName, serverPort, serverDifficulty, serverGamemode, serverMaxPlayers, hardcoreMode, minecraftVersion, loader, loaderVersion]);


    useEffect(() =>
    {
        Java.installed().then((versions) =>
        {
            if (versions.length === 0)
            {
                alert({
                    title: "Java not installed",
                    message: "Please install Java to create a server.",
                    type: "error",
                    actions: [
                        {
                            label: "Goto Settings",
                            onClick: () =>
                            {
                                navigate("/app/settings/#java-settings");
                            }
                        }
                    ]
                });
            }
        });
    }, []);


    const createServer = async () =>
    {
        if (!isValid) return;
        setIsLoading(true);
        try
        {

            const newlyCreatedServer = await Server.create(serverName, +serverPort, serverDifficulty, serverGamemode, hardcoreMode, serverMaxPlayers, minecraftVersion, loader, loaderVersion);
            if (newlyCreatedServer)
            {
                console.log("Server created successfully!");
                setSelectedServerId(newlyCreatedServer.id);
                if (serverIcon)
                {
                    try
                    {
                        const fileSystem = new FileSystem(newlyCreatedServer.id);
                        await fileSystem.upload(serverIcon, "/", "server-icon.png");
                    } catch (e)
                    {
                        console.error(e);
                        alert({
                            title: "Create Server",
                            message: "An error occurred while uploading the server icon.",
                            type: "error",
                            actions: [
                                {
                                    label: "Close"
                                }
                            ]
                        });
                    }
                }
                navigate(`/app/server/`);
            }
        } catch (e)
        {
            console.error(e);
            alert({
                title: "Create Server",
                message: "An error occurred while creating the server.",
                type: "error",
                actions: [
                    {
                        label: "Close"
                    }
                ]
            });

        }
        setIsLoading(false);
    };

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 max-h-[calc(100dvh_-_130px)] h-dvh overflow-y-auto gap-4"}>
            <p className={"text-lg font-semibold"}>Create Server</p>
            <UploadIcon onUpload={setServerIcon}/>
            <ServerInfo
                serverName={serverName}
                setServerName={setServerName}
                serverPort={serverPort}
                setServerPort={setServerPort}
                portError={portError}
                setPortError={setPortError}
            />
            <ServerSettings
                serverDifficulty={serverDifficulty}
                setServerDifficulty={setServerDifficulty}
                serverGamemode={serverGamemode}
                setServerGamemode={setServerGamemode}
                hardcoreMode={hardcoreMode}
            />
            <AdvancedSettings
                hardcoreMode={hardcoreMode}
                setHardcoreMode={setHardcoreMode}
                serverMaxPlayers={serverMaxPlayers}
                setServerMaxPlayers={setServerMaxPlayers}
            />
            <VersionSettings
                onVersionChange={setMinecraftVersion}
                onLoaderChange={(loader, version) =>
                {
                    setLoader(loader);
                    setLoaderVersion(version);
                }}
            />
            <Button color={"primary"} className={"mt-4 shrink-0"} onClick={createServer} isDisabled={!isValid} isLoading={isLoading}>Create Server</Button>
        </div>
    );
}