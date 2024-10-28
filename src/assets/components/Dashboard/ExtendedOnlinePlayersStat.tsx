import {Bar, BarChart, ResponsiveContainer, Tooltip as ChartTooltip, XAxis, YAxis} from "recharts";
import {Button, Divider, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArrowDown, faCircle} from "@fortawesome/free-solid-svg-icons";
import {HTMLAttributes} from "react";


interface CustomTooltipProps
{
    active: boolean;
    payload: Payload[];
    label: string;
}

interface Payload
{
    name: string;
    value: number;
}

export default function ExtendedOnlinePlayersStat(props: HTMLAttributes<any>)
{

    // @ts-ignore
    const CustomTooltip = ({active, payload, label}: CustomTooltipProps) =>
    {
        if (active && payload && payload.length)
        {
            const total = payload.reduce((acc, cur) => acc + cur.value, 0);
            return (
                <div className="bg-neutral-800 shadow-xl p-4 rounded-md flex flex-col w-[300px] shrink-0 grow">
                    <p className="font-light text-tiny">{label}</p>
                    <Divider/>
                    {

                        Array.from(payload).map((i, n) =>
                        {
                            return (
                                <div key={`${i.name}-${i.value}-${n}`}>
                                    <p className={"flex flex-row"}>
                                        <span className="font-light mr-auto italic">{i.name}:</span>
                                        <span className="font-bold">{i.value} <span className="font-light">/ {total}</span></span>
                                    </p>
                                    <Divider/>
                                </div>
                            );
                        })
                    }
                </div>
            );
        }

        return null;
    };
    let data = [];
    for (let i = 0; i < 12; i++)
    {
        const month = new Date();
        month.setMonth(i);

        data.push({
            month: month.toLocaleString("default", {month: "short"}),
            "SMP Server": Math.ceil(Math.random() * 10),
            "All The Mods 6": Math.ceil(Math.random() * 10),
            "Other": Math.ceil(Math.random() * 10)
        });
    }

    const total = data.reduce((acc, cur) => acc + cur["SMP Server"] + cur["All The Mods 6"] + cur["Other"], 0);

    return (
        <div  {...props} className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 max-w-full h-lg min-w-[540px] grow shrink"}>
            <p className={"text-[16px] font-semibold text-neutral-400 mb-2"}>Total Online Players</p>
            <div className={"flex flex-row w-full items-center gap-8"}>
                <p className={"text-3xl font-bold"}>{total} Players</p>
                <div className={"flex flex-row items-center gap-1"}>
                    <FontAwesomeIcon icon={faCircle} width={8} color={"#CB3CFF"}/>
                    <span className={"text-neutral-400 text-sm"}>SMP Server</span>
                </div>
                <div className={"flex flex-row items-center gap-1"}>
                    <FontAwesomeIcon icon={faCircle} width={8} color={"#0E43FB"}/>
                    <span className={"text-neutral-400 text-sm"}>All The Mods 6</span>
                </div>
                <div className={"flex flex-row items-center gap-1"}>
                    <FontAwesomeIcon icon={faCircle} width={8} color={"#00C2FF"}/>
                    <span className={"text-neutral-400 text-sm"}>Other</span>
                </div>
                <Tooltip content={"Export a CSV with a breakdown of online player stats."}>
                    <Button
                        endContent={<FontAwesomeIcon icon={faArrowDown}/>}
                        className={"ml-auto"}
                    >
                        Export
                    </Button>
                </Tooltip>
            </div>
            <ResponsiveContainer width={"100%"} height={"100%"} className={"min-h-[200px]"}>
                <BarChart
                    width={500}
                    height={300}
                    margin={{
                        top: 20,
                        right: 30,
                        left: 20,
                        bottom: 5
                    }}
                    data={data}
                >
                    <XAxis dataKey={"month"} tick={{fontSize: 12}}/>
                    <YAxis/>
                    {/* @ts-ignore */}
                    <ChartTooltip content={<CustomTooltip/>} animationDuration={200}/>
                    <Bar dataKey={"SMP Server"} fill={"#CB3CFF"} stackId={"a"} barSize={15}/>
                    <Bar dataKey={"All The Mods 6"} fill={"#0E43FB"} stackId={"a"} barSize={15}/>
                    <Bar dataKey={"Other"} fill={"#00C2FF"} radius={[20, 20, 0, 0]} stackId={"a"} barSize={15}/>
                </BarChart>
            </ResponsiveContainer>
        </div>
    );
}