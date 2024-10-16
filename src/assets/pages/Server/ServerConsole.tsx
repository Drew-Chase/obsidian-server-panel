import {Autocomplete, AutocompleteItem, Button, ScrollShadow} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPaperPlane, faTerminal} from "@fortawesome/free-solid-svg-icons";
// @ts-ignore
import {Prism as SyntaxHighlighter} from "react-syntax-highlighter";
// @ts-ignore
import {duotoneDark} from "react-syntax-highlighter/dist/esm/styles/prism";

import "../../scss/logpage.scss";
import {useEffect} from "react";

import $ from "jquery";
import {setTitle} from "../../../main.tsx";

export default function ServerConsole()
{
    setTitle("Server Console");
    let scrollLock: boolean = true;

    useEffect(() =>
    {
        const view = $("#log-view");
        view.on("scroll", () =>
        {
            scrollLock = (view.scrollTop() ?? 0) + (view.innerHeight() ?? 0) >= view[0].scrollHeight;
            console.log("Scroll Lock: ", scrollLock);
        });
        const scrollInterval = setInterval(() =>
        {
            if (scrollLock)
            {
                view.scrollTop(view[0].scrollHeight + 10);
                console.log("scrolling");
            }
        }, 500);

        return () =>
        {
            clearInterval(scrollInterval);
            view.off("scroll");
        };

    }, []);

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Console</p>
            </div>
            <ScrollShadow id={"log-view"} className={"max-h-[calc(100dvh_-_250px)] h-screen overflow-y-auto bg-neutral-800 rounded-2xl p-4"}>
                <SyntaxHighlighter language={"log"} style={duotoneDark} wrapLongLines wrapLines>
                    {log}
                </SyntaxHighlighter>
            </ScrollShadow>
            <Autocomplete
                label={"Command"}
                placeholder={"Enter command..."}
                startContent={<FontAwesomeIcon icon={faTerminal}/>}
                endContent={<Button variant={"light"}><FontAwesomeIcon icon={faPaperPlane}/></Button>}
                className={"w-full drop-shadow-lg shrink-0 pr-0"}
                inputProps={{
                    classNames: {
                        inputWrapper: "bg-neutral-700"
                    }
                }}
            >
                {Array.from({length: 20}, (_, i) => (<AutocompleteItem key={`say-${i}`} value={"say"} textValue={`say ${i}`}>say {i}</AutocompleteItem>))}
            </Autocomplete>
        </div>
    );
}


