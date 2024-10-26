import {Button, Listbox, ListboxItem, ScrollShadow, Spinner, Tooltip} from "@nextui-org/react";
import ExtendedSwitch from "../../Extends/ExtendedSwitch.tsx";
import DownloadFile from "../../../images/DownloadFile.svg.tsx";
import {useEffect, useState} from "react";
import MinecraftVersions, {MinecraftVersion} from "../../../ts/mincraft-versions.ts";

export default function MinecraftVersionsList()
{
    const [includeSnapshots, setIncludeSnapshots] = useState(false);
    const [versions, setVersions] = useState<MinecraftVersion[]>([]);
    const [loading, setLoading] = useState(false);


    useEffect(() =>
    {
        if (includeSnapshots)
        {
            setLoading(true);
            MinecraftVersions.snapshots().then((versions) =>
            {
                setVersions(versions);
                setLoading(false);
            });
        } else
        {
            setLoading(true);
            MinecraftVersions.releases().then((versions) =>
            {
                setVersions(versions);
                setLoading(false);
            });
        }
    }, [includeSnapshots]);

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-h-[400px] h-dvh overflow-y-auto"}>
            <div className={"flex flex-row"}>

                <p className={"text-lg font-semibold mr-auto"}>Minecraft Versions</p>
                <ExtendedSwitch
                    label={"Include Snapshots"}
                    toggle={includeSnapshots}
                    onToggle={setIncludeSnapshots}
                />
            </div>

            <div className={"flex flex-row w-full text-tiny my-4"}>
                <p className={"mr-auto"}>Version / Type</p>
                <p>Actions</p>
            </div>
            {loading ? <Spinner size={"lg"} className={"mx-auto mt-5"}/> : (
                <ScrollShadow className={"max-h-[300px] h-[400px] overflow-y-auto"}>
                    <Listbox aria-label="Minecraft version list">
                        {versions.map((version, i) => (
                            <ListboxItem
                                key={i}
                                title={
                                    <div className={"flex flex-row items-center gap-2"}>
                                        <p className={"max-w-[90px] truncate"}>{version.id}</p>
                                    </div>
                                }
                                description={version.latest ? `latest ${version.type}` : version.type}
                                endContent={
                                    <div className={"flex flex-row"}>
                                        <Tooltip content={`Switch to ${version.id}`}>
                                            <Button aria-label={`Switch to version ${version.id}`} variant={"light"} size={"sm"} className={"min-w-0"}><DownloadFile/></Button>
                                        </Tooltip>
                                    </div>
                                }
                                textValue={`Version ${version.id}`}
                            />
                        ))}
                    </Listbox>
                </ScrollShadow>
            )}
        </div>
    );
}