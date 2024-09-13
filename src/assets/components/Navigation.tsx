import {Navbar, NavbarContent, NavbarItem} from "@nextui-org/navbar";
import Logo from "../images/Logo.svg.tsx";
import {Accordion, AccordionItem, Autocomplete, AutocompleteItem, Input} from "@nextui-org/react";
import MagnifyGlass from "../images/MagnifyGlass.svg.tsx";
import Home from "../images/Home.svg.tsx";

export default function Navigation()
{

    return (
        <Navbar
            classNames={{
                item: "text-neutral-400 text-medium p-4 cursor-pointer hover:bg-neutral-700 hover:text-neutral-100 rounded-md",
            }}
            className={"items-start min-w-[300px] w-[25%] max-w-[500px] bg-neutral-800 drop-shadow-[5px_0_5px_hsl(226_63%_8%)] border-r-neutral-600 border-r-1"}
        >
            <NavbarContent className={"flex flex-col items-start w-full h-full mt-14 gap-8"}>
                <div className={"flex flex-row gap-2"}>
                    <Logo/>
                    <h1 className={"font-semibold text-[1.25rem]"}>Obsidian</h1>
                </div>
                <Input
                    label={"Search"}
                    placeholder={"Search for..."}
                    startContent={<MagnifyGlass size={18}/>}
                    className={"w-full drop-shadow-lg"}
                    classNames={{
                        inputWrapper: "w-full rounded-lg border-[#343B4F]/50 border-1"
                    }}
                />
                <Accordion
                    itemClasses={{
                        title: "data-[open=true]:text-primary",
                        base:"data-[open=true]:text-primary"
                    }}
                >
                    <AccordionItem startContent={<Home/>} title={"Dashboard"}>
                        <NavbarItem>Overview</NavbarItem>
                        <NavbarItem>Servers</NavbarItem>
                        <NavbarItem>Instances</NavbarItem>
                        <NavbarItem>Create Server</NavbarItem>
                    </AccordionItem>
                    <AccordionItem startContent={<MagnifyGlass/>} title={"Discover"}>
                        <NavbarItem>All</NavbarItem>
                        <NavbarItem>Modrinth</NavbarItem>
                        <NavbarItem>CurseForge</NavbarItem>
                        <NavbarItem>ATLauncher</NavbarItem>
                    </AccordionItem>
                </Accordion>
            </NavbarContent>

        </Navbar>
    );
}