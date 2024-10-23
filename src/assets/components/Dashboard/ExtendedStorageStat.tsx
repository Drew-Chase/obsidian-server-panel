import {Button, CircularProgress, Listbox, ListboxItem, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArrowDown, faCircle, faCloudDownload} from "@fortawesome/free-solid-svg-icons";
import {toast} from "sonner";
import {HTMLAttributes} from "react";

export default function ExtendedStorageStat(props: HTMLAttributes<any>)
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 xl:max-w-md w-full max-w-full grow shrink h-lg"} aria-label="Extended Storage Statistic" {...props}>
            <div className={"flex flex-row w-full items-center"}>
                <p className={"text-lg font-semibold"}>Storage Usage</p>
                <Tooltip content={"Export a spreadsheet csv with a breakdown of your storage consumption."} aria-label="Export Tooltip">
                    <Button
                        endContent={<FontAwesomeIcon icon={faArrowDown}/>}
                        className={"ml-auto"}
                        aria-label="Export to CSV"
                        onClick={() => toast("Exported CSV file.", {description: "A CSV file has been exported with a breakdown of your storage consumption.", icon: <FontAwesomeIcon icon={faCloudDownload}/>})}
                    >
                        Export
                    </Button>
                </Tooltip>
            </div>
            <div className={"relative w-[300px] h-[300px] mx-auto my-4"} aria-label="Storage Usage Circular Progress">
                <p className={"absolute text-4xl font-semibold top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/6"}>800GB</p>
                <div className={"-rotate-90 -translate-x-[165px] translate-y-[165px]"}>
                    <CircularProgress
                        value={50}
                        maxValue={100}
                        className={"mt-4 absolute left-0 translate-y-[45px] translate-x-[45px]"}
                        size={"sm"}
                        classNames={{
                            svg: "w-[210px] h-[210px]",
                            track: "stroke-transparent",
                            indicator: "stroke-[#00C2FF]"
                        }}
                        aria-label="Blue progress indicator"
                    />
                    <CircularProgress
                        value={60}
                        maxValue={100}
                        className={"mt-4 absolute left-0 translate-y-[25px] translate-x-[25px]"}
                        size={"sm"}
                        classNames={{
                            svg: "w-[250px] h-[250px]",
                            track: "stroke-transparent",
                            indicator: "stroke-[#0E43FB]"
                        }}
                        aria-label="Dark blue progress indicator"
                    />
                    <CircularProgress
                        value={80}
                        maxValue={100}
                        className={"mt-4 absolute left-0"}
                        size={"sm"}
                        classNames={{
                            svg: "w-[300px] h-[300px]",
                            track: "stroke-transparent"
                        }}
                        aria-label="Primary progress indicator"
                    />
                </div>
            </div>
            <Listbox className={"mt-4"} aria-label="Storage Usage Listbox">
                <ListboxItem
                    key={"primary"}
                    startContent={
                        <FontAwesomeIcon icon={faCircle} width={8} color={"#CB3CFF"}/>
                    }
                    endContent={<p className={"text-tiny opacity-70"}> 400GB - 50%</p>}
                    aria-label="SMP Server storage usage"
                >
                    <span className={"text-neutral-400"}>SMP Server</span>
                </ListboxItem>
                <ListboxItem
                    key={"secondary"}
                    startContent={
                        <FontAwesomeIcon icon={faCircle} width={8} color={"#0E43FB"}/>
                    }
                    endContent={<p className={"text-tiny opacity-70"}>400GB - 50%</p>}
                    aria-label="All The Mods 6 storage usage"
                >
                    <span className={"text-neutral-400"}>All The Mods 6</span>
                </ListboxItem>
                <ListboxItem
                    key={"other"}
                    startContent={
                        <FontAwesomeIcon icon={faCircle} width={8} color={"#00C2FF"}/>
                    }
                    endContent={<p className={"text-tiny opacity-70"}>400GB - 50%</p>}
                    aria-label="Other storage usage"
                >
                    <span className={"text-neutral-400"}>Other</span>
                </ListboxItem>
            </Listbox>
        </div>
    );
}
