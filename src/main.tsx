import React from "react";
import {BrowserRouter, Route, Routes, useNavigate} from "react-router-dom";
import ReactDOM from "react-dom/client";
import $ from "jquery";
import {NextUIProvider} from "@nextui-org/react";

import "./assets/scss/index.scss";
import Home from "./assets/pages/Home.tsx";
import Navigation from "./assets/components/Navigation.tsx";
import {applyTheme} from "./assets/ts/Theme.ts";


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
                        <Route path="/" element={<Home/>}/>
                    </Route>
                </Routes>
            </main>
        </NextUIProvider>
    );
}