const log: string = `[00:00:00] [ftbbackups2_Worker-1/INFO]: Attempting to create an automatic backup
[00:00:00] [ftbbackups2_Worker-1/INFO]: Found world folder at /var/games/minecraft/servers/smp/./world/.
[00:00:00] [ftbbackups2_Worker-1/INFO]: Last backup size: 1.2GB Current world size: 1.7GB Current Available space: 12.1TB ExpectedSize 1.3GB
[00:00:00] [ftbbackups2_Worker-1/INFO]: Setting world minecraft:overworld save state to true
[00:00:00] [ftbbackups2_Worker-1/INFO]: Setting world minecraft:the_end save state to true
[00:00:00] [ftbbackups2_Worker-1/INFO]: Setting world minecraft:the_nether save state to true
[00:00:00] [ftbbackups2_Worker-1/INFO]: Writing to file /var/games/minecraft/servers/smp/backups/backups.json
[00:00:00] [FTB Backups backup thread 0/INFO]: Waiting for world save to complete.
[00:00:00] [FTB Backups backup thread 0/INFO]: Skipping backup preview because preview is disabled.
[00:00:43] [FTB Backups backup thread 0/INFO]: Setting world minecraft:overworld save state to false
[00:00:43] [FTB Backups backup thread 0/INFO]: Setting world minecraft:the_end save state to false
[00:00:43] [FTB Backups backup thread 0/INFO]: Setting world minecraft:the_nether save state to false
[00:00:44] [FTB Backups backup thread 0/INFO]: Backup size 1.2GB World Size 1.7GB
[00:00:44] [FTB Backups backup thread 0/INFO]: Writing to file /var/games/minecraft/servers/smp/backups/backups.json
[00:00:44] [FTB Backups backup thread 0/INFO]: New backup created at /var/games/minecraft/servers/smp/backups/2024-9-14_0-0-0.zip size: 1.2GB Took: 0m, 43s, 88ms Sha1: 13089873cbbf0e15fdb3f1a3dd273fc66f435d64
[03:00:43] [Server thread/INFO]: /advancement (grant|revoke)
[03:00:43] [Server thread/INFO]: /attribute <target> <attribute> (get|base|modifier)
[03:00:43] [Server thread/INFO]: /execute (run|if|unless|as|at|store|positioned|rotated|facing|align|anchored|in|summon|on)
[03:00:43] [Server thread/INFO]: /bossbar (add|remove|list|set|get)
[03:00:43] [Server thread/INFO]: /clear [<targets>]
[03:00:43] [Server thread/INFO]: /clone (<begin>|from)
[03:00:43] [Server thread/INFO]: /damage <target> <amount> [<damageType>]
[03:00:43] [Server thread/INFO]: /data (merge|get|remove|modify)
[03:00:43] [Server thread/INFO]: /datapack (enable|disable|list)
[03:00:43] [Server thread/INFO]: /debug (start|stop|function)
[03:00:43] [Server thread/INFO]: /defaultgamemode <gamemode>
[03:00:43] [Server thread/INFO]: /difficulty [peaceful|easy|normal|hard]
[03:00:43] [Server thread/INFO]: /effect (clear|give)
[03:00:43] [Server thread/INFO]: /me <action>
[03:00:43] [Server thread/INFO]: /enchant <targets> <enchantment> [<level>]
[03:00:43] [Server thread/INFO]: /experience (add|set|query)
[03:00:43] [Server thread/INFO]: /xp -> experience
[03:00:43] [Server thread/INFO]: /fill <from> <to> <block> [replace|keep|outline|hollow|destroy]
[03:00:43] [Server thread/INFO]: /fillbiome <from> <to> <biome> [replace]
[03:00:43] [Server thread/INFO]: /forceload (add|remove|query)
[03:00:43] [Server thread/INFO]: /function <name> [<arguments>|with]
[03:00:43] [Server thread/INFO]: /gamemode <gamemode> [<target>]
[03:00:43] [Server thread/INFO]: /gamerule (announceAdvancements|blockExplosionDropDecay|commandBlockOutput|commandModificationBlockLimit|disableElytraMovementCheck|disableRaids|doDaylightCycle|doEntityDrops|doFireTick|doImmediateRespawn|doInsomnia|doLimitedCrafting|doMobLoot|doMobSpawning|doPatrolSpawning|doTileDrops|doTraderSpawning|doVinesSpread|doWardenSpawning|doWeatherCycle|drowningDamage|enderPearlsVanishOnDeath|fallDamage|fireDamage|forgiveDeadPlayers|freezeDamage|globalSoundEvents|keepInventory|lavaSourceConversion|logAdminCommands|maxCommandChainLength|maxCommandForkCount|maxEntityCramming|mobExplosionDropDecay|mobGriefing|naturalRegeneration|playersNetherPortalCreativeDelay|playersNetherPortalDefaultDelay|playersSleepingPercentage|projectilesCanBreakBlocks|randomTickSpeed|reducedDebugInfo|sendCommandFeedback|showDeathMessages|snowAccumulationHeight|spawnRadius|spectatorsGenerateChunks|tntExplosionDropDecay|universalAnger|waterSourceConversion)
[03:00:43] [Server thread/INFO]: /give <targets> <item> [<count>]
[03:00:43] [Server thread/INFO]: /help [<command>]
[03:00:43] [Server thread/INFO]: /item (replace|modify)
[03:00:43] [Server thread/INFO]: /kick <targets> [<reason>]
[03:00:43] [Server thread/INFO]: /kill [<targets>]
[03:00:43] [Server thread/INFO]: /list [uuids]
[03:00:43] [Server thread/INFO]: /locate (structure|biome|poi)
[03:00:43] [Server thread/INFO]: /loot (replace|insert|give|spawn)
[03:00:43] [Server thread/INFO]: /msg <targets> <message>
[03:00:43] [Server thread/INFO]: /tell -> msg
[03:00:43] [Server thread/INFO]: /w -> msg
[03:00:43] [Server thread/INFO]: /particle <name> [<pos>]
[03:00:43] [Server thread/INFO]: /place (feature|jigsaw|structure|template)
[03:00:43] [Server thread/INFO]: /playsound <sound> (master|music|record|weather|block|hostile|neutral|player|ambient|voice)
[03:00:43] [Server thread/INFO]: /random (value|roll|reset)
[03:00:43] [Server thread/INFO]: /reload
[03:00:43] [Server thread/INFO]: /recipe (give|take)
[03:00:43] [Server thread/INFO]: /return (<value>|fail|run)
[03:00:43] [Server thread/INFO]: /ride <target> (mount|dismount)
[03:00:43] [Server thread/INFO]: /say <message>
[03:00:43] [Server thread/INFO]: /schedule (function|clear)
[03:00:43] [Server thread/INFO]: /scoreboard (objectives|players)
[03:00:43] [Server thread/INFO]: /seed
[03:00:43] [Server thread/INFO]: /setblock <pos> <block> [destroy|keep|replace]
[03:00:43] [Server thread/INFO]: /spawnpoint [<targets>]
[03:00:43] [Server thread/INFO]: /setworldspawn [<pos>]
[03:00:43] [Server thread/INFO]: /spectate [<target>]`;