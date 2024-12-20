import StatCard from "../StatCard.tsx";
import CPU from "../../../images/CPU.svg.tsx";
import Conversions from "../../../ts/conversions.ts";
import RAM from "../../../images/RAM.svg.tsx";
import Storage from "../../../images/Storage.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUser} from "@fortawesome/free-solid-svg-icons";
import {useEffect, useState} from "react";
import SystemMonitor from "../../../ts/system-monitor.ts";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import Server from "../../../ts/servers.ts";

export default function OverviewStatCard()
{
    const [cpuUsage, setCpuUsage] = useState(0);
    const [memoryUsage, setMemoryUsage] = useState(0);
    const [totalMemory, setTotalMemory] = useState(0);
    const [storageUsage, setStorageUsage] = useState(0);
    const [totalStorageUsage, setTotalStorageUsage] = useState(0);
    const [onlinePlayers, setOnlinePlayers] = useState(0);
    const {server} = useSelectedServer();

    useEffect(() =>
    {
        const monitor = new SystemMonitor();
        setCpuUsage(monitor.current_data.cpu_usage);
        setMemoryUsage(monitor.current_data.memory.used);
        setTotalMemory(monitor.current_data.memory.total);
        setOnlinePlayers(4);
        monitor.startMonitoring((data) =>
        {
            setCpuUsage(Math.min(Math.floor(data.cpu_usage), 100));
            setMemoryUsage(data.memory.used);
            setTotalMemory(data.memory.total);
            setOnlinePlayers(4);
        });

        if (server) setStorageUsage(server.size);
        Server
            .list()
            .then(servers => servers.reduce((total, server) => total + server.size, 0))
            .then(totalSize =>
            {
                if (!server) setStorageUsage(totalSize);
                setTotalStorageUsage(totalSize);
            });


        return () => monitor.stopMonitoring();
    }, []);

    return (
        <div className={"flex flex-row w-full justify-between flex-wrap gap-4 grow shrink"}>
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
                maxValue={totalStorageUsage}
                maxValueDisplay={`${Conversions.bytesToSize(totalStorageUsage)}`}
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
    );
}