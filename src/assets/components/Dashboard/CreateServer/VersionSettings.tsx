import {AutocompleteItem} from "@nextui-org/react";
import ExtendedSwitch from "../../Extends/ExtendedSwitch.tsx";
import OAutocomplete from "../../Extends/OAutocomplete.tsx";
import {useEffect, useState} from "react";
import MinecraftVersions, {MinecraftVersion} from "../../../ts/mincraft-versions.ts";
import LoaderSettings from "./LoaderSettings.tsx";

export default function VersionSettings()
{
    const [versions, setVersions] = useState<MinecraftVersion[]>([]);
    const [showSnapshots, setShowSnapshots] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [minecraftVersion, setMinecraftVersion] = useState<string>("");

    useEffect(() =>
    {
        setIsLoading(true);
        if (showSnapshots)
        {

            MinecraftVersions
                .versions()
                .then((versions) => setVersions(versions))
                .finally(() => setIsLoading(false));
        } else
        {
            MinecraftVersions
                .releases()
                .then((versions) => setVersions(versions))
                .finally(() => setIsLoading(false));
        }
    }, [showSnapshots]);
    return (
        <>
            <OAutocomplete
                label={"Minecraft Version"}
                placeholder={"Select a Minecraft version"}
                isLoading={isLoading}
                onSelectionChange={key =>
                {
                    if (key)
                        setMinecraftVersion(key as string);
                }}
                value={minecraftVersion}
            >
                {versions.map((version) => (
                    <AutocompleteItem key={version.id}>{version.id}</AutocompleteItem>
                ))}
            </OAutocomplete>
            <ExtendedSwitch
                label={"Show Snapshots"}
                description={"Show snapshots in the version list"}
                className={"max-w-full"}
                toggle={showSnapshots}
                onToggle={setShowSnapshots}
            />

            <LoaderSettings minecraft_version={minecraftVersion} snapshots={showSnapshots}/>
        </>
    );
}