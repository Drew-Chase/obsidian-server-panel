import {Input, Listbox, ListboxItem, Select, SelectItem} from "@nextui-org/react";
import MagnifyGlass from "../images/MagnifyGlass.svg.tsx";
import DiscoverModItem from "../components/Server/DiscoverModItem.tsx";
import icon from "../images/demo/test-mod-icon.webp";
import {setTitle} from "../../main.tsx";

export default function DiscoverModpacks()
{
    setTitle("Discover Modpacks");
    return (

        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4"}>
            <div className={"flex flex-col"}>
                <div className={"flex flex-row items-center gap-4"}>
                    <h2 className={"text-2xl font-semibold text-nowrap"}>Discover Modpacks</h2>
                    <Input
                        label={"Search"}
                        placeholder={"Search for mods"}
                        startContent={<MagnifyGlass/>}
                        className={"w-full"}
                        classNames={{
                            inputWrapper: "bg-neutral-700"
                        }}
                    />
                    <Select
                        label={"Source"}
                        disallowEmptySelection
                        defaultSelectedKeys={["all"]}
                        className={"w-[400px]"}
                        classNames={{
                            trigger: "bg-neutral-700"
                        }}
                    >
                        <SelectItem key={"all"}>All</SelectItem>
                        <SelectItem key={"modrinth"}>Modrinth</SelectItem>
                        <SelectItem key={"curseforge"}>CurseForge</SelectItem>
                    </Select>
                    <Select
                        label={"Sort By"}
                        disallowEmptySelection
                        defaultSelectedKeys={["relevance"]}
                        className={"w-[400px]"}
                        classNames={{
                            trigger: "bg-neutral-700"
                        }}
                    >
                        <SelectItem key={"relevance"}>Relevance</SelectItem>
                        <SelectItem key={"downloads"}>Download count</SelectItem>
                        <SelectItem key={"published"}>Published Date</SelectItem>
                        <SelectItem key={"updated"}>Updated Date</SelectItem>
                    </Select>
                </div>
                <div className={"flex flex-col gap-4 mt-4 rounded-lg bg-neutral-800 p-4 overflow-y-auto max-h-[calc(100dvh_-_200px)] h-screen"}>
                    <Listbox
                        aria-label="Discover Modpacks List"
                        itemClasses={{
                            base: "data-[hover]:bg-neutral-600"
                        }}
                    >
                        {Array.from({length: 10}).map((_, i) => (
                            <ListboxItem key={i} textValue={`Mod Item ${i + 1}`}>
                                <DiscoverModItem icon={icon} name={"Sodium"} author={"jellysquid3"} description={"The fastest and most compatible rendering optimization mod for Minecraft. Now available for both NeoForge and Fabric!"}/>
                            </ListboxItem>
                        ))}
                    </Listbox>
                </div>
            </div>
        </div>
    );
}