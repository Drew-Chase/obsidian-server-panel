import {useState} from "react";
import {Input, Slider} from "@nextui-org/react";

export default function MemoryAllocationInput()
{
    const [minMemory, setMinMemory] = useState<string>("4");
    const [maxMemory, setMaxMemory] = useState<string>("6");
    const systemMemoryCapacity = 32;
    return (
        <div className={"flex flex-col gap-2"}>
            <p>Memory Allocation</p>
            <div className={"flex flex-row justify-between"}>
                <Input
                    label={"Min"}
                    className={"w-[100px]"}
                    value={minMemory}
                    onValueChange={value => setMinMemory(value.replace(/\D/g, ""))}
                    onFocusChange={isFocused => !isFocused && setMinMemory((Number.isNaN(Number.parseInt(minMemory.replace(/\D/g, ""))) ? 1 : Number.parseInt(minMemory.replace(/\D/g, ""))).toString())}
                    endContent={<p className={"text-medium text-neutral-400"}>GB</p>}
                    classNames={{
                        inputWrapper: "bg-neutral-700"
                    }}
                />
                <Input
                    label={"Max"}
                    className={"w-[100px]"}
                    value={maxMemory}
                    onValueChange={value => setMaxMemory(value.replace(/\D/g, ""))}
                    onFocusChange={isFocused => !isFocused && setMaxMemory((Number.isNaN(Number.parseInt(maxMemory.replace(/\D/g, ""))) ? systemMemoryCapacity : Number.parseInt(maxMemory.replace(/\D/g, ""))).toString())}
                    endContent={<p className={"text-medium text-neutral-400"}>GB</p>}
                    classNames={{
                        inputWrapper: "bg-neutral-700"
                    }}
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