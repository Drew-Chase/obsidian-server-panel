import React from "react";
import {BrowserRouter, Route, Routes, useNavigate} from "react-router-dom";
import ReactDOM from "react-dom/client";
import $ from "jquery";
import {NextUIProvider} from "@nextui-org/react";

import "./assets/scss/index.scss";
import DashboardOverview from "./assets/pages/DashboardOverview.tsx";
import Navigation from "./assets/components/Navigation.tsx";
import {applyTheme} from "./assets/ts/Theme.ts";
import Login from "./assets/pages/Login.tsx";


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
    applyTheme();
    const navigate = useNavigate();
    return (
        <NextUIProvider navigate={navigate} className={"flex flex-row gap-8"}>
            <Navigation/>
            <main className={"max-h-dvh h-dvh overflow-y-auto w-full p-6 mr-6"}>
                <Routes>
                    <Route>
                        <Route path="/" element={<Login/>}/>
                        <Route path="/app" element={<DashboardOverview/>}/>
                    </Route>
                </Routes>
            </main>
        </NextUIProvider>
    );
}
