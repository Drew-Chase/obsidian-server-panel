import {RadialBar, RadialBarChart, ResponsiveContainer} from "recharts";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCircle} from "@fortawesome/free-solid-svg-icons";

export default function BackupsStat()
{
    let data = [{
        "SMP Server": 75,
        "All The Mods 6": 30,
        "Other": 20
    }];
    const total = data.reduce((acc, cur) => acc + cur["SMP Server"] + cur["All The Mods 6"] + cur["Other"], 0);
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 max-h-[400px] h-dvh overflow-y-auto grow relative"}>
            <p className={"text-lg font-semibold"}>Backups</p>
            <ResponsiveContainer width={"100%"} height={"100%"}>
                <RadialBarChart
                    cx="50%"
                    cy="50%"
                    innerRadius="80%"
                    outerRadius="80%"
                    barSize={20}
                    data={data}
                    startAngle={180}
                    endAngle={0}
                >
                    <RadialBar dataKey={"SMP Server"} fill={"#CB3CFF"} stackId={"a"}/>
                    <RadialBar dataKey={"All The Mods 6"} fill={"#0E43FB"} stackId={"a"}/>
                    <RadialBar dataKey={"Other"} fill={"#00C2FF"} stackId={"a"}/>
                </RadialBarChart>
            </ResponsiveContainer>
            <div className={"absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full items-center flex flex-col"}>
                <p className={"font-bold text-5xl"}>{total}</p>
                <p className={"font-light text-xl opacity-70 text-center text-neutral-400"}>64GB Total</p>
            </div>
            <div className={"flex flex-row w-full items-center justify-center gap-8 absolute top-[260px] left-0"}>
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
            </div>
        </div>


    );
}