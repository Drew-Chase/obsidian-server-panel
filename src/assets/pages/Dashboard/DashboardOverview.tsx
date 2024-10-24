import {setTitle} from "../../../main.tsx";
import ExtendedStorageStat from "../../components/Dashboard/ExtendedStorageStat.tsx";
import ExtendedOnlinePlayersStat from "../../components/Dashboard/ExtendedOnlinePlayersStat.tsx";
import RecentConnectionsStat from "../../components/Dashboard/RecentConnectionsStat.tsx";
import CrashReportsStat from "../../components/Dashboard/CrashReportsStat.tsx";
import BackupsStat from "../../components/Dashboard/BackupsStat.tsx";
import ServerList from "../../components/Dashboard/ServerList.tsx";
import OverviewStatCard from "../../components/Dashboard/StatCards/OverviewStatCard.tsx";
import {useScreenSize} from "../../providers/ScreenSizeProvider.tsx";

export default function DashboardOverview()
{
    const {width} =useScreenSize();
    setTitle("Dashboard Overview");
    return (
        <div className={"flex flex-col gap-8"}>
            <OverviewStatCard/>
            <div className={"flex flex-row w-full justify-between flex-wrap xl:flex-nowrap gap-4"}>
                <ExtendedStorageStat/>
                <ExtendedOnlinePlayersStat hidden={width < 930}/>
            </div>
            <div className={"flex flex-row w-full justify-between flex-wrap xl:flex-nowrap gap-4"}>
                <RecentConnectionsStat/>
                <CrashReportsStat/>
                <BackupsStat/>
            </div>
            <ServerList/>
        </div>
    );
}