import {useEffect, useState} from "react";
import {Slider} from "@nextui-org/react";
import OInput from "../../Extends/OInput.tsx";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import $ from "jquery";
import {useAuth} from "../../../providers/AuthProvider.tsx";

export default function MemoryAllocationInput()
{
    const [minMemory, setMinMemory] = useState<string>("");
    const [maxMemory, setMaxMemory] = useState<string>("");
    const systemMemoryCapacity = 32;
    const {server} = useSelectedServer();
    const {auth} = useAuth();

    useEffect(() =>
    {
        if (server) server?.settings().then(settings =>
        {
            setMaxMemory(settings.max_ram.toString());
            setMinMemory(settings.min_ram.toString());
        });
    }, [server]);

    useEffect(() =>
    {
        if (
            !server ||
            !auth.token ||
            minMemory === "" ||
            maxMemory === "" ||
            Number.isNaN(Number.parseInt(minMemory.replace(/\D/g, ""))) ||
            Number.isNaN(Number.parseInt(maxMemory.replace(/\D/g, "")))
        )
            return;
        console.log(`Updating memory allocation for ${server.name} to ${minMemory}GB - ${maxMemory}GB`);
        $.ajax({
            url: `/api/server/${server?.id}/settings?max-ram=${maxMemory}&min-ram=${minMemory}`,
            method: "POST",
            headers: {
                "X-Authorization-Token": auth.token
            }
        });

    }, [minMemory, maxMemory]);

    return (
        <div className={"flex flex-col gap-2"}>
            <p>Memory Allocation</p>
            <div className={"flex flex-row justify-between"}>
                <OInput
                    label={"Min"}
                    className={"w-[100px]"}
                    value={minMemory}
                    onValueChange={value => setMinMemory(value.replace(/\D/g, ""))}
                    onFocusChange={isFocused => !isFocused && setMinMemory((Number.isNaN(Number.parseInt(minMemory.replace(/\D/g, ""))) ? 1 : Number.parseInt(minMemory.replace(/\D/g, ""))).toString())}
                    endContent={<p className={"text-medium text-neutral-400"}>GB</p>}
                    aria-label="Minimum Memory"
                />
                <OInput
                    label={"Max"}
                    className={"w-[100px]"}
                    value={maxMemory}
                    onValueChange={value => setMaxMemory(value.replace(/\D/g, ""))}
                    onFocusChange={isFocused => !isFocused && setMaxMemory((Number.isNaN(Number.parseInt(maxMemory.replace(/\D/g, ""))) ? systemMemoryCapacity : Number.parseInt(maxMemory.replace(/\D/g, ""))).toString())}
                    endContent={<p className={"text-medium text-neutral-400"}>GB</p>}
                    aria-label="Maximum Memory"
                />
            </div>
            <Slider
                step={1}
                minValue={1}
                maxValue={systemMemoryCapacity}
                showTooltip
                showSteps={systemMemoryCapacity <= 32}
                value={
                    [
                        Number.isNaN(Number.parseInt(minMemory.replace(/\D/g, ""))) ? 1 : Number.parseInt(minMemory.replace(/\D/g, "")),
                        Number.isNaN(Number.parseInt(maxMemory.replace(/\D/g, ""))) ? systemMemoryCapacity : Number.parseInt(maxMemory.replace(/\D/g, ""))
                    ]
                }
                formatOptions={{style: "unit", unit: "gigabyte"}}
                aria-label="Memory Allocation Slider"
                onChange={(value) =>
                {
                    if (Array.isArray(value))
                    {
                        const [min, max] = value;
                        setMinMemory(`${min}`);
                        setMaxMemory(`${max}`);
                    }
                }}
            />
        </div>
    );
}