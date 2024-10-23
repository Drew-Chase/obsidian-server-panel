import InstancesList from "../../components/Dashboard/InstancesList.tsx";
import {setTitle} from "../../../main.tsx";
import OverviewStatCard from "../../components/Dashboard/StatCards/OverviewStatCard.tsx";

export default function DashboardInstances()
{
    setTitle("Dashboard Instances");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Instances</p>
            </div>
            <OverviewStatCard/>
            <InstancesList/>
        </div>
    );
}