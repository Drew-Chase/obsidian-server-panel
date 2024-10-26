import {Button, Listbox, ListboxItem, ScrollShadow, Tooltip} from "@nextui-org/react";
import DownloadFile from "../../../images/DownloadFile.svg.tsx";

export default function LoaderVersionsList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-h-[400px] h-dvh overflow-y-auto"}>
            <p className={"text-lg font-semibold mr-auto"}>Fabric Versions</p>

            <div className={"flex flex-row w-full text-tiny my-4"}>
                <p className={"mr-auto"}>Version / Type</p>
                <p>Actions</p>
            </div>
            <ScrollShadow className={"max-h-[300px] h-[400px] overflow-y-auto"}>
                <Listbox aria-label="List of Fabric Versions">
                    {Array.from({length: 10}, (_, i) => (
                        <ListboxItem
                            key={i}
                            title={
                                <div className={"flex flex-row items-center gap-2"}>
                                    <p className={"max-w-[90px] truncate"}>0.16.5</p>
                                </div>
                            }
                            textValue={`0.16.5 - ${i === 0 ? "latest release" : i === 1 ? "latest snapshot" : "release"}`}
                            description={i === 0 ? "latest release" : i === 1 ? "latest snapshot" : "release"}
                            endContent={
                                <div className={"flex flex-row"}>
                                    <Tooltip content={"Switch to this version."}>
                                        <Button variant={"light"} size={"sm"} className={"min-w-0"} aria-label="Download version"><DownloadFile/></Button>
                                    </Tooltip>
                                </div>
                            }
                        />
                    ))}
                </Listbox>
            </ScrollShadow>
        </div>
    );
}