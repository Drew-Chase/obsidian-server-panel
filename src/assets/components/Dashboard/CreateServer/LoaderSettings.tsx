import {AutocompleteItem, Button, Tab, Tabs} from "@nextui-org/react";
import OAutocomplete from "../../Extends/OAutocomplete.tsx";
import {useEffect, useState} from "react";
import {getLoaderVersions, getSupportedLoaders} from "../../../ts/loaders.ts";
import OTooltip from "../../Extends/OTooltip.tsx";


export default function LoaderSettings({minecraft_version, snapshots, onLoaderChange}: { minecraft_version: string, snapshots: boolean, onLoaderChange: (loader: string, version: string) => void })
{
    const [versions, setVersions] = useState<string[]>([]);
    const [loader, setLoader] = useState<string>("Vanilla");
    const [loaderVersion, setLoaderVersion] = useState<string>("");
    const [supportedLoaders, setSupportedLoaders] = useState<string[]>([]);
    useEffect(() =>
    {
        getSupportedLoaders(minecraft_version || "all", snapshots).then(setSupportedLoaders);
        if (loader && minecraft_version)
            getLoaderVersions(loader, minecraft_version).then(setVersions);
    }, [minecraft_version, loader]);

    useEffect(() =>
    {
        onLoaderChange(loader, loaderVersion);
    }, [loader, loaderVersion]);

    return (
        <div>
            <p>Select Loader</p>
            <Tabs onSelectionChange={key => setLoader(key as string)} isDisabled={minecraft_version == ""}>
                {supportedLoaders.map((supportedLoader) => (
                    <Tab key={supportedLoader} value={supportedLoader} title={supportedLoader}>
                        <div className={"flex flex-row w-full gap-4 items-center"} hidden={supportedLoader === "Vanilla"}>
                            <OAutocomplete
                                label={`${supportedLoader} Loader Version`}
                                placeholder={`Select a ${supportedLoader} Loader version`}
                                className={"w-full"}
                                onSelectionChange={(key) => setLoaderVersion(key as string)}
                            >
                                {versions.map((version) => (
                                    <AutocompleteItem key={version}>{version}</AutocompleteItem>
                                ))}
                            </OAutocomplete>
                            {supportedLoader === "Fabric" &&
                                <OTooltip content={"This is required for most fabric mods"}>
                                    <Button>Install Fabric API</Button>
                                </OTooltip>
                            }
                        </div>
                    </Tab>

                ))}
            </Tabs>
        </div>
    );
}