import {AutocompleteItem, Button, Tab, Tabs, Tooltip} from "@nextui-org/react";
import OAutocomplete from "../../Extends/OAutocomplete.tsx";
import {useEffect, useState} from "react";
import {getFabricVersions, getForgeVersions} from "../../../ts/loaders.ts";

export default function LoaderSettings({minecraft_version, snapshots}: { minecraft_version: string, snapshots: boolean })
{

    const [versions, setVersions] = useState<string[]>([]);
    const [loader, setLoader] = useState<string>("Vanilla");
    const [forgeOnly, setForgeOnly] = useState<boolean>(false);
    useEffect(() =>
    {
        switch (loader)
        {
            case "Vanilla":
                break;
            case "Fabric":
                getFabricVersions().then(setVersions);
                break;
            case "Forge":
                getForgeVersions(minecraft_version).then(setVersions);
                break;
            case "NeoForge":
                break;
            case "Quilt":
                break;
        }
        if (minecraft_version !== "")
        {
            const parts: number[] = minecraft_version.split(".").map((part) => parseInt(part));
            setForgeOnly(parts.length >= 2 && parts[1] <= 14);
        } else
        {
            setForgeOnly(false);
        }

    }, [minecraft_version, loader]);


    return (
        <div>
            <p>Select Loader</p>
            <Tabs onSelectionChange={key => setLoader(key as string)} isDisabled={minecraft_version == ""}>
                <Tab title={"Vanilla"}></Tab>
                <Tab title={"Fabric"} key={"Fabric"} isDisabled={forgeOnly}>
                    <div className={"flex flex-row w-full gap-4 items-center"}>
                        <OAutocomplete
                            label={"Fabric Loader Version"}
                            placeholder={"Select a Fabric Loader version"}
                            className={"w-full"}
                        >
                            {versions.map((version) => (
                                <AutocompleteItem key={version}>{version}</AutocompleteItem>
                            ))}
                        </OAutocomplete>
                        <Tooltip content={"This is required for most fabric mods"}>
                            <Button>Install Fabric API</Button>
                        </Tooltip>
                    </div>
                </Tab>
                <Tab title={"Forge"} key={"Forge"} isDisabled={snapshots}>
                    <OAutocomplete
                        label={"Forge Version"}
                        placeholder={"Select a Forge version"}
                        className={"w-full"}
                    >
                        {versions.map((version) => (
                            <AutocompleteItem key={version}>{version}</AutocompleteItem>
                        ))}
                    </OAutocomplete>
                </Tab>
                <Tab title={"NeoForge"} key={"NeoForge"} isDisabled={snapshots || forgeOnly}>
                    <OAutocomplete
                        label={"NeoForge Version"}
                        placeholder={"Select a NeoForge version"}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </OAutocomplete>
                </Tab>
                <Tab title={"Quilt"} key={"Quilt"} isDisabled={forgeOnly}>
                    <OAutocomplete
                        label={"Quilt Version"}
                        placeholder={"Select a Quilt version"}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </OAutocomplete>
                </Tab>
            </Tabs>
        </div>
    );
}