import {setTitle} from "../../../main.tsx";
import ServerList from "../../components/Dashboard/ServerList.tsx";
import OverviewStatCard from "../../components/Dashboard/StatCards/OverviewStatCard.tsx";

export default function DashboardServers()
{
    setTitle("Dashboard Servers");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Servers</p>
            </div>
            <OverviewStatCard/>
            <ServerList/>
        </div>
    );
}