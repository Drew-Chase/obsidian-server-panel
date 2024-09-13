import {nextui} from "@nextui-org/react";

/** @type {import('tailwindcss').Config} */
export default {
    content:     [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
        "./node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}"
    ], theme:    {
        extend: {},
    }, darkMode: "class", plugins: [
        nextui({
            themes: {
                light:   {
                    colors: {
                        primary:      {
                            DEFAULT: "#f13848", foreground: "#fff",
                        }, secondary: "#2b2b2b", background: "#e3e3ea",

                    }
                }, dark: {
                    "colors": {
                        default:         {
                            "100":     "#0B1739",
                            "200":     "#111e45",
                            "300":     "#182753",
                            "400":     "#182753",
                            "DEFAULT": "#182753",
                        },
                        content1:        "#0B1739",
                        content2:        "#111e45",
                        content3:        "#182753",
                        content4:        "#182753",
                        "primary":       {
                            "DEFAULT":    "#CB3CFF",
                            "dark":       "#8951ff",
                            "foreground": "#ffffff"
                        },
                        "secondary":     "#21c3fc",
                        "accent":        "#0e43fb",
                        "neutral":       {
                            "100": "#ffffff",
                            "200": "#d9e1fa",
                            "300": "#d1dbf9",
                            "400": "#aeb9e1",
                            "500": "#7e89ac",
                            "600": "#0b1739",
                            "700": "#0a1330",
                            "800": "#081028"
                        },
                        "system":        {
                            "blue":   {
                                "400": "#086cd9",
                                "300": "#1d88fe",
                                "200": "#8fc3ff",
                                "100": "#eaf4ff"
                            },
                            "green":  {
                                "400": "#11845b",
                                "300": "#05c168",
                                "200": "#7fdca4",
                                "100": "#def2e6"
                            },
                            "red":    {
                                "400": "#dc2b2b",
                                "300": "#ff5a65",
                                "200": "#ffbec2",
                                "100": "#ffeff0"
                            },
                            "orange": {
                                "400": "#d5691b",
                                "300": "#ff9e2c",
                                "200": "#ffd19b",
                                "100": "#fff3e4"
                            }
                        },
                        "other":         {
                            "purple":  {
                                "100": "#575dff",
                                "50":  "#575dff"
                            },
                            "blue":    {
                                "50": "#1d88fe"
                            },
                            "skyBlue": {
                                "50": "#57c3ff"
                            },
                            "red":     {
                                "50": "#ff5a65"
                            },
                            "gray":    {
                                "50": "#aeb9e1"
                            },
                            "green":   {
                                "50": "#05c168"
                            },
                            "yellow":  {
                                "50": "#ffb016"
                            },
                            "overlay": {
                                "60%": "#000000",
                                "40%": "#000000"
                            }
                        },
                        "illustrations": {
                            "orange":      {
                                "100": "#ffe3d3",
                                "200": "#e8ad8c"
                            },
                            "blueDark":    {
                                "100": "#27266a"
                            },
                            "purpleLight": "#dbd8ff"
                        }
                    }

                },
            }
        })
    ]
}