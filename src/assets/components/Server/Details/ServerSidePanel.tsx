import MemoryAllocationInput from "./MemoryAllocationInput.tsx";
import StopButton from "./StopButton.tsx";
import {AutocompleteItem, Button, Link, Tooltip} from "@nextui-org/react";
import DownloadFile from "../../../images/DownloadFile.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArchive, faLayerGroup} from "@fortawesome/free-solid-svg-icons";
import OInput from "../../Extends/OInput.tsx";
import OAutocomplete from "../../Extends/OAutocomplete.tsx";

export default function ServerSidePanel()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md w-full mx-2 gap-4"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Actions</p>
            </div>
            <StopButton/>

            <OInput
                label={"Server Name"}
                placeholder={"My Server"}
                isRequired
            />

            <MemoryAllocationInput/>
            <OInput
                label={"Minecraft Arguments"}
                placeholder={"nogui"}
                description={<p>These arguments are passed to the Minecraft server when starting, for more information please visit <Link className={"text-tiny"} href={"https://minecraft.wiki/w/Tutorials/Setting_up_a_server#Minecraft_options"} target={"_blank"}>Minecraft's Documentation</Link>.</p>}
            />
            <OInput
                label={"Additional Java Arguments"}
                placeholder={"-XX:+DisableExplicitGC"}
                description={<p>These arguments are passed to the JVM when starting the server, for more information please visit <Link className={"text-tiny"} href={"https://docs.oracle.com/en/java/javase/17/docs/specs/man/java.html"} target={"_blank"}>Java's Documentation</Link>.</p>}
            />

            <OAutocomplete
                label={"Runnable Jar"}
                placeholder={"Select a runnable jar"}
            >
                <AutocompleteItem key={"server.jar"} value={"server.jar"}>server.jar</AutocompleteItem>

            </OAutocomplete>

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