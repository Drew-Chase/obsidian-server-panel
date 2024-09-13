import {Button, CircularProgress, Listbox, ListboxItem, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArrowDown} from "@fortawesome/free-solid-svg-icons";

export default function ExtendedStorageStat()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-w-md w-full mx-2 h-lg"}>
            <div className={"flex flex-row w-full items-center"}>
                <p className={"text-lg font-semibold"}>Storage Usage</p>
                <Tooltip content={"Export a spreadsheet csv with a breakdown of your storage consumption."}>
                    <Button
                        endContent={<FontAwesomeIcon icon={faArrowDown}/>}
                        className={"ml-auto"}
                    >
                        Export
                    </Button>
                </Tooltip>
            </div>
            <div className={"relative w-[300px] h-[300px] mx-auto my-4"}>
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
                    />
                </div>
            </div>
            <Listbox className={"mt-4"}>
                <ListboxItem
                    key={"primary"}
                    startContent={
                        <svg width="8" height="8">
                            <rect x="0.742188" y="0.84082" width="7" height="7" rx="3.5" fill="#CB3CFF"/>
                        </svg>
                    }
                    endContent={<p className={"text-tiny opacity-70"}> 400GB - 50%</p>}
                >
                    <span className={"text-neutral-400"}>SMP Server</span>
                </ListboxItem>
                <ListboxItem
                    key={"secondary"}
                    startContent={
                        <svg width="8" height="8">
                            <rect x="0.742188" y="0.84082" width="7" height="7" rx="3.5" fill="#0E43FB"/>
                        </svg>
                    }
                    endContent={<p className={"text-tiny opacity-70"}>400GB - 50%</p>}
                >

                    <span className={"text-neutral-400"}>All The Mods 6</span>
                </ListboxItem>
                <ListboxItem
                    key={"other"}
                    startContent={
                        <svg width="8" height="8">
                            <rect x="0.742188" y="0.84082" width="7" height="7" rx="3.5" fill="#00C2FF"/>
                        </svg>
                    }
                    endContent={<p className={"text-tiny opacity-70"}>400GB - 50%</p>}
                >
                    <span className={"text-neutral-400"}>Other</span>
                </ListboxItem>
            </Listbox>
        </div>
    );
}
