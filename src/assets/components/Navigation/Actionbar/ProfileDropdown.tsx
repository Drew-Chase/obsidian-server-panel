import ODropdown from "../../Extends/ODropdown.tsx";
import {Avatar, DropdownItem, DropdownMenu, DropdownSection, DropdownTrigger, Link, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faExternalLink, faSignOut, faSliders, faUser, faUsersGear, faUserShield} from "@fortawesome/free-solid-svg-icons";
import {useAuth} from "../../../providers/AuthProvider.tsx";
import {useNavigate} from "react-router-dom";

export default function ProfileDropdown()
{
    const {auth, setIsLoggedIn} = useAuth();
    const navigate = useNavigate();
    return (

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

    );
}