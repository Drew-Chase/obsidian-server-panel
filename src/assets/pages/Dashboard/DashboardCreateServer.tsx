import {Button} from "@nextui-org/react";
import {useState} from "react";
import {setTitle} from "../../../main.tsx";
import ServerInfo from "../../components/Dashboard/CreateServer/ServerInfo.tsx";
import ServerSettings from "../../components/Dashboard/CreateServer/ServerSettings.tsx";
import AdvancedSettings from "../../components/Dashboard/CreateServer/AdvancedSettings.tsx";
import VersionSettings from "../../components/Dashboard/CreateServer/VersionSettings.tsx";
import UploadIcon from "../../components/Dashboard/CreateServer/UploadIcon.tsx";

export default function DashboardCreateServer()
{
    setTitle("Create Server");
    const [serverName, setServerName] = useState<string>("");
    const [serverPort, setServerPort] = useState<string>("25565");
    const [serverDifficulty, setServerDifficulty] = useState<string>("easy");
    const [serverGamemode, setServerGamemode] = useState<string>("survival");
    const [serverMaxPlayers, setServerMaxPlayers] = useState<number>(20);
    const [hardcoreMode, setHardcoreMode] = useState<boolean>(false);
    const [portError, setPortError] = useState<string | null>(null);

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 max-h-[calc(100dvh_-_60px)] h-dvh overflow-y-auto gap-4"}>
            <p className={"text-lg font-semibold"}>Create Server</p>
            <UploadIcon/>
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
            <VersionSettings/>
            <Button color={"primary"} className={"mt-4 shrink-0"}>Create Server</Button>
        </div>
    );
}