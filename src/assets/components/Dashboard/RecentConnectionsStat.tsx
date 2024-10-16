import {Avatar, Listbox, ListboxItem, ScrollShadow} from "@nextui-org/react";
import testImage from "../../images/demo/test-server.png";

export default function RecentConnectionsStat()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-w-md w-full mx-2 max-h-[400px] h-dvh overflow-y-auto"}>
            <p className={"text-lg font-semibold"}>Recent Connections</p>

            <div className={"flex flex-row w-full text-tiny my-4"}>
                <p className={"mr-auto"}>Players / Server</p>
                <p>Duration</p>
            </div>
            <ScrollShadow className={"max-h-[300px] h-[400px] overflow-y-auto"}>
                <Listbox aria-label={"A list of all recent connections"}>
                    {Array.from({length: 10}, (_, i) => (
                        <ListboxItem
                            key={i}
                            startContent={
                                <div className={"rounded-md bg-neutral-800 p-1"}>
                                    <Avatar src={testImage}/>
                                </div>
                            }
                            description={"SMP Server"}
                            endContent={<p className={"text-tiny text-nowrap"}>4 Hours</p>}
                            aria-label={"Shroototem"}
                            textValue={"Shroototem"}
                        >
                            <p>Shroototem</p>
                        </ListboxItem>

                    ))}
                </Listbox>
            </ScrollShadow>
        </div>
    );
}