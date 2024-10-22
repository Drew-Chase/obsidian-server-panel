import {SelectItem} from "@nextui-org/react";
import OSelect from "../../Extends/OSelect.tsx";

interface ServerSettingsProps
{
    serverDifficulty: string;
    setServerDifficulty: (difficulty: string) => void;
    serverGamemode: string;
    setServerGamemode: (gamemode: string) => void;
    hardcoreMode: boolean;
}

export default function ServerSettings({serverDifficulty, setServerDifficulty, serverGamemode, setServerGamemode, hardcoreMode}: ServerSettingsProps)
{
    return (
        <>
            <OSelect
                label="Difficulty"
                placeholder="Select a difficulty"
                defaultSelectedKeys={["easy"]}
                selectedKeys={[serverDifficulty]}
                onSelectionChange={(e) => setServerDifficulty([...e][0] as string)}
                isRequired
                isDisabled={hardcoreMode}
            >
                <SelectItem key={"peaceful"}>Peaceful</SelectItem>
                <SelectItem key={"easy"}>Easy</SelectItem>
                <SelectItem key={"normal"}>Normal</SelectItem>
                <SelectItem key={"hard"}>Hard</SelectItem>
            </OSelect>
            <OSelect
                label="Gamemode"
                placeholder="Select a gamemode"
                defaultSelectedKeys={["survival"]}
                selectedKeys={[serverGamemode]}
                isDisabled={hardcoreMode}
                onSelectionChange={(e) => setServerGamemode([...e][0] as string)}
                isRequired
            >
                <SelectItem key={"survival"}>Survival</SelectItem>
                <SelectItem key={"creative"}>Creative</SelectItem>
                <SelectItem key={"adventure"}>Adventure</SelectItem>
            </OSelect>
        </>
    );
}