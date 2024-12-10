import ODropdown from "../../Extends/ODropdown.tsx";
import {Avatar, DropdownItem, DropdownMenu, DropdownSection, DropdownTrigger, Link} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faExternalLink, faSignOut, faSliders, faUser, faUsersGear, faUserShield} from "@fortawesome/free-solid-svg-icons";
import {useAuth} from "../../../providers/AuthProvider.tsx";
import {useNavigate} from "react-router-dom";
import OTooltip from "../../Extends/OTooltip.tsx";

export default function ProfileDropdown()
{
    const {auth, setIsLoggedIn} = useAuth();
    const navigate = useNavigate();
    return (

        <ODropdown>
            <DropdownTrigger>
                <div>
                    <OTooltip content={auth.getUserProfile().username}>
                        <Avatar className={"cursor-pointer text-white bg-[#2c41aa] data-[hover]:bg-white data-[hover]:text-black transition-all"} name={auth.getUserProfile().username[0].toUpperCase()}/>
                    </OTooltip>
                </div>
            </DropdownTrigger>
            <DropdownMenu>
                <DropdownSection showDivider title={"Account"} key="account">
                    <DropdownItem href={"/app/settings/profile/"} endContent={<FontAwesomeIcon icon={faUser}/>} key="profile">Profile</DropdownItem>
                    <DropdownItem href={"/app/users/"} endContent={<FontAwesomeIcon icon={faUserShield}/>} key="manage-users">Manage Users</DropdownItem>
                    <DropdownItem href={"/app/users/groups/"} endContent={<FontAwesomeIcon icon={faUsersGear}/>} key="manage-groups">Manage Groups</DropdownItem>
                    <DropdownItem href={"/app/settings/"} endContent={<FontAwesomeIcon icon={faSliders}/>} key="settings">Settings</DropdownItem>
                </DropdownSection>
                <DropdownSection key="external-links">
                    <DropdownItem
                        as={Link}
                        target={"_blank"}
                        className={"text-inherit"}
                        endContent={<FontAwesomeIcon icon={faExternalLink} fontSize={12}/>}
                        href={"https://github.com/Drew-Chase/obsidian-server-panel"}
                        key="source-code"
                    >
                        Source Code
                    </DropdownItem>
                    <DropdownItem
                        as={Link}
                        target={"_blank"}
                        className={"text-inherit"}
                        endContent={<FontAwesomeIcon icon={faExternalLink} fontSize={12}/>}
                        href={"https://github.com/Drew-Chase/obsidian-server-panel/issues"}
                        key="help"
                    >
                        Help
                    </DropdownItem>
                    <DropdownItem
                        as={Link}
                        target={"_blank"}
                        className={"text-inherit"}
                        endContent={<FontAwesomeIcon icon={faExternalLink} fontSize={12}/>}
                        href={"https://github.com/Drew-Chase/obsidian-server-panel/issues/new"}
                        key="feedback"
                    >
                        Feedback
                    </DropdownItem>
                </DropdownSection>
                <DropdownSection title={"Danger Zone"} className={"text-danger"} key="danger-zone">
                    <DropdownItem
                        endContent={<FontAwesomeIcon icon={faSignOut}/>}
                        color={"danger"}
                        onClick={() =>
                        {
                            auth.logout();
                            setIsLoggedIn(false);
                            navigate("/");
                        }}
                        key="logout"
                    >
                        Logout
                    </DropdownItem>
                </DropdownSection>
            </DropdownMenu>
        </ODropdown>

    );
}