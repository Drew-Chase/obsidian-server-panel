import {Listbox, ListboxItem, SelectItem, Spinner} from "@nextui-org/react";
import MagnifyGlass from "../images/MagnifyGlass.svg.tsx";
import DiscoverModItem from "../components/Server/DiscoverModItem.tsx";
import {setTitle} from "../../main.tsx";
import Instances, {Instance, Platforms, SortOptions} from "../ts/instances.ts";
import {useEffect, useState} from "react";
import OInput from "../components/Extends/OInput.tsx";
import OSelect from "../components/Extends/OSelect.tsx";
import OTooltip from "../components/Extends/OTooltip.tsx";

export default function DiscoverInstances()
{
    const [instances, setInstances] = useState<Instance[]>([]);
    const [search, setSearch] = useState<string>("");
    const [platform, setPlatform] = useState<Platforms>(Platforms.ALL);
    const [sort, setSort] = useState<SortOptions>(SortOptions.RELEVANT);
    const [loading, setLoading] = useState<boolean>(true);
    let abortController = new AbortController();


    useEffect(() =>
    {
        setLoading(true);
        abortController.abort();
        abortController = new AbortController();
        Instances.browse({
            platform: platform,
            search: search,
            sort: sort,
            limit: 50,
            offset: 0
        }, abortController.signal)
            .then(i => i.hits)
            .then(setInstances)
            .finally(() => setLoading(false));
    }, [search, platform, sort]);

    setTitle("Discover Modpacks");
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4"}>
            <div className={"flex flex-col"}>
                <div className={"flex flex-row items-center gap-4"}>
                    <h2 className={"text-2xl font-semibold text-nowrap"}>Discover Instances</h2>
                    <div className={"-mb-4 flex flex-row items-center gap-4 w-full"}>
                        <OInput
                            label={"Search"}
                            placeholder={"Search for mods"}
                            startContent={<MagnifyGlass/>}
                            className={"w-full"}
                            value={search}
                            onValueChange={setSearch}
                            description={"Search for mods by name, author, or category"}
                        />
                        <OSelect
                            label={"Source"}
                            disallowEmptySelection
                            defaultSelectedKeys={[platform]}
                            disabledKeys={[Platforms.ATLAUNCHER]}
                            className={"w-[400px]"}
                            onSelectionChange={(key) => setPlatform([...key][0] as string as Platforms)}
                            value={platform}
                            description={"AtLauncher is not supported yet"}
                        >
                            {Object.values(Platforms).map(platform => (
                                <SelectItem key={platform}>{platform}</SelectItem>
                            ))}
                        </OSelect>
                        <OTooltip content={"Sort options only work for Modrinth"} >
                            <OSelect
                                label={"Sort By"}
                                disallowEmptySelection
                                defaultSelectedKeys={[sort]}
                                className={"w-[400px]"}
                                onSelectionChange={(key) => setSort([...key][0] as string as SortOptions)}
                                value={sort}
                                description={"Sort options only work for Modrinth"}
                            >
                                {Object.values(SortOptions).map(sort => (
                                    <SelectItem key={sort}>{sort}</SelectItem>
                                ))}
                            </OSelect>
                        </OTooltip>
                    </div>
                </div>
                <div className={"flex flex-col gap-4 mt-4 rounded-lg bg-neutral-800 p-4 overflow-y-auto max-h-[calc(100dvh_-_270px)] h-screen"}>
                    {loading ? <Spinner color={"primary"} className={"mx-auto"}/> : (
                        <Listbox
                            aria-label="Discover Modpacks List"
                            itemClasses={{
                                base: "data-[hover]:bg-neutral-600"
                            }}
                            emptyContent={"No mods found"}
                        >
                            {instances.map((instance, i) => (
                                <ListboxItem key={i} textValue={instance.name}>
                                    <DiscoverModItem {...instance}/>
                                </ListboxItem>
                            ))}
                        </Listbox>
                    )}
                </div>
            </div>
        </div>
    );
}
