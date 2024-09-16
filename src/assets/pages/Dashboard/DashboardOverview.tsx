import {setTitle} from "../../../main.tsx";
import StatCard from "../../components/Dashboard/StatCard.tsx";
import CPU from "../../images/CPU.svg.tsx";
import RAM from "../../images/RAM.svg.tsx";
import Storage from "../../images/Storage.svg.tsx";
import ExtendedStorageStat from "../../components/Dashboard/ExtendedStorageStat.tsx";
import ExtendedOnlinePlayersStat from "../../components/Dashboard/ExtendedOnlinePlayersStat.tsx";
import RecentConnectionsStat from "../../components/Dashboard/RecentConnectionsStat.tsx";
import CrashReportsStat from "../../components/Dashboard/CrashReportsStat.tsx";
import BackupsStat from "../../components/Dashboard/BackupsStat.tsx";
import ServerList from "../../components/Dashboard/ServerList.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUser} from "@fortawesome/free-solid-svg-icons";

export default function DashboardOverview()
{
    setTitle("Dashboard Overview");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Overview</p>
            </div>
            <div className={"flex flex-row w-full justify-between"}>
                <StatCard
                    title={"CPU Usage"}
                    value={`40%`}
                    maxValue={100}
                    icon={<CPU size={24}/>}
                />
                <StatCard
                    title={"Memory Usage"}
                    value={`28 GB`}
                    maxValue={64}
                    icon={<RAM size={24}/>}
                />
                <StatCard
                    title={"Storage Usage"}
                    value={`800 GB`}
                    maxValue={1000}
                    icon={<Storage size={24}/>}
                />
                <StatCard
                    title={"Online Players"}
                    value={`4 Players`}
                    maxValue={20}
                    icon={<FontAwesomeIcon icon={faUser}/>}
                />
            </div>
            <div className={"flex flex-row w-full justify-between"}>
                <ExtendedStorageStat/>
                <ExtendedOnlinePlayersStat/>
            </div>
            <div className={"flex flex-row w-full justify-between gap-4"}>
                <RecentConnectionsStat/>
                <CrashReportsStat/>
                <BackupsStat/>
            </div>
            <ServerList/>
        </div>
    );
}