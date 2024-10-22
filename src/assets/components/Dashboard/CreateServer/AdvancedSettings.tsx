import {Slider} from "@nextui-org/react";
import ExtendedSwitch from "../../Extends/ExtendedSwitch.tsx";
import {useState} from "react";
import OInput from "../../Extends/OInput.tsx";

interface AdvancedSettingsProps
{
    hardcoreMode: boolean;
    setHardcoreMode: (value: boolean) => void;
    serverMaxPlayers: number;
    setServerMaxPlayers: (value: number) => void;
}

export default function AdvancedSettings(props: AdvancedSettingsProps)
{
    const [serverMaxPlayersState, setServerMaxPlayersState] = useState<string>(props.serverMaxPlayers.toString());

    const handleServerMaxPlayersChange = (value: string) =>
    {
        setServerMaxPlayersState(value);
        props.setServerMaxPlayers(+value);
    };

    return (
        <>
            <ExtendedSwitch
                label={"Enable Hardcore Mode"}
                description={"Enable hardcore mode for the server"}
                toggle={props.hardcoreMode}
                onToggle={props.setHardcoreMode}
                classNames={{base: "max-w-full"}}
            />
            <Slider
                minValue={1}
                maxValue={100}
                step={1}
                value={Number.isNaN(parseInt(serverMaxPlayersState)) ? 1 : parseInt(serverMaxPlayersState)}
                onChange={(e) =>
                {
                    handleServerMaxPlayersChange(e.toString());
                }}
                label={"Max Players"}
                showTooltip
                showSteps
                renderValue={() =>
                {
                    return <OInput
                        value={serverMaxPlayersState.toString()}
                        onValueChange={(e) =>
                        {
                            const value = e.replace(/\D/g, "");
                            handleServerMaxPlayersChange(value);
                        }}
                        onFocusChange={(isFocused) =>
                        {
                            if (!isFocused)
                            {
                                if (serverMaxPlayersState === "" || parseInt(serverMaxPlayersState) < 1)
                                {
                                    const value = "20";
                                    handleServerMaxPlayersChange(value);
                                }
                            }
                        }}
                        className={"w-16"}
                    />;
                }}
            />
        </>
    );
}