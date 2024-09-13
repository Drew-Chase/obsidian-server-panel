import {setTitle} from "../../main.tsx";
import CalendarDropdown from "../components/Extends/CalendarDropdown.tsx";

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
        </div>
    );
}