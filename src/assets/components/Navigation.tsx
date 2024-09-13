import {Navbar, NavbarContent, NavbarItem} from "@nextui-org/navbar";
import Logo from "../images/Logo.svg.tsx";
import {Accordion, AccordionItem, cn, Input, User} from "@nextui-org/react";
import MagnifyGlass from "../images/MagnifyGlass.svg.tsx";
import Home from "../images/Home.svg.tsx";
import UserIcon from "../images/User.svg.tsx";
import Folder from "../images/Folder.svg.tsx";
import Settings from "../images/Settings.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faChevronRight} from "@fortawesome/free-solid-svg-icons";
import {useLocation, useNavigate} from "react-router-dom";

export default function Navigation()
{
    if (!window.location.pathname.startsWith("/app")) return;

    const indicator = (<FontAwesomeIcon className={"ml-auto text-neutral-400"} icon={faChevronRight} width={6} height={6}/>);
    const navigate = useNavigate();
    const {pathname} = useLocation();

    return (
        <Navbar
            classNames={{
                wrapper: "flex flex-col pt-8",
                item: cn(
                    "text-neutral-400 text-medium p-4 cursor-pointer hover:bg-neutral-700 hover:text-neutral-100 rounded-md",
                    "border-l-4 border-transparent",
                    "data-[active=true]:bg-neutral-700 data-[active=true]:border-primary"
                )
            }}
            className={
                cn(
                    "items-start min-w-[300px] w-[25%] max-w-[500px]",
                    "shadow-[5px_0_5px_rgba(25,25,112,0.3)] border-r border-r-neutral-600",
                    "bg-custom-gradient h-screen overflow-y-auto"
                )
            }
        >

            <NavbarContent className={"flex flex-col items-start w-full h-auto gap-8"}>
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
            </NavbarContent>
            <NavbarContent className={"flex flex-col items-start w-full h-auto gap-8"}>
                <Accordion
                    itemClasses={{
                        title: "data-[open=true]:text-primary",
                        base: "data-[open=true]:text-primary",
                        trigger: "p-4 rounded-lg hover:bg-neutral-800 data-[open=true]:bg-neutral-800",
                        content: "ml-6 flex flex-col gap-2"
                    }}
                    onSelectionChange={(selected) =>
                    {
                        const keys = [...selected];
                        if (keys && keys.length > 0)
                        {
                            navigate(`/app/${keys[0]}/`.replace(/\/\//g, "/"));
                        }
                    }}
                    defaultExpandedKeys={[""]}
                >
                    <AccordionItem key={""} startContent={<Home/>} title={"Dashboard"} indicator={indicator}>
                        <NavbarItem key={"overview"} onClick={() => navigate("/app/")} isActive={pathname === "/app/"}>Overview</NavbarItem>
                        <NavbarItem key={"servers"} onClick={() => navigate("/app/servers/")} isActive={pathname === "/app/servers/"}>Servers</NavbarItem>
                        <NavbarItem key={"instances"} onClick={() => navigate("/app/instances/")} isActive={pathname === "/app/instances/"}>Instances</NavbarItem>
                        <NavbarItem key={"create-server"} onClick={() => navigate("/app/create-server/")} isActive={pathname === "/app/create-server/"}>Create Server</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"discover"} startContent={<MagnifyGlass/>} title={"Discover"} indicator={indicator}>
                        <NavbarItem key={"discover-all"} onClick={() => navigate("/app/discover/")} isActive={pathname === "/app/discover/"}>All</NavbarItem>
                        <NavbarItem key={"discover-modrinth"} onClick={() => navigate("/app/discover/modrinth/")} isActive={pathname === "/app/discover/modrinth/"}>Modrinth</NavbarItem>
                        <NavbarItem key={"discover-curseforge"} onClick={() => navigate("/app/discover/curseforge/")} isActive={pathname === "/app/discover/curseforge/"}>CurseForge</NavbarItem>
                        <NavbarItem key={"discover-atlauncher"} onClick={() => navigate("/app/discover/atlauncher/")} isActive={pathname === "/app/discover/atlauncher/"}>ATLauncher</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"users"} startContent={<UserIcon/>} title={"Users"} indicator={indicator}>
                        <NavbarItem key={"manage-users"} onClick={() => navigate("/app/users/")} isActive={pathname === "/app/users/"}>Manage Users</NavbarItem>
                        <NavbarItem key={"manage-groups"} onClick={() => navigate("/app/users/groups/")} isActive={pathname === "/app/users/groups/"}>Manage Groups</NavbarItem>
                        <NavbarItem key={"add-user"} onClick={() => navigate("/app/users/add/")} isActive={pathname === "/app/users/add/"}>Add User</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"files"} startContent={<Folder/>} title={"Files"} indicator={indicator}>
                        <NavbarItem key={"manage-files"} onClick={() => navigate("/app/files/")} isActive={pathname === "/app/files/"}>Manage Files</NavbarItem>
                        <NavbarItem key={"manage-files"} onClick={() => navigate("/app/files/backups")} isActive={pathname === "/app/files/backups"}>Manage Backups</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"settings"} startContent={<Settings/>} title={"Settings"} indicator={indicator}>
                        <NavbarItem key={"settings"} onClick={() => navigate("/app/settings/")} isActive={pathname === "/app/settings/"}>General Settings</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"profile"} startContent={<User name={"Drew Chase"} description={"Account Settings"}/>} indicator={indicator}>
                        <NavbarItem key={"logout"} onClick={() => navigate("/")}>Logout</NavbarItem>
                    </AccordionItem>
                </Accordion>
            </NavbarContent>
        </Navbar>
    );
}