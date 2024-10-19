import {JavaVersion} from "../../ts/java.ts";
import {Button, Tooltip} from "@nextui-org/react";

interface JavaVersionComponentProps
{
    version: JavaVersion,
    onInstall: (version: JavaVersion) => void
    onUninstall: (version: JavaVersion) => void
}

export default function JavaVersionComponent(props: JavaVersionComponentProps)
{
    return (
        <div className={"flex flex-row w-full h-16 rounded-md data-[installed=true]:bg-neutral-700 bg-neutral-800 items-center px-2"} data-installed={props.version.installed}>
            <p className={"text-neutral-200 font-bold"}>{props.version.installed ? props.version.executable : props.version.version}</p>
            <Tooltip content={props.version.installed ? `Uninstall Java ${props.version.version} from your server!` : `Install Java ${props.version.version} to your server!`} closeDelay={0}>
                <Button
                    onClick={() => props.version.installed ? props.onUninstall(props.version) : props.onInstall(props.version)}
                    className={"ml-auto w-24"}
                    color={props.version.installed ? "danger" : "default"}
                    variant={props.version.installed ? "light" : "solid"}
                >
                    {props.version.installed ? "Uninstall" : "Install"}
                </Button>
            </Tooltip>
        </div>
    );
}