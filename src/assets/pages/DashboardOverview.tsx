import {setTitle} from "../../main.tsx";
import CalendarDropdown from "../components/Extends/CalendarDropdown.tsx";
import StatCard from "../components/Dashboard/StatCard.tsx";
import {DropdownItem} from "@nextui-org/react";

export default function DashboardOverview()
{
    setTitle("Dashboard Overview");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Overview</p>
                <CalendarDropdown
                    showDay
                />
            </div>
            <StatCard
                title={"CPU Usage"}
                value={40}
                valueContext={"%"}
            >
                <DropdownItem key={"hi"}>H</DropdownItem>
            </StatCard>
        </div>
    );
}