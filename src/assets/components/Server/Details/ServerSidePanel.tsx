import MemoryAllocationInput from "./MemoryAllocationInput.tsx";
import StopButton from "./StopButton.tsx";
import {AutocompleteItem, Button, Link, Tooltip} from "@nextui-org/react";
import DownloadFile from "../../../images/DownloadFile.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArchive, faEdit, faLayerGroup} from "@fortawesome/free-solid-svg-icons";
import OInput from "../../Extends/OInput.tsx";
import {useEffect, useState} from "react";
import ServerSettings from "../../../ts/server-settings.ts";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";

export default function ServerSidePanel()
{
    const [settings, setSettings] = useState<ServerSettings | null>(null);
    const {server} = useSelectedServer();

    useEffect(() =>
    {
        if (server) server?.settings().then(setSettings);
    }, [server]);


    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md w-full mx-2 gap-4"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Actions</p>
            </div>
            <StopButton/>

            <OInput
                label={"Server Name"}
                placeholder={"My Server"}
                value={settings?.name}
                isRequired
            />

            <MemoryAllocationInput/>
            <OInput
                label={"Minecraft Arguments"}
                placeholder={"nogui"}
                value={settings?.minecraft_arguments}
                description={<p>These arguments are passed to the Minecraft server when starting, for more information please visit <Link className={"text-tiny"} href={"https://minecraft.wiki/w/Tutorials/Setting_up_a_server#Minecraft_options"} target={"_blank"}>Minecraft's Documentation</Link>.</p>}
            />
            <OInput
                label={"Additional Java Arguments"}
                placeholder={"-XX:+DisableExplicitGC"}
                value={settings?.java_arguments}
                description={<p>These arguments are passed to the JVM when starting the server, for more information please visit <Link className={"text-tiny"} href={"https://docs.oracle.com/en/java/javase/17/docs/specs/man/java.html"} target={"_blank"}>Java's Documentation</Link>.</p>}
            />

            <OInput
                label={"Runnable Jar"}
                placeholder={"Select a runnable jar"}
                value={settings?.executable}
                isReadOnly
                endContent={
                    <Tooltip content={"Set the servers start executable."}>
                        <Button variant={"light"} className={"min-w-0"} as={Link} href={`/app/server/files/?filter=${encodeURIComponent(".jar,.sh,.bat")}`}>
                            <FontAwesomeIcon icon={faEdit}/>
                        </Button>
                    </Tooltip>
                }
            >
                <AutocompleteItem key={"server.jar"} value={"server.jar"}>server.jar</AutocompleteItem>

            </OInput>

            <Tooltip content={"This will zip the entire server and download it."}>
                <Button endContent={<DownloadFile/>}><span className={"w-full"}>Download Server</span></Button>
            </Tooltip>
            <Tooltip content={"This will archive the server and remove it from the server list."}>
                <Button endContent={<FontAwesomeIcon icon={faArchive}/>}><span className={"w-full"}>Archive Server</span></Button>
            </Tooltip>
            <Tooltip content={"This will create a new server instance from the current server."}>
                <Button endContent={<FontAwesomeIcon icon={faLayerGroup}/>}><span className={"w-full"}>Create Instance from Server</span></Button>
            </Tooltip>


        </div>
    );
}