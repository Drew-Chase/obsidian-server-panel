import StatCard from "../../components/Dashboard/StatCard.tsx";
import CPU from "../../images/CPU.svg.tsx";
import RAM from "../../images/RAM.svg.tsx";
import Storage from "../../images/Storage.svg.tsx";
import InstancesList from "../../components/Dashboard/InstancesList.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUser} from "@fortawesome/free-solid-svg-icons";
import {setTitle} from "../../../main.tsx";
import Conversions from "../../ts/Conversions.ts";
import {useState} from "react";

export default function DashboardInstances()
{
    // @ts-ignore
    const [cpuUsage, setCpuUsage] = useState(0);
    // @ts-ignore
    const [memoryUsage, setMemoryUsage] = useState(0);
    // @ts-ignore
    const [totalMemory, setTotalMemory] = useState(0);
    // @ts-ignore
    const [storageUsage, setStorageUsage] = useState(0);
    // @ts-ignore
    const [onlinePlayers, setOnlinePlayers] = useState(0);
    setTitle("Dashboard Instances");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Instances</p>
            </div>
            <div className={"flex flex-row w-full justify-between"}>
                <StatCard
                    title={"CPU Usage"}
                    value={cpuUsage}
                    valueDisplay={`${cpuUsage}%`}
                    maxValue={100}
                    maxValueDisplay={"100%"}
                    icon={<CPU size={24}/>}
                />
                <StatCard
                    title={"Memory Usage"}
                    value={memoryUsage}
                    valueDisplay={`${Conversions.bytesToSize(memoryUsage)}`}
                    maxValue={totalMemory}
                    maxValueDisplay={Conversions.bytesToSize(totalMemory)}
                    icon={<RAM size={24}/>}
                />
                <StatCard
                    title={"Storage Usage"}
                    value={storageUsage}
                    valueDisplay={`${Conversions.bytesToSize(storageUsage)}`}
                    maxValue={1000}
                    maxValueDisplay={"1000 GB"}
                    icon={<Storage size={24}/>}
                />
                <StatCard
                    title={"Online Players"}
                    value={onlinePlayers}
                    valueDisplay={`${onlinePlayers} Players`}
                    maxValue={20}
                    icon={<FontAwesomeIcon icon={faUser}/>}
                />
            </div>
            <InstancesList/>
        </div>
    );
}