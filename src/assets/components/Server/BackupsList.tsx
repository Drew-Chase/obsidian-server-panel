import {Button, Listbox, ListboxItem, ScrollShadow, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPlus} from "@fortawesome/free-solid-svg-icons";

export default function BackupsList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 max-h-[400px] h-dvh overflow-y-auto grow"}>
            <div className={"flex flex-row"}>

                <p className={"text-lg font-semibold mr-auto"}>Backups</p>
                <Tooltip content={"Create a manual backup"}>
                    <Button aria-label={"Create a manual backup"}><FontAwesomeIcon icon={faPlus}/></Button>
                </Tooltip>
            </div>

            <div className={"flex flex-row w-full text-tiny my-4"}>
                <p className={"mr-auto"}>Name / Type</p>
                <p>Timestamp</p>
            </div>
            <ScrollShadow className={"max-h-[300px] h-[400px] overflow-y-auto"}>
                <Listbox aria-label={"List of backups"}>
                    {Array.from({length: 10}, (_, i) => (
                        <ListboxItem
                            key={i}
                            description={"Automatic backup"}
                            endContent={<p className={"text-tiny text-nowrap"}>2:36:50 PM</p>}
                            textValue={`Backup #${i} Automatic backup`}
                            aria-label={`Backup #${i} details`}
                        >
                            <div className={"flex flex-row items-center gap-2"}>
                                <p className={"max-w-[90px] truncate"}>Backup #{i}</p>
                            </div>
                        </ListboxItem>
                    ))}
                </Listbox>
            </ScrollShadow>
        </div>
    );
}