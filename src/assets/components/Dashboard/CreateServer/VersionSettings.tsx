import {AutocompleteItem} from "@nextui-org/react";
import ExtendedSwitch from "../../Extends/ExtendedSwitch.tsx";
import OAutocomplete from "../../Extends/OAutocomplete.tsx";
import {useEffect, useState} from "react";
import MinecraftVersions, {MinecraftVersion} from "../../../ts/mincraft-versions.ts";

export default function VersionSettings()
{
    const [versions, setVersions] = useState<MinecraftVersion[]>([]);
    const [showSnapshots, setShowSnapshots] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

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
        </>
    );
}