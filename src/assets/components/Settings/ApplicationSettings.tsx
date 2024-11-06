import ExtendedSwitch from "../Extends/ExtendedSwitch.tsx";
import {Button} from "@nextui-org/react";
import {useEffect, useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faFolderOpen} from "@fortawesome/free-solid-svg-icons";
import OInput from "../Extends/OInput.tsx";
import OTooltip from "../Extends/OTooltip.tsx";

export default function ApplicationSettings()
{
    const [portForward, setPortForward] = useState(false);
    const [portForwardServers, setPortForwardServers] = useState(false);
    const [webUIPort, setWebUIPort] = useState("8080");
    const [serversDirectory, setServersDirectory] = useState("");
    const [backupsDirectory, setBackupsDirectory] = useState("");
    const [javaDirectory, setJavaDirectory] = useState("");

    useEffect(() =>
    {
        // TODO: Load settings
        return () =>
        {
            // TODO: Save settings
        };
    }, []);

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto gap-4"}>
            <p className={"text-lg font-semibold mr-auto mb-8"}>Application Settings</p>
            <div className={"flex flex-row items-center gap-4"}>
                <ExtendedSwitch
                    label={"Port Forward WebUI"}
                    description={"Allow access to the WebUI from outside the local network, IE. the world wide web."}
                    toggle={portForward}
                    onToggle={setPortForward}
                />
                <OInput
                    label={"WebUI Port"}
                    value={webUIPort}
                    isRequired
                    description={"The port the WebUI will be accessible on."}
                    onValueChange={(value) => setWebUIPort(value.replace(/\D/g, ""))}
                    onFocusChange={(focused) =>
                    {
                        if (!focused)
                        {
                            setWebUIPort((+webUIPort).toString());
                        }
                    }}
                    isInvalid={+webUIPort < 1 || +webUIPort > 65535}
                    errorMessage={"Invalid port number. Port must be between 1 and 65535."}
                />
            </div>
            <ExtendedSwitch
                label={"Automatically Port Forward Servers"}
                description={"Automatically port forward servers when they are started."}
                toggle={portForwardServers}
                onToggle={setPortForwardServers}
                classNames={{
                    base: "max-w-full"
                }}
            />
            <OInput
                label={"Servers Directory"}
                value={serversDirectory}
                onValueChange={setServersDirectory}
                isRequired
                description={"The directory where server instances are stored."}
                endContent={
                    <OTooltip content={"Select Directory"}>
                        <Button size={"sm"} className={"min-w-0"}><FontAwesomeIcon icon={faFolderOpen}/></Button>
                    </OTooltip>
                }
            />
            <OInput
                label={"Backups Directory"}
                value={backupsDirectory}
                onValueChange={setBackupsDirectory}
                isRequired
                description={"The directory where server backups are stored."}
                endContent={
                    <OTooltip content={"Select Directory"}>
                        <Button size={"sm"} className={"min-w-0"}><FontAwesomeIcon icon={faFolderOpen}/></Button>
                    </OTooltip>
                }
            />
            <OInput
                label={"Java Directory"}
                value={javaDirectory}
                onValueChange={setJavaDirectory}
                isRequired
                description={"The directory where the Java installations are located."}
                endContent={
                    <OTooltip content={"Select Directory"}>
                        <Button size={"sm"} className={"min-w-0"}><FontAwesomeIcon icon={faFolderOpen}/></Button>
                    </OTooltip>
                }
            />
        </div>
    );
}