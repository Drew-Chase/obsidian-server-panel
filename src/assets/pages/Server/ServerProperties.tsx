import {setTitle} from "../../../main.tsx";
import {Input, ScrollShadow, Skeleton} from "@nextui-org/react";
import ExtendedSwitch from "../../components/Extends/ExtendedSwitch.tsx";
import {useEffect, useState} from "react";
import MagnifyGlass from "../../images/MagnifyGlass.svg.tsx";
import OInput from "../../components/Extends/OInput.tsx";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";
import {toast} from "sonner";

export interface ServerPropertiesItem
{
    name: string;
    value: string;
    type: "string" | "number" | "boolean";
}

export default function ServerProperties()
{
    setTitle("Server Properties");
    const [properties, setProperties] = useState<ServerPropertiesItem[]>([]);
    const [search, setSearch] = useState("");
    const {server} = useSelectedServer();
    const [loading, setLoading] = useState(false);

    useEffect(() =>
    {
        setLoading(true);
        if (server == null) return;
        server
            .properties()
            .then(setProperties)
            .finally(() => setLoading(false));
    }, [server]);


    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Properties</p>
            </div>
            <Input
                label={"Search"}
                placeholder={"Search for..."}
                value={search}
                startContent={<MagnifyGlass/>}
                onValueChange={setSearch}
                autoFocus
                variant={"underlined"}
            />
            <ScrollShadow className={"flex flex-col max-h-[calc(100dvh_-_305px)] h-dvh gap-4 overflow-y-auto pr-4 shrink-0 pb-8"}>
                {loading ? <>{Array.from({length: 10}).map(() => (<Skeleton><OInput/></Skeleton>))}</> :
                    <>
                        {properties
                            .filter(({name}) => name.replace(/[-._]/g, " ").toLowerCase().includes(search.toLowerCase()))
                            .sort((a, b) => a.name.localeCompare(b.name))
                            .map(({name, value, type}) => (
                                type === "string" || type === "number" ? (
                                    <OInput
                                        key={name}
                                        label={name}
                                        defaultValue={value}
                                        onValueChange={value =>
                                        {
                                            setProperties(properties.map(p => p.name === name ? {...p, value} : p));
                                        }}
                                        onFocusChange={focused =>
                                        {
                                            if (!focused)
                                            {
                                                toast("Property updated", {description: `Property ${name} has been set to ${value}`});
                                                server?.updateProperty(name, value);
                                            }
                                        }}
                                    />
                                ) : (
                                    <ExtendedSwitch
                                        key={name}
                                        label={name}
                                        toggle={value === "true"}
                                        onToggle={async (toggle) =>
                                        {
                                            toast("Property updated", {description: `Property ${name} has been set to ${toggle}`});
                                            await server?.updateProperty(name, toggle);
                                        }}
                                        className={"max-w-full shrink-0"}
                                    />
                                )
                            ))}
                    </>}
            </ScrollShadow>
        </div>
    );
}
