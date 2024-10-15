import {setTitle} from "../../../main.tsx";
import ExtendedStorageStat from "../../components/Dashboard/ExtendedStorageStat.tsx";
import ExtendedOnlinePlayersStat from "../../components/Dashboard/ExtendedOnlinePlayersStat.tsx";
import RecentConnectionsStat from "../../components/Dashboard/RecentConnectionsStat.tsx";
import CrashReportsStat from "../../components/Dashboard/CrashReportsStat.tsx";
import BackupsStat from "../../components/Dashboard/BackupsStat.tsx";
import ServerList from "../../components/Dashboard/ServerList.tsx";
import OverviewStatCard from "../../components/Dashboard/StatCards/OverviewStatCard.tsx";

export default function DashboardOverview()
{
    setTitle("Dashboard Overview");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Overview</p>
            </div>
            <OverviewStatCard/>
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