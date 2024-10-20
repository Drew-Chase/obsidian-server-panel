import {Navbar, NavbarContent, NavbarItem} from "@nextui-org/navbar";
import {Accordion, AccordionItem, cn, Image, Input, User} from "@nextui-org/react";
import MagnifyGlass from "../images/MagnifyGlass.svg.tsx";
import Home from "../images/Home.svg.tsx";
import UserIcon from "../images/User.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faChevronRight, faFileLines} from "@fortawesome/free-solid-svg-icons";
import {useLocation, useNavigate} from "react-router-dom";
import {useAuth} from "../providers/AuthProvider.tsx";
import Logo from "../images/logo.gif";

export default function Navigation()
{
    if (!window.location.pathname.startsWith("/app")) return;

    const indicator = (<FontAwesomeIcon className={"ml-auto text-neutral-400"} icon={faChevronRight} width={6} height={6}/>);
    const navigate = useNavigate();
    const {pathname} = useLocation();

    const sections = ["server", "discover", "users", "files", "settings", "profile"];
    const section = sections.find((s) => pathname.startsWith(`/app/${s}`)) || "";

    const {auth, isLoggedIn} = useAuth();

    if (!isLoggedIn && pathname.startsWith("/app"))
    {
        return (<></>);
    }
    return (
        <Navbar
            classNames={{
                wrapper: "flex flex-col pt-8",
                item: cn(
                    "text-neutral-400 text-medium p-4 cursor-pointer hover:bg-black/30 hover:text-neutral-100 rounded-md",
                    "border-l-4 border-transparent",
                    "data-[active=true]:bg-black/30 data-[active=true]:border-primary"
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
                    <Image src={Logo} width={32} radius={"sm"}/>
                    <h1 className={"font-semibold text-[1.25rem]"}>Obsidian</h1>
                </div>
                <Input
                    label={"Search"}
                    placeholder={"Search for..."}
                    variant={"underlined"}
                    aria-label={"Search"}
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
                        trigger: "p-4 rounded-lg hover:bg-black/40 data-[open=true]:bg-black/40",
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
                    disallowEmptySelection
                    selectionMode={"single"}
                    defaultExpandedKeys={[section]}
                >
                    <AccordionItem key={""} startContent={<Home/>} title={"Dashboard"} indicator={indicator} aria-label="Dashboard">
                        <NavbarItem key={"overview"} onClick={() => navigate("/app/")} isActive={pathname === "/app/"} aria-label="Overview">Overview</NavbarItem>
                        <NavbarItem key={"servers"} onClick={() => navigate("/app/servers/")} isActive={pathname === "/app/servers/"} aria-label="Servers">Servers</NavbarItem>
                        <NavbarItem key={"instances"} onClick={() => navigate("/app/instances/")} isActive={pathname === "/app/instances/"} aria-label="Instances">Instances</NavbarItem>
                        <NavbarItem key={"create-server"} onClick={() => navigate("/app/create-server/")} isActive={pathname === "/app/create-server/"} aria-label="Create Server">Create Server</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"server"} startContent={<FontAwesomeIcon icon={faFileLines}/>} title={"SMP Server"} indicator={indicator} aria-label="SMP Server">
                        <NavbarItem key={"server-profile"} onClick={() => navigate("/app/server/")} isActive={pathname === "/app/server/"} aria-label="Details">Details</NavbarItem>
                        <NavbarItem key={"server-properties"} onClick={() => navigate("/app/server/properties/")} isActive={pathname === "/app/server/properties/"} aria-label="Properties">Properties</NavbarItem>
                        <NavbarItem key={"server-mods"} onClick={() => navigate("/app/server/mods/")} isActive={pathname === "/app/server/mods/"} aria-label="Mods">Mods</NavbarItem>
                        <NavbarItem key={"server-files"} onClick={() => navigate("/app/server/files/")} isActive={pathname === "/app/server/files/"} aria-label="Files">Files</NavbarItem>
                        <NavbarItem key={"server-backups"} onClick={() => navigate("/app/server/backups/")} isActive={pathname === "/app/server/backups/"} aria-label="Backups">Backups</NavbarItem>
                        <NavbarItem key={"server-console"} onClick={() => navigate("/app/server/console/")} isActive={pathname === "/app/server/console/"} aria-label="Console">Console</NavbarItem>
                        <NavbarItem key={"server-players"} onClick={() => navigate("/app/server/players/")} isActive={pathname === "/app/server/players/"} aria-label="Players">Players</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"discover"} startContent={<MagnifyGlass/>} title={"Discover"} indicator={indicator} aria-label="Discover">
                        <NavbarItem key={"discover-all"} onClick={() => navigate("/app/discover/")} isActive={pathname === "/app/discover/"} aria-label="Instances">Instances</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"users"} startContent={<UserIcon/>} title={"Users"} indicator={indicator} aria-label="Users">
                        <NavbarItem key={"manage-users"} onClick={() => navigate("/app/users/")} isActive={pathname === "/app/users/"} aria-label="Manage Users">Manage Users</NavbarItem>
                        <NavbarItem key={"manage-groups"} onClick={() => navigate("/app/users/groups/")} isActive={pathname === "/app/users/groups/"} aria-label="Manage Groups">Manage Groups</NavbarItem>
                    </AccordionItem>
                    <AccordionItem key={"settings/profile"} startContent={<User name={auth.getUserProfile().username} description={"Administrator"}/>} indicator={indicator} aria-label="Settings/Profile">
                        <NavbarItem key={"profile"} onClick={() => navigate("/app/settings/profile/")} isActive={pathname === "/app/settings/profile/"} aria-label="Profile Settings">Profile Settings</NavbarItem>
                        <NavbarItem key={"settings"} onClick={() => navigate("/app/settings/")} isActive={pathname === "/app/settings/"} aria-label="Application Settings">Application Settings</NavbarItem>
                        <NavbarItem key={"logout"} onClick={() =>
                        {
                            auth.logout();
                            navigate("/");
                        }} aria-label="Logout">Logout</NavbarItem>
                    </AccordionItem>
                </Accordion>
            </NavbarContent>
        </Navbar>
    );
}