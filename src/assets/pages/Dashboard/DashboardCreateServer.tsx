import {Autocomplete, AutocompleteItem, Button, Divider, Input, Select, SelectItem, Slider, Tab, Tabs, Tooltip} from "@nextui-org/react";
import {useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUpload} from "@fortawesome/free-solid-svg-icons";
import ExtendedSwitch from "../../components/Extends/ExtendedSwitch.tsx";

export default function DashboardCreateServer()
{
    const [serverName, setServerName] = useState<string>("");
    const [serverPort, setServerPort] = useState<string>("25565");
    const [serverDifficulty, setServerDifficulty] = useState<string>("easy");
    const [serverGamemode, setServerGamemode] = useState<string>("survival");
    const [serverMaxPlayers, setServerMaxPlayers] = useState<string>("20");
    const [hardcoreMode, setHardcoreMode] = useState<boolean>(false);

    const [portError, setPortError] = useState<string | null>(null);


    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 max-h-[calc(100dvh_-_60px)] h-dvh overflow-y-auto gap-4"}>
            <p className={"text-lg font-semibold"}>Create Server</p>

            <div className={"outline-2 outline-dotted outline-primary w-full h-[200px] rounded-2xl flex flex-row p-4 gap-8 items-center justify-center shadow-inner"}>
                <p className={"text-4xl font-bold text-center p-4"}>
                    Drag<br/>&amp;<br/>Drop
                </p>
                <Divider orientation={"vertical"} className={"mx-[100px]"}/>
                <div className={"flex flex-col"}>
                    <Button size={"lg"} variant={"ghost"} color={"primary"} className={"p-8"} startContent={<FontAwesomeIcon icon={faUpload}/>}>Select Icon</Button>
                </div>
            </div>

            <Input
                label="Server Name"
                placeholder="Enter a name for your server"
                classNames={{inputWrapper: "bg-neutral-700"}}
                value={serverName}
                onValueChange={setServerName}
                isRequired
            />

            <Input
                label="Server Port"
                placeholder="Enter a port for your server"
                classNames={{inputWrapper: "bg-neutral-700"}}
                value={serverPort}
                onValueChange={e =>
                {
                    const portString = e.replace(/\D/g, "");
                    const minPort = 1;
                    const maxPort = 65535;
                    setServerPort(portString);
                    if (portString === "" || (parseInt(portString) >= minPort && parseInt(portString) <= maxPort))
                    {
                        setPortError(null);
                    } else
                    {
                        setPortError("Invalid port number, please enter a valid port number between 1 and 65535");
                    }
                }}
                errorMessage={portError}
                isInvalid={portError !== null}
                isRequired
            />

            <Select
                label="Difficulty"
                placeholder="Select a difficulty"
                defaultSelectedKeys={["easy"]}
                classNames={{trigger: "bg-neutral-700"}}
                selectedKeys={[serverDifficulty]}
                onSelectionChange={(e) => setServerDifficulty([...e][0] as string)}
                isRequired
                isDisabled={hardcoreMode}
            >
                <SelectItem key={"peaceful"}>Peaceful</SelectItem>
                <SelectItem key={"easy"}>Easy</SelectItem>
                <SelectItem key={"normal"}>Normal</SelectItem>
                <SelectItem key={"hard"}>Hard</SelectItem>
            </Select>

            <Select
                label="Gamemode"
                placeholder="Select a gamemode"
                defaultSelectedKeys={["survival"]}
                classNames={{trigger: "bg-neutral-700"}}
                selectedKeys={[serverGamemode]}
                isDisabled={hardcoreMode}
                onSelectionChange={(e) => setServerGamemode([...e][0] as string)}
                isRequired
            >
                <SelectItem key={"survival"}>Survival</SelectItem>
                <SelectItem key={"creative"}>Creative</SelectItem>
                <SelectItem key={"adventure"}>Adventure</SelectItem>
            </Select>

            <ExtendedSwitch
                label={"Enable Hardcore Mode"}
                description={"Enable hardcore mode for the server"}
                toggle={hardcoreMode}
                onToggle={value =>
                {
                    if (value)
                    {
                        setServerGamemode("survival");
                        setServerDifficulty("hard");
                    }
                    setHardcoreMode(value);
                }}
            />

            <Slider
                minValue={1}
                maxValue={100}
                step={1}
                value={Number.isNaN(parseInt(serverMaxPlayers)) ? 1 : parseInt(serverMaxPlayers)}
                onChange={(e) => setServerMaxPlayers(e.toString())}
                label={"Max Players"}
                showTooltip
                showSteps
                renderValue={() =>
                {
                    return <Input
                        value={serverMaxPlayers.toString()}
                        onValueChange={(e) => setServerMaxPlayers(e.replace(/\D/g, ""))}
                        onFocusChange={(isFocused) =>
                        {
                            if (!isFocused)
                            {
                                if (serverMaxPlayers === "" || parseInt(serverMaxPlayers) < 1)
                                {
                                    setServerMaxPlayers("20");
                                }
                            }
                        }}
                        classNames={{inputWrapper: "bg-neutral-700"}}
                        className={"w-16"}
                    />;
                }}
            />

            <Autocomplete
                label={"Minecraft Version"}
                placeholder={"Select a Minecraft version"}
                inputProps={{classNames: {inputWrapper: "bg-neutral-700"}}}
            >
                <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
            </Autocomplete>
            <ExtendedSwitch
                label={"Show Snapshots"}
                description={"Show snapshots in the version list"}
                className={"max-w-full"}
            />
            <p>Select Loader</p>
            <Tabs>
                <Tab title={"Vanilla"}></Tab>
                <Tab title={"Fabric"}>
                    <div className={"flex flex-row w-full gap-4 items-center"}>
                        <Autocomplete
                            label={"Fabric Loader Version"}
                            placeholder={"Select a Fabric Loader version"}
                            inputProps={{classNames: {inputWrapper: "bg-neutral-700"}}}
                            className={"w-full"}
                        >
                            <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                        </Autocomplete>
                        <Tooltip content={"This is required for most fabric mods"}>
                            <Button>Install Fabric API</Button>
                        </Tooltip>
                    </div>
                </Tab>
                <Tab title={"Forge"}>
                    <Autocomplete
                        label={"Forge Version"}
                        placeholder={"Select a Forge version"}
                        inputProps={{classNames: {inputWrapper: "bg-neutral-700"}}}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </Autocomplete>
                </Tab>
                <Tab title={"NeoForge"}>
                    <Autocomplete
                        label={"NeoForge Version"}
                        placeholder={"Select a NeoForge version"}
                        inputProps={{classNames: {inputWrapper: "bg-neutral-700"}}}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </Autocomplete>
                </Tab>
                <Tab title={"Quilt"}>
                    <Autocomplete
                        label={"Quilt Version"}
                        placeholder={"Select a Quilt version"}
                        inputProps={{classNames: {inputWrapper: "bg-neutral-700"}}}
                        className={"w-full"}
                    >
                        <AutocompleteItem key={"1.20.4"}>1.20.4</AutocompleteItem>
                    </Autocomplete>
                </Tab>
            </Tabs>

            <Button color={"primary"} className={"mt-4 shrink-0"}>Create Server</Button>

        </div>
    );
}
