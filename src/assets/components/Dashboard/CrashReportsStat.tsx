import {Avatar, Listbox, ListboxItem, ScrollShadow} from "@nextui-org/react";
import testImage from "../../images/demo/test-server.png";
import {useState} from "react";
import EditorModal from "../EditorModal.tsx";

interface CrashReportsStatProps
{
    serverName?: string;
}

export default function CrashReportsStat(props: CrashReportsStatProps)
{
    const [isEditorOpen, setIsEditorOpen] = useState(false);
    return (
        <>
            <EditorModal isOpen={isEditorOpen} onClose={() => setIsEditorOpen(false)}/>
            <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-w-md w-full max-h-[400px] h-dvh overflow-y-auto"}>
                <p className={"text-lg font-semibold"}>Crash Reports</p>

                <div className={"flex flex-row w-full text-tiny my-4"}>
                    <p className={"mr-auto"}>Description / Mod / Server</p>
                    <p>Timestamp</p>
                </div>
                <ScrollShadow className={"max-h-[300px] h-[400px] overflow-y-auto"}>
                    <Listbox aria-label={"A list of all recent crash reports"}>
                        {Array.from({length: 10}, (_, i) => (
                            <ListboxItem
                                key={i}
                                startContent={
                                    <div className={"rounded-md bg-neutral-800 p-1"}>
                                        <Avatar src={testImage}/>
                                    </div>
                                }
                                description={props.serverName || "SMP Server"}
                                endContent={<p className={"text-tiny text-nowrap"}>2:36:50 PM</p>}
                                textValue={"Ticking entity"}
                                aria-label={"Ticking entity"}
                                onClick={() => setIsEditorOpen(true)}
                            >
                                <div className={"flex flex-row items-center gap-2"}>
                                    <p className={"max-w-[90px] truncate"}>Ticking entity</p>
                                    <p className={"max-w-[90px] truncate opacity-50 font-light text-tiny"}>carpet-tis-addition-v1.62.0-mc1.20.4.jar</p>
                                </div>
                            </ListboxItem>
                        ))}
                    </Listbox>
                </ScrollShadow>
            </div>
        </>
    );
}
