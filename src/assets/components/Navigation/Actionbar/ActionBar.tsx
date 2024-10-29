import {Navbar, NavbarBrand, NavbarContent, NavbarItem} from "@nextui-org/navbar";
import {cn, Image} from "@nextui-org/react";
import Logo from "../../../images/logo.gif";
import {useAuth} from "../../../providers/AuthProvider.tsx";
import ProfileDropdown from "./ProfileDropdown.tsx";
import NotificationDropdown from "./NotificationDropdown.tsx";

export default function ActionBar()
{
    const {isLoggedIn} = useAuth();
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
                            <NavbarItem><NotificationDropdown/></NavbarItem>
                            <NavbarItem><ProfileDropdown/></NavbarItem>
                        </>
                    )}
                </NavbarContent>
            </Navbar>
        </>
    );
}
