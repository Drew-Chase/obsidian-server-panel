import {Button, Listbox, ListboxItem, ScrollShadow, Tooltip} from "@nextui-org/react";
import ExtendedSwitch from "../Extends/ExtendedSwitch.tsx";
import DownloadFile from "../../images/DownloadFile.svg.tsx";

export default function MinecraftVersionsList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-h-[400px] h-dvh overflow-y-auto"}>
            <div className={"flex flex-row"}>

                <p className={"text-lg font-semibold mr-auto"}>Minecraft Versions</p>
                <ExtendedSwitch
                    label={"Include Snapshots"}
                />
            </div>

            <div className={"flex flex-row w-full text-tiny my-4"}>
                <p className={"mr-auto"}>Version / Type</p>
                <p>Actions</p>
            </div>
            <ScrollShadow className={"max-h-[300px] h-[400px] overflow-y-auto"}>
                <Listbox aria-label="Minecraft version list">
                    {Array.from({length: 10}, (_, i) => (
                        <ListboxItem
                            key={i}
                            title={
                                <div className={"flex flex-row items-center gap-2"}>
                                    <p className={"max-w-[90px] truncate"}>1.20.4</p>
                                </div>
                            }
                            description={i === 0 ? "latest release" : i === 1 ? "latest snapshot" : "release"}
                            endContent={
                                <div className={"flex flex-row"}>
                                    <Tooltip content={"Switch to this version."}>
                                        <Button aria-label={`Switch to version ${i === 0 ? "latest release" : i === 1 ? "latest snapshot" : "release"}`} variant={"light"} size={"sm"} className={"min-w-0"}><DownloadFile/></Button>
                                    </Tooltip>
                                </div>
                            }
                            textValue={`Version ${i === 0 ? "latest release" : i === 1 ? "latest snapshot" : "release"}`}
                        />
                    ))}
                </Listbox>
            </ScrollShadow>
        </div>
    );
}