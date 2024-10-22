import {AutocompleteItem, Button, Tab, Tabs, Tooltip} from "@nextui-org/react";
import OAutocomplete from "../../Extends/OAutocomplete.tsx";

export default function LoaderSettings()
{
    return (
        <>
            <p>Select Loader</p>
            <Tabs>
                <Tab title={"Vanilla"}></Tab>
                <Tab title={"Fabric"}>
                    <div className={"flex flex-row w-full gap-4 items-center"}>
                        <OAutocomplete
                            label={"Fabric Loader Version"}
                            placeholder={"Select a Fabric Loader version"}
                            className={"w-full"}
                        >
                            <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                        </OAutocomplete>
                        <Tooltip content={"This is required for most fabric mods"}>
                            <Button>Install Fabric API</Button>
                        </Tooltip>
                    </div>
                </Tab>
                <Tab title={"Forge"}>
                    <OAutocomplete
                        label={"Forge Version"}
                        placeholder={"Select a Forge version"}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </OAutocomplete>
                </Tab>
                <Tab title={"NeoForge"}>
                    <OAutocomplete
                        label={"NeoForge Version"}
                        placeholder={"Select a NeoForge version"}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </OAutocomplete>
                </Tab>
                <Tab title={"Quilt"}>
                    <OAutocomplete
                        label={"Quilt Version"}
                        placeholder={"Select a Quilt version"}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </OAutocomplete>
                </Tab>
            </Tabs>
        </>
    );
}