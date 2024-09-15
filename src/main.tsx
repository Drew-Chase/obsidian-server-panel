import React from "react";
import {BrowserRouter, Route, Routes, useNavigate} from "react-router-dom";
import ReactDOM from "react-dom/client";
import $ from "jquery";
import {NextUIProvider} from "@nextui-org/react";

import "./assets/scss/index.scss";
import DashboardOverview from "./assets/pages/Dashboard/DashboardOverview.tsx";
import Navigation from "./assets/components/Navigation.tsx";
import Login from "./assets/pages/Login.tsx";
import DashboardServers from "./assets/pages/Dashboard/DashboardServers.tsx";
import DashboardInstances from "./assets/pages/Dashboard/DashboardInstances.tsx";
import ServerDetails from "./assets/pages/Server/ServerDetails.tsx";
import ServerProperties from "./assets/pages/Server/ServerProperties.tsx";
import ServerConsole from "./assets/pages/Server/ServerConsole.tsx";
import ServerBackups from "./assets/pages/Server/ServerBackups.tsx";
import DashboardCreateServer from "./assets/pages/Dashboard/DashboardCreateServer.tsx";
import ServerPlayers from "./assets/pages/Server/ServerPlayers.tsx";
import ServerFiles from "./assets/pages/Server/ServerFiles.tsx";


export const debug_mode = true;

export const api_domain = "http://localhost:1420";
export const setTitle = (title: string) =>
{
    document.title = `${title} - Obsidian Minecraft Server Panel`;
};


ReactDOM.createRoot($("#root")[0]!).render(
    <React.StrictMode>
        <BrowserRouter>
            <MainContentRenderer/>
        </BrowserRouter>
    </React.StrictMode>
);

export function MainContentRenderer()
{
    const navigate = useNavigate();
    return (
        <NextUIProvider navigate={navigate} className={"flex flex-row gap-8"}>
            <Navigation/>
            <main className={"max-h-dvh h-dvh overflow-y-auto w-full p-6 mr-6"}>
                <Routes>
                    <Route>
                        <Route path="/" element={<Login/>}/>
                        {/* Dashboard */}
                        <Route path="/app/" element={<DashboardOverview/>}/>
                        <Route path="/app/servers/" element={<DashboardServers/>}/>
                        <Route path="/app/instances/" element={<DashboardInstances/>}/>
                        <Route path="/app/create-server/" element={<DashboardCreateServer/>}/>
                        {/* Server */}
                        <Route path="/app/server/" element={<ServerDetails/>}/>
                        <Route path="/app/server/properties/" element={<ServerProperties/>}/>
                        <Route path="/app/server/console/" element={<ServerConsole/>}/>
                        <Route path="/app/server/backups/" element={<ServerBackups/>}/>
                        <Route path="/app/server/players/" element={<ServerPlayers/>}/>
                        <Route path="/app/server/files/" element={<ServerFiles/>}/>
                    </Route>
                </Routes>
            </main>
        </NextUIProvider>
    );
}
