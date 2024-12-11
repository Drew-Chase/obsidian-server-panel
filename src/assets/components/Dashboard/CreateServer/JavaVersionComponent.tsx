import {Button} from "@nextui-org/react";
import {JavaVersion} from "../../../ts/java.ts";
import OTooltip from "../../Extends/OTooltip.tsx";

interface JavaVersionComponentProps
{
    version: JavaVersion,
    selected: boolean,
    onInstall: (version: JavaVersion) => void
    onSelect: (version: JavaVersion) => void
}

export default function JavaVersionComponent(props: JavaVersionComponentProps)
{
    return (
        <div className={"flex flex-row w-full h-16 rounded-md data-[installed=true]:bg-neutral-700 bg-neutral-800 items-center px-2"} data-installed={props.version.installed}>
            <p className={"text-neutral-200 font-bold"}>
                <p>{props.version.version} <span className={"text-sm italic opacity-50"}>({props.version.runtime})</span> {props.version.installed && (<span className={"text-sm text-primary italic"}>installed</span>)}</p>
            </p>
            <OTooltip content={!props.version.installed ? `Install Java ${props.version.version} to your server!` : !props.selected ? `Use Java ${props.version.version} on your server!` : `Currently selected Java ${props.version.version}!`}>
                <Button
                    onClick={() =>
                    {
                        if (!props.version.installed) props.onInstall(props.version);
                        props.onSelect(props.version);
                    }}
                    className={"ml-auto w-24"}
                    color={props.selected ? "primary" : "default"}
                    variant={"solid"}
                >
                    {!props.version.installed ? "Install" : props.selected ? "Selected" : "Select"}
                </Button>
            </OTooltip>
        </div>
    );
}