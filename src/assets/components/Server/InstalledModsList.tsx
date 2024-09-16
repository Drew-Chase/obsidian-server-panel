import {Button, Input, Listbox, ListboxItem} from "@nextui-org/react";
import MagnifyGlass from "../../images/MagnifyGlass.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faDownload, faUpload} from "@fortawesome/free-solid-svg-icons";
import InstalledModItem from "./InstalledModItem.tsx";
import icon from "../../images/demo/test-mod-icon.webp";

export default function InstalledModsList()
{
    return (
        <div className={"flex flex-col"}>
            <div className={"flex flex-row items-center gap-4"}>
                <h2 className={"text-2xl font-semibold text-nowrap"}>Installed Mods</h2>
                <Input
                    label={"Search"}
                    placeholder={"Search for mods"}
                    startContent={<MagnifyGlass/>}
                    className={"w-full"}
                    classNames={{
                        inputWrapper: "bg-neutral-700"
                    }}
                />
                <Button startContent={<FontAwesomeIcon icon={faUpload}/>} className={"px-8"}> Upload</Button>
                <Button startContent={<FontAwesomeIcon icon={faDownload}/>} className={"px-8"}> Update all</Button>
            </div>
            <div className={"flex flex-col gap-4 mt-4 rounded-lg bg-neutral-800 p-4 overflow-y-auto max-h-[calc(100dvh_-_240px)] h-screen"}>
                <Listbox
                    itemClasses={{
                        base: "data-[hover]:bg-neutral-600"
                    }}
                >
                    {Array.from({length: 10}).map((_, i) => (
                        <ListboxItem key={i}>
                            <InstalledModItem icon={icon} name={"Sodium"} author={"jellysquid3"} version={"mc1.20.4-0.5.8"}/>
                        </ListboxItem>
                    ))}
                </Listbox>
            </div>
        </div>
    );
}