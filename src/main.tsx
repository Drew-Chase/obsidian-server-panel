import {BrowserRouter, Route, Routes, useNavigate} from "react-router-dom";
import ReactDOM from "react-dom/client";
import $ from "jquery";
import {NextUIProvider} from "@nextui-org/react";

import "./assets/scss/index.scss";
import DashboardOverview from "./assets/pages/Dashboard/DashboardOverview.tsx";
import Navigation from "./assets/components/Navigation/Navigation.tsx";
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
import ServerMods from "./assets/pages/Server/ServerMods.tsx";
import DiscoverInstances from "./assets/pages/DiscoverInstances.tsx";
import Users from "./assets/pages/Users/Users.tsx";
import UserGroups from "./assets/pages/Users/UserGroups.tsx";
import ApplicationSettings from "./assets/pages/Settings/ApplicationSettings.tsx";
import ProfileSettings from "./assets/pages/Profile/ProfileSettings.tsx";
import Register from "./assets/pages/Register.tsx";
import {AuthProvider, useAuth} from "./assets/providers/AuthProvider.tsx";
import {Toaster} from "sonner";
import React from "react";
import {ScreenSizeProvider} from "./assets/providers/ScreenSizeProvider.tsx";
import {SelectedServerProvider} from "./assets/providers/SelectedServerProvider.tsx";
import {AlertModalProvider} from "./assets/providers/AlertModalProvider.tsx";
import ActionBar from "./assets/components/Navigation/Actionbar/ActionBar.tsx";

export const setTitle = (title: string) =>
{
    document.title = `${title} - Obsidian Minecraft Server Panel`;
};


ReactDOM.createRoot($("#root")[0]!).render(
    <React.StrictMode>
        <BrowserRouter>
            <AlertModalProvider>
                <ScreenSizeProvider>
                    <AuthProvider>
                        <SelectedServerProvider>
                            <MainContentRenderer/>
                        </SelectedServerProvider>
                    </AuthProvider>
                </ScreenSizeProvider>
            </AlertModalProvider>
        </BrowserRouter>
    </React.StrictMode>
);

export function MainContentRenderer()
{
    const navigate = useNavigate();
    const {isLoggedIn} = useAuth();

    return (
        <NextUIProvider navigate={navigate} className={"flex flex-col"}>
            <Toaster position={"bottom-right"} closeButton richColors theme={"dark"} toastOptions={{
                className: "bg-default/50 border-2 border-primary/50 rounded-md shadow-lg backdrop-blur-sm"
            }}/>
            {isLoggedIn ? (
                <>
                    <ActionBar/>
                    <div className={"flex flex-row"}>
                        <Navigation/>
                        <main className={"max-h-[calc(100dvh_-_75px)] h-dvh overflow-y-auto w-full p-6 mr-6"}>
                            <Routes>
                                <Route>
                                    <Route path="/" element={<Login/>}/>
                                    <Route path="/register/" element={<Register/>}/>
                                    {/* Dashboard */}
                                    <Route path="/app/" element={<DashboardOverview/>}/>
                                    <Route path="/app/servers/" element={<DashboardServers/>}/>
                                    <Route path="/app/instances/" element={<DashboardInstances/>}/>
                                    <Route path="/app/create-server/" element={<DashboardCreateServer/>}/>
                                    <Route path="/app/instances/discover/" element={<DiscoverInstances/>}/>
                                    {/* Server */}
                                    <Route path="/app/server/" element={<ServerDetails/>}/>
                                    <Route path="/app/server/properties/" element={<ServerProperties/>}/>
                                    <Route path="/app/server/console/" element={<ServerConsole/>}/>
                                    <Route path="/app/server/backups/" element={<ServerBackups/>}/>
                                    <Route path="/app/server/players/" element={<ServerPlayers/>}/>
                                    <Route path="/app/server/files/" element={<ServerFiles/>}/>
                                    <Route path="/app/server/mods/" element={<ServerMods/>}/>
                                    {/* Users */}
                                    <Route path="/app/users/" element={<Users/>}/>
                                    <Route path="/app/users/groups/" element={<UserGroups/>}/>
                                    {/* Settings */}
                                    <Route path="/app/settings/profile/" element={<ProfileSettings/>}/>
                                    <Route path="/app/settings/" element={<ApplicationSettings/>}/>
                                </Route>
                            </Routes>
                        </main>
                    </div>
                </>
            ) : (
                <>
                    <main className={"max-h-dvh h-dvh overflow-y-auto w-full p-6 mr-6"}>
                        <Routes>
                            <Route>
                                <Route path="/" element={<Login/>}/>
                                <Route path="/register/" element={<Register/>}/>
                            </Route>
                        </Routes>
                    </main>
                </>
            )}
        </NextUIProvider>
    );
}
