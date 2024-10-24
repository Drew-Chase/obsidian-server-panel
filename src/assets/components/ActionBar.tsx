import {Navbar, NavbarBrand, NavbarContent, NavbarItem} from "@nextui-org/navbar";
import {Avatar, Badge, cn, DropdownItem, DropdownMenu, DropdownSection, DropdownTrigger, Image, Link, Tooltip} from "@nextui-org/react";
import Logo from "../images/logo.gif";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import ODropdown from "./Extends/ODropdown";
import {faBell, faExternalLink, faSignOut, faSliders, faUser, faUsersGear, faUserShield} from "@fortawesome/free-solid-svg-icons";
import {useAuth} from "../providers/AuthProvider.tsx";
import {useNavigate} from "react-router-dom";

export default function ActionBar()
{
    const {auth, isLoggedIn, setIsLoggedIn} = useAuth();
    const navigate = useNavigate();
    return (
        <>
            <Navbar
                maxWidth={"full"}

                className={
                    cn(
                        "items-start min-w-[300px] w-[25%] max-w-[500px]",
                        "shadow-[5px_0_5px_rgba(25,25,112,0.3)] border-r border-r-neutral-600",
                        "bg-[#152578] overflow-y-auto rounded-2xl grow",
                        "rounded-xl mx-4 mt-2 w-[100vw_-_1rem] max-w-full"
                    )
                }
            >
                <NavbarBrand className={"gap-4"}>
                    <Image src={Logo} width={32} radius={"sm"}/>
                    <h1 className={"font-semibold text-[1.25rem]"}>Obsidian</h1>
                </NavbarBrand>
                <NavbarContent className={"flex flex-row gap-8"} justify={"end"}>
                    {isLoggedIn && (
                        <>
                            <NavbarItem>
                                <ODropdown>
                                    <DropdownTrigger>
                                        <div>
                                            <Badge content={"5"} color={"primary"}>
                                                <div className={"p-1 cursor-pointer"}>
                                                    <Tooltip content={"Notifications"}>
                                                        <FontAwesomeIcon icon={faBell} fontSize={18} className={"cursor-pointer text-white opacity-50 hover:opacity-100 transition-all"}/>
                                                    </Tooltip>
                                                </div>
                                            </Badge>
                                        </div>
                                    </DropdownTrigger>
                                    <DropdownMenu>
                                        <DropdownSection showDivider title={"Account"}>
                                            <DropdownItem>Profile</DropdownItem>
                                            <DropdownItem>Settings</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection>
                                            <DropdownItem>Help</DropdownItem>
                                            <DropdownItem>Feedback</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection title={"Danger Zone"} className={"text-danger"}>
                                            <DropdownItem endContent={<FontAwesomeIcon icon={faSignOut}/>} color={"danger"}>Logout</DropdownItem>
                                        </DropdownSection>
                                    </DropdownMenu>
                                </ODropdown>
                            </NavbarItem>
                            <NavbarItem>
                                <ODropdown>
                                    <DropdownTrigger>
                                        <div>
                                            <Tooltip content={auth.getUserProfile().username}>
                                                <Avatar className={"cursor-pointer text-white bg-[#2c41aa] data-[hover]:bg-white data-[hover]:text-black transition-all"} name={auth.getUserProfile().username[0].toUpperCase()}/>
                                            </Tooltip>
                                        </div>
                                    </DropdownTrigger>
                                    <DropdownMenu>
                                        <DropdownSection showDivider title={"Account"}>
                                            <DropdownItem href={"/app/settings/profile/"} endContent={<FontAwesomeIcon icon={faUser}/>}>Profile</DropdownItem>
                                            <DropdownItem href={"/app/users/"} endContent={<FontAwesomeIcon icon={faUserShield}/>}>Manage Users</DropdownItem>
                                            <DropdownItem href={"/app/users/groups/"} endContent={<FontAwesomeIcon icon={faUsersGear}/>}>Manage Groups</DropdownItem>
                                            <DropdownItem href={"/app/settings/"} endContent={<FontAwesomeIcon icon={faSliders}/>}>Settings</DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection>
                                            <DropdownItem
                                                as={Link}
                                                target={"_blank"}
                                                className={"text-inherit"}
                                                endContent={<FontAwesomeIcon icon={faExternalLink} fontSize={12}/>}
                                                href={"https://github.com/Drew-Chase/obsidian-server-panel"}
                                            >
                                                Source Code
                                            </DropdownItem>
                                            <DropdownItem
                                                as={Link}
                                                target={"_blank"}
                                                className={"text-inherit"}
                                                endContent={<FontAwesomeIcon icon={faExternalLink} fontSize={12}/>}
                                                href={"https://github.com/Drew-Chase/obsidian-server-panel/issues"}
                                            >
                                                Help
                                            </DropdownItem>
                                            <DropdownItem
                                                as={Link}
                                                target={"_blank"}
                                                className={"text-inherit"}
                                                endContent={<FontAwesomeIcon icon={faExternalLink} fontSize={12}/>}
                                                href={"https://github.com/Drew-Chase/obsidian-server-panel/issues/new"}
                                            >
                                                Feedback
                                            </DropdownItem>
                                        </DropdownSection>
                                        <DropdownSection title={"Danger Zone"} className={"text-danger"}>
                                            <DropdownItem
                                                endContent={<FontAwesomeIcon icon={faSignOut}/>}
                                                color={"danger"}
                                                onClick={() =>
                                                {
                                                    auth.logout();
                                                    setIsLoggedIn(false);
                                                    navigate("/");
                                                }}
                                            >
                                                Logout
                                            </DropdownItem>
                                        </DropdownSection>
                                    </DropdownMenu>
                                </ODropdown>
                            </NavbarItem>
                        </>
                    )}
                </NavbarContent>
            </Navbar>
        </>
    );
}
