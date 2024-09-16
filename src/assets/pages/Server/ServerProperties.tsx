import {setTitle} from "../../../main.tsx";
import {Input, ScrollShadow} from "@nextui-org/react";
import ExtendedSwitch from "../../components/Extends/ExtendedSwitch.tsx";
import {useState} from "react";
import MagnifyGlass from "../../images/MagnifyGlass.svg.tsx";

interface ServerPropertiesItem
{
    name: string;
    value: string;
    type: "string" | "number" | "boolean";
}

export default function ServerProperties()
{
    setTitle("Server Properties");
    const properties: ServerPropertiesItem[] = getServerProperties();
    const [search, setSearch] = useState("");

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Properties</p>
            </div>
            <Input
                label={"Search"}
                placeholder={"Search for..."}
                value={search}
                startContent={<MagnifyGlass/>}
                onValueChange={setSearch}
                autoFocus
                variant={"underlined"}
            />
            <ScrollShadow className={"flex flex-col max-h-[calc(100dvh_-_250px)] gap-4 overflow-y-auto pr-4 shrink-0"}>
                {properties
                    .filter(({name}) => name.replace(/[-._]/g, " ").toLowerCase().includes(search.toLowerCase()))
                    .sort((a, b) => a.name.localeCompare(b.name))
                    .map(({name, value, type}) => (
                        type === "string" || type === "number" ? (
                            <Input
                                label={name}
                                value={value}
                                className={"w-full drop-shadow-lg shrink-0"}
                                classNames={{
                                    inputWrapper: "bg-neutral-700"
                                }}
                            />
                        ) : (
                            <ExtendedSwitch
                                label={name}
                                toggle={value === "true"}
                                className={"max-w-full shrink-0"}
                            />
                        )
                    ))}
            </ScrollShadow>
        </div>
    );
}

function getServerProperties(): ServerPropertiesItem[]
{
    return [
        {name: "level-name", value: "world", type: "string"},
        {name: "allow-flight", value: "false", type: "boolean"},
        {name: "server-port", value: "25565", type: "number"},
        {name: "accepts-transfers", value: "false", type: "boolean"},
        {name: "allow-nether", value: "true", type: "boolean"},
        {name: "broadcast-console-to-ops", value: "true", type: "boolean"},
        {name: "broadcast-rcon-to-ops", value: "true", type: "boolean"},
        {name: "bug-report-link", value: "", type: "string"},
        {name: "debug", value: "false", type: "boolean"},
        {name: "difficulty", value: "easy", type: "string"},
        {name: "enable-command-block", value: "false", type: "boolean"},
        {name: "enable-jmx-monitoring", value: "false", type: "boolean"},
        {name: "enable-query", value: "false", type: "boolean"},
        {name: "enable-rcon", value: "false", type: "boolean"},
        {name: "enable-status", value: "true", type: "boolean"},
        {name: "enforce-secure-profile", value: "true", type: "boolean"},
        {name: "enforce-whitelist", value: "false", type: "boolean"},
        {name: "entity-broadcast-range-percentage", value: "100", type: "number"},
        {name: "force-gamemode", value: "false", type: "boolean"},
        {name: "function-permission-level", value: "2", type: "number"},
        {name: "gamemode", value: "survival", type: "string"},
        {name: "generate-structures", value: "true", type: "boolean"},
        {name: "generator-settings", value: "{}", type: "string"},
        {name: "hardcore", value: "false", type: "boolean"},
        {name: "hide-online-players", value: "false", type: "boolean"},
        {name: "initial-disabled-packs", value: "", type: "string"},
        {name: "initial-enabled-packs", value: "vanilla", type: "string"},
        {name: "level-seed", value: "", type: "string"},
        {name: "level-type", value: "minecraft\\:normal", type: "string"},
        {name: "log-ips", value: "true", type: "boolean"},
        {name: "max-chained-neighbor-updates", value: "1000000", type: "number"},
        {name: "max-players", value: "20", type: "number"},
        {name: "max-tick-time", value: "60000", type: "number"},
        {name: "max-world-size", value: "29999984", type: "number"},
        {name: "motd", value: "A Minecraft Server", type: "string"},
        {name: "network-compression-threshold", value: "256", type: "number"},
        {name: "online-mode", value: "true", type: "boolean"},
        {name: "op-permission-level", value: "4", type: "number"},
        {name: "player-idle-timeout", value: "0", type: "number"},
        {name: "prevent-proxy-connections", value: "false", type: "boolean"},
        {name: "pvp", value: "true", type: "boolean"},
        {name: "query.port", value: "25565", type: "number"},
        {name: "rate-limit", value: "0", type: "number"},
        {name: "rcon.password", value: "", type: "string"},
        {name: "rcon.port", value: "25575", type: "number"},
        {name: "region-file-compression", value: "deflate", type: "string"},
        {name: "require-resource-pack", value: "false", type: "boolean"},
        {name: "resource-pack", value: "", type: "string"},
        {name: "resource-pack-id", value: "", type: "string"},
        {name: "resource-pack-prompt", value: "", type: "string"},
        {name: "resource-pack-sha1", value: "", type: "string"},
        {name: "server-ip", value: "", type: "string"},
        {name: "simulation-distance", value: "10", type: "number"},
        {name: "spawn-animals", value: "true", type: "boolean"},
        {name: "spawn-monsters", value: "true", type: "boolean"},
        {name: "spawn-npcs", value: "true", type: "boolean"},
        {name: "spawn-protection", value: "16", type: "number"},
        {name: "sync-chunk-writes", value: "true", type: "boolean"},
        {name: "text-filtering-config", value: "", type: "string"},
        {name: "use-native-transport", value: "true", type: "boolean"},
        {name: "view-distance", value: "10", type: "number"},
        {name: "white-list", value: "false", type: "boolean"}
    ];
}