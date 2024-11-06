import {JavaVersion} from "../../ts/java.ts";
import {Button} from "@nextui-org/react";
import OTooltip from "../Extends/OTooltip.tsx";

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
            <p className={"text-neutral-200 font-bold"}>{props.version.installed ? props.version.executable : (<p>{props.version.version} <span className={"text-sm italic opacity-50"}>({props.version.runtime})</span></p>)}</p>
            <OTooltip content={props.version.installed ? `Uninstall Java ${props.version.version} from your server!` : `Install Java ${props.version.version} to your server!`} >
                <Button
                    onClick={() => props.version.installed ? props.onUninstall(props.version) : props.onInstall(props.version)}
                    className={"ml-auto w-24"}
                    color={props.version.installed ? "danger" : "default"}
                    variant={props.version.installed ? "light" : "solid"}
                >
                    {props.version.installed ? "Uninstall" : "Install"}
                </Button>
            </OTooltip>
        </div>
    );
}